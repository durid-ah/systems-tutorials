use bincode;
use serde::{Serialize, Deserialize};
use std::mem::{self, MaybeUninit};

const PAGE_SIZE: u32 = 4096;
const TABLE_MAX_PAGES: usize = 100;
const ROW_SIZE: usize = 307;
const ROWS_PER_PAGE: usize = (PAGE_SIZE as usize) / ROW_SIZE; // About 13 rows
const TABLE_MAX_ROWS: usize = ROWS_PER_PAGE * TABLE_MAX_PAGES;

/// convert the row to a vec<u8>
fn _serialize_row(row: &Row) -> Vec<u8> {
   bincode::serialize(&row).unwrap()
}

fn _deserialize_row(row: &Vec<u8>) -> Row {
   bincode::deserialize(&row).unwrap()
}

pub enum ExecuteResult {
   TableFull,
   Success
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Row {
   pub id: u32,
   pub username: String,
   pub email: String,
}

impl Row {
   pub fn new(id:u32, username: &str, email: &str) -> Result<Row, String> {

      if username.len() > 32 || username.len() == 0 {
         return Result::Err(String::from("username must have between 1 and 32 characcters"));
      }
      
      if email.len() > 255 || email.len() == 0 {
         return Result::Err(String::from("username must have between 1 and 255 characcters"));
      }

      Result::Ok(Row{id, username: String::from(username), email: String::from(email)})
   }
}

pub struct Table {
   pub num_rows: usize,
   pub pages: [Option<[Option<Vec<u8>>; ROWS_PER_PAGE]>; TABLE_MAX_PAGES]
}

impl Table {
   /// Create the table
   pub fn new() -> Table {
      
      let _pages = {
         let mut _pages: [MaybeUninit<Option<[Option<Vec<u8>>; ROWS_PER_PAGE]>>; TABLE_MAX_PAGES] = unsafe {
            MaybeUninit::uninit().assume_init()
         };

         for elem in &mut _pages[..] {
            *elem = MaybeUninit::new(Option::None);
         }

         unsafe { mem::transmute::<_, [Option<[Option<Vec<u8>>; ROWS_PER_PAGE]>; TABLE_MAX_PAGES]>(_pages)}
      };
      return Table{num_rows: 0, pages: _pages}
   }

   /// Return the index of the page where the row number resides
   fn get_page_idx(&self, row_num: usize) -> usize { return row_num / ROWS_PER_PAGE}

   /// Get the row within the page where the row resides
   fn get_row_idx(&self, row_num: usize) -> usize { return row_num % ROWS_PER_PAGE }

   /// Get a reference to the row in the table based on the row number
   pub fn get_row(&mut self, row_num: usize) -> &mut Option<Vec<u8>> {
      let page_num: usize = self.get_page_idx(row_num);
      let row_idx: usize = self.get_row_idx(row_num);

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

   /// Insert the row into the next available slot
   pub fn insert_row(&mut self, row: &Row) -> ExecuteResult {
      if self.num_rows >= TABLE_MAX_ROWS {
         return ExecuteResult::TableFull;
      }

      let bin_row = _serialize_row(row);
      let table_row = self.get_row(self.num_rows);
      *table_row = Some(bin_row);

      ExecuteResult::Success
   }

   pub fn select_rows(&mut self) -> Vec<Row> {
      let mut res: Vec<Row> = Vec::new();
      for i in 0..(self.num_rows + 1) {
         let r = self.get_row(i).clone();
      
         let deserialized_r = _deserialize_row(&r.unwrap());
         res.push(deserialized_r);
      }

      res
   }
}

// TODO: Write tests
#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn test_row_creation() {
      let new_row = Row::new(1, "user", "email");
      match new_row {
         Result::Ok(r) => {
            assert_eq!(r.id, 1);
            assert_eq!(r.username, "user");
            assert_eq!(r.email, "email");
         }
         _ => assert!(false, "Unable to create row")
      }
   }

   #[test]
   fn page_idx_test() {
      let r = Table::new();
      let first_page_idx = r.get_page_idx(0);
      let first_page_idx_2 = r.get_page_idx(12);
      
      assert_eq!(first_page_idx, 0,
         "The first row must return the first page in the table"
      );

      assert_eq!(first_page_idx_2, 0,
         "The 13th row must return the first page in the table"
      );
   }

   #[test]
   fn insert_into_table() {
      let mut r = Table::new();
      r.insert_row(&Row::new(1, "stuff", "stuff").unwrap());
      println!("{:?}", r.pages);
      assert!(false)
   }
}