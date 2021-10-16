use core::mem::{self, MaybeUninit};
use std::fs::{File, OpenOptions};
use std::io::{Seek, SeekFrom};

use super::size_constants::{
   ROWS_PER_PAGE,
   TABLE_MAX_PAGES,
};

type Page = [Option<Vec<u8>>; ROWS_PER_PAGE];
type UninitPage = [MaybeUninit<Option<Vec<u8>>>; ROWS_PER_PAGE];

pub struct Pager {
   pub file: File,
   pub file_length: u64,
   pub pages: [Option<Page>; TABLE_MAX_PAGES]
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
   
   pub fn get_row(&mut self, row_num: usize)-> &mut Option<Vec<u8>> {
      let page_num: usize = self.get_page_idx(row_num);
      let row_idx: usize = self.get_row_idx(row_num);
   
      let page = &mut self.pages[(page_num as usize)];
      if let Option::None =  page {
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

      let res = self.pages[(page_num as usize)].as_mut().unwrap();
      &mut res[row_idx]
   }

   /// Init the rows of the page to Option::None
   pub fn init_page_rows(&mut self, page: &mut Option<Page>) {
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
}