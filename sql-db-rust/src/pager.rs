use core::mem::{self, MaybeUninit};
use std::fs::{File, OpenOptions};
use std::io::{Seek, SeekFrom};

use super::size_constants::{
   ROWS_PER_PAGE,
   TABLE_MAX_PAGES,
};

pub struct Pager {
   pub file: File,
   pub file_length: u64,
   pub pages: [Option<[Option<Vec<u8>>; ROWS_PER_PAGE]>; TABLE_MAX_PAGES]
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

   fn init_pages() -> [Option<[Option<Vec<u8>>; ROWS_PER_PAGE]>; TABLE_MAX_PAGES] {
      return {
         let mut _pages: [MaybeUninit<Option<[Option<Vec<u8>>; ROWS_PER_PAGE]>>; TABLE_MAX_PAGES] = unsafe {
            // the compiler assumes the array is initialized when it isn't
            MaybeUninit::uninit().assume_init()
         };

         // set each page to Option::None
         for elem in &mut _pages[..] {
            *elem = MaybeUninit::new(Option::None);
         }

         //remove the MaybeUninit part of the type to make it a an option array
         unsafe { mem::transmute::<_, [Option<[Option<Vec<u8>>; ROWS_PER_PAGE]>; TABLE_MAX_PAGES]>(_pages)}
      };
   }

   pub fn get_row(&mut self, page_num: usize, row_idx: usize)-> &mut Option<Vec<u8>> {
      let page = &mut self.pages[(page_num as usize)];
      if let Option::None =  page {
         let mut _page: [Option<Vec<u8>>; ROWS_PER_PAGE] = {
            let mut _init_page : [MaybeUninit<Option<Vec<u8>>>; ROWS_PER_PAGE] = unsafe {
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
}

#[cfg(test)]
mod tests {
   use super::*;
   
   #[test]
   fn constructor_test() {
      let pager  = Pager::open_pager(String::from("test_file.txt"));
      assert_eq!(true, true)
   }
}
