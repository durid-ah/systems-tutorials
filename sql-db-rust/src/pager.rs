use core::mem::{self, MaybeUninit};
use std::fs::{File, OpenOptions};
use std::io::{Seek, SeekFrom, Read};

use super::size_constants::{
   ROWS_PER_PAGE,
   TABLE_MAX_PAGES,
   PAGE_SIZE,
   ROW_SIZE
};

type Page = [Option<Vec<u8>>; ROWS_PER_PAGE];
type UninitPage = [MaybeUninit<Option<Vec<u8>>>; ROWS_PER_PAGE];

pub struct Pager {
   file: File,
   file_length: u64,
   pages: [Option<Page>; TABLE_MAX_PAGES]
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
      
      return Pager {file: _file, file_length: _size, pages: _pages}
   }

   pub fn close_pager() {

   }

   fn flush_page() {

   }


   pub fn get_row(&mut self, row_num: usize)-> &mut Option<Vec<u8>> {
      let page_num: usize = self.get_page_idx(row_num);
      let row_idx: usize = self.get_row_idx(row_num);
   
      let page = &mut self.pages[page_num];

      if let Option::None =  page {
         Pager::init_page_rows(page);
         Pager::load_page_from_file(page, self.file_length, &mut self.file, page_num);
      }

      let res = page.as_mut().unwrap();
      &mut res[row_idx]
   }

   /// Initialize each page to Option::None
   fn init_pages() -> [Option<Page>; TABLE_MAX_PAGES] {
      return {
         let mut _pages: [MaybeUninit<Option<Page>>; TABLE_MAX_PAGES] = unsafe {
            // the compiler assumes the array is initialized when it isn't
            MaybeUninit::uninit().assume_init()
         };

         // set each page to Option::None
         for elem in &mut _pages[..] {
            *elem = MaybeUninit::new(Option::None);
         }

         //remove the MaybeUninit part of the type to make it a an option array
         unsafe { mem::transmute::<_, [Option<Page>; TABLE_MAX_PAGES]>(_pages)}
      };
   }

   /// Return the index of the page where the row number resides
   fn get_page_idx(&self, row_num: usize) -> usize { return row_num / ROWS_PER_PAGE}
   
   /// Get the row within the page where the row resides
   fn get_row_idx(&self, row_num: usize) -> usize { return row_num % ROWS_PER_PAGE }
   
   /// Init the rows of the page to Option::None
   fn init_page_rows( page: &mut Option<Page>) {
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

   fn map_bytes_to_rows(page: &mut Option<Page>, buffer: &[u8]) {
      let rows = page.as_mut().unwrap();
      let mut begin = 0;
      let mut end = ROW_SIZE;

      
      for i in 0 .. rows.len() {
         let mut data: Vec<u8> = Vec::with_capacity(ROW_SIZE);
         for j in begin .. end {
            data.push(buffer[j])
         }

         println!("BEGIN INDEX: {:?}", begin);
         println!("END INDEX: {:?}", end);
         println!("DATA ROW: {:?}", data);

         rows[i] = Option::Some(data);
         begin = begin + ROW_SIZE;
         end = end + ROW_SIZE;
      }
   }

   fn load_page_from_file(
      page: &mut Option<Page>, file_length: u64, file: &mut File, page_num: usize
   ) {
      // number of pages in the file
      let mut page_count = file_length / PAGE_SIZE;
      
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
         let _ = file.read(&mut read_buf[..]);
         Pager::map_bytes_to_rows(page, &read_buf);
      }
   }
}