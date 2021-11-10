use core::mem::{self, MaybeUninit};
use std::fs::{File, OpenOptions};
use std::io::{Seek, SeekFrom, Read, Write, IoSlice};
use std::vec::Vec;
use super::file_manager::FileManager;

use super::size_constants::{
   ROWS_PER_PAGE,
   TABLE_MAX_PAGES,
   PAGE_SIZE,
   ROW_SIZE
};

// INFO: it seems like the files are stored without padding at the end
// TODO: Check the serialization/deserialization
type Page = [Option<Vec<u8>>; ROWS_PER_PAGE as usize];
type UninitPage = [MaybeUninit<Option<Vec<u8>>>; ROWS_PER_PAGE as usize];

pub struct Pager {
   file: File,
   file_mgr: FileManager,
   pub file_length: u64,
   pages: [Option<Page>; TABLE_MAX_PAGES as usize]
}

impl Pager {
   pub fn open_pager(file_name: String) -> Pager {
      let mut _file = OpenOptions::new()
         .read(true)
         .write(true)
         .create(true)
         .open(file_name)
         .unwrap();

      let _size = _file.seek(SeekFrom::End(0)).unwrap();
      let _pages = Pager::init_pages();
      let file_mgr = FileManager::new(file_name);
      
      return Pager {file: _file, file_length: _size, pages: _pages, file_mgr}
   }

   pub fn close_pager(&mut self, num_rows: u64) {
      let full_page_count = num_rows / ROWS_PER_PAGE;
      let leftover_rows = num_rows % ROWS_PER_PAGE;

      for i in 0..(full_page_count as usize) {
         let page = &self.pages[i];
         Pager::flush_page(page, i as u64, &mut self.file)
      }

      if leftover_rows != 0 {
         let page = &self.pages[full_page_count as usize];
         Pager::flush_page(page, full_page_count, &mut self.file);
      }

      self.file.sync_all().expect("Unable to finish closing the database");
   }

   fn flush_page(page: &Option<Page>, page_num: u64, file: &mut File) {
      println!("flush_page()");
      let page_to_write = page.as_ref().expect("Attempting To Flush None Page");
      let offset = Pager::get_page_file_offset(page_num as u64);
      
      let _ = file.seek(offset);

      for i in 0..(ROWS_PER_PAGE as usize) {
         if page_to_write[i].is_none() {
            println!("Skipping row: {:?}", i);
            continue;
         }

         let unwraped_row = page_to_write[i].as_ref().unwrap();

         println!("Writting row: {:?}", i);
         println!("Writting row data: \n {:?}", unwraped_row);

         let row_slice = IoSlice::new(unwraped_row.as_slice());
         file.write(&row_slice).expect("unable to write to file");
      }
   }

   pub fn get_row(&mut self, row_num: u64)-> &mut Option<Vec<u8>> {
      println!("get_row()");
      let page_num: usize = self.get_page_idx(row_num) as usize;
      let row_idx: usize = self.get_row_idx(row_num) as usize;

      println!("Getting Page #: {:?}", page_num);
      println!("Getting Row IDX: {:?}", row_idx);
   
      let page = &mut self.pages[page_num];

      if let Option::None =  page {
         Pager::init_page_rows(page);
      }

      let res = page.as_mut().unwrap();
      &mut res[row_idx]
   }

   /// Initialize each page to Option::None
   fn init_pages() -> [Option<Page>; TABLE_MAX_PAGES as usize] {
      return {
         let mut _pages: [MaybeUninit<Option<Page>>; TABLE_MAX_PAGES as usize] = unsafe {
            // the compiler assumes the array is initialized when it isn't
            MaybeUninit::uninit().assume_init()
         };

         // set each page to Option::None
         for elem in &mut _pages[..] {
            *elem = MaybeUninit::new(Option::None);
         } 

         //remove the MaybeUninit part of the type to make it a an option array
         unsafe { mem::transmute::<_, [Option<Page>; TABLE_MAX_PAGES as usize]>(_pages)}
      };
   }

   /// Return the index of the page where the row number resides
   fn get_page_idx(&self, row_num: u64) -> u64 { return row_num / ROWS_PER_PAGE}
   
   /// Get the row within the page where the row resides
   fn get_row_idx(&self, row_num: u64) -> u64 { return row_num % ROWS_PER_PAGE }

   /// Get a page's offset in the file
   fn get_page_file_offset(page_num: u64) -> SeekFrom { SeekFrom::Start(page_num * PAGE_SIZE) }
   
   /// Init the rows of the page to Option::None
   fn init_page_rows(page: &mut Option<Page>) {
      let mut _page: Page = {
         let mut _init_page : UninitPage = unsafe {
            MaybeUninit::uninit().assume_init()
         };

         for r in &mut _init_page[..] {
            *r = MaybeUninit::new(Option::None);
         }
         
         unsafe {mem::transmute(_init_page)}
      };

      *page = Option::Some(_page);
   }

   fn map_bytes_to_rows(page: &mut Option<Page>, buffer: &[u8], bytes_read: usize) {
      let rows = page.as_mut().unwrap();
      let mut begin = 0;
      let mut end = ROW_SIZE;
      let mut end_mapping = false;

      println!("BUFFER_SIZE: {:?}", buffer.len());

      for i in 0 .. rows.len() {
         let mut data: Vec<u8> = Vec::with_capacity(ROW_SIZE as usize);
         for j in begin .. end {
            if j as usize  >= bytes_read {
               end_mapping = true;
               break;
            }
            data.push(buffer[j as usize]);
         }

         println!("DATA: {:?}", data);
         println!("BEGIN INDEX: {:?}", begin);
         println!("END INDEX: {:?}", end);

         if data.len() == 0 {
            break;
         }

         rows[i] = Option::Some(data);
         begin = begin + ROW_SIZE;
         end = end + ROW_SIZE;

         if end_mapping {
            break;
         }
      }
   }

   fn load_page_from_file(
      page: &mut Option<Page>, file_length: u64, file: &mut File, page_num: usize
   ) {
      // number of pages in the file
      let mut page_count = file_length / PAGE_SIZE;

      println!("LOADING PAGE: {:?}", page_num);
      
      // add a page for the leftover rows at the end
      if file_length % PAGE_SIZE != 0 {
         page_count += 1
      }

      if page_num <= (page_count as usize) {
         let offset_bytes = (page_num * (PAGE_SIZE as usize)) as u64;
         let mut read_buf: [u8; (PAGE_SIZE as usize)] = [0; (PAGE_SIZE as usize)]; 
         let offset = SeekFrom::Start(offset_bytes);
         
         // TODO: handle errors
         let _ = file.seek(offset);
         let bytes_read = file.read(&mut read_buf[..]);
         println!("Amount Read: {:?}", bytes_read);
         Pager::map_bytes_to_rows(page, &read_buf, bytes_read.unwrap());
      }
   }
}