use std::cell::RefCell;
use std::rc::Rc;
use core::mem::{self, MaybeUninit};
use std::vec::Vec;

use crate::file_manager::FileManager;
use crate::db_config::DBConfig;

use crate::size_constants::{
   ROWS_PER_PAGE, TABLE_MAX_PAGES };

pub type RowRef = Rc<RefCell<Option<Vec<u8>>>>;
type Page = [RowRef; ROWS_PER_PAGE as usize];
type UninitPage = [MaybeUninit<RowRef>; ROWS_PER_PAGE as usize];

pub struct Pager {
   file_mgr: FileManager,
   config: DBConfig,
   pages: [Option<Page>; TABLE_MAX_PAGES as usize]
}

impl Pager {
   pub fn open_pager(file_mgr: FileManager, config: DBConfig) -> Pager {
      let pages = Pager::init_pages();

      Pager { pages, file_mgr, config}
   }

   pub fn calculate_num_rows(&self) -> u64 { self.config.num_rows }

   pub fn close_pager(&mut self, num_rows: u64) {
      let full_page_count = num_rows / ROWS_PER_PAGE;
      let leftover_rows = num_rows % ROWS_PER_PAGE;

      for i in 0..(full_page_count as usize) {
         let page = &self.pages[i];
         Pager::flush_page(page, i as u64, &mut self.file_mgr)
      }

      if leftover_rows != 0 {
         let page = &self.pages[full_page_count as usize];
         Pager::flush_page(page, full_page_count, &mut self.file_mgr);
      }

      self.config.num_rows = num_rows;
      DBConfig::save(&self.config);
      self.file_mgr.close_file()
   }

   fn flush_page(page: &Option<Page>, page_num: u64, file_mgr: &mut FileManager) {
      let page_to_write = page.as_ref().expect("Attempting To Flush None Page");

      let _ = file_mgr.seek_to_page(page_num);

      for i in 0..(ROWS_PER_PAGE as usize) {
         let page_to_write = page_to_write[i].borrow();
         match &*page_to_write {
            Some(unwrapped_row) =>
               file_mgr.write_row(unwrapped_row, unwrapped_row.len() as u16),
            None => continue
         }
      }
   }

   pub fn get_row(&mut self, row_num: u64)-> RowRef {
      let row_num = row_num - 1;
      let page_num: usize = self.get_page_idx(row_num) as usize;
      let row_idx: usize = self.get_row_idx(row_num) as usize;

      let page = &mut self.pages[page_num];

      if let Option::None =  page {
         self.file_mgr.seek_to_page(page_num as u64);
         Pager::init_page_rows(page, &mut self.file_mgr);
      }

      let res = page.as_mut().unwrap();
      res[row_idx].clone()
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

   /// Init the rows of the page to Option::None
   fn init_page_rows(page: &mut Option<Page>, file_mgr: &mut FileManager) {
      let mut _page: Page = {
         let mut _init_page : UninitPage = unsafe {
            MaybeUninit::uninit().assume_init()
         };

         for r in &mut _init_page[..] {
            let row = Rc::new(RefCell::new(file_mgr.read_row()));
            *r = MaybeUninit::new(row);
         }

         unsafe {mem::transmute(_init_page)}
      };

      *page = Option::Some(_page);
   }
}