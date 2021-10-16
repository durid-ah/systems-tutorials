use crate::pager::Pager;
use std::mem::{self, MaybeUninit};
use super::row::{Row, _serialize_row, _deserialize_row};

use super::size_constants::{
   ROWS_PER_PAGE,
   TABLE_MAX_ROWS
};


pub enum ExecuteResult {
   TableFull,
   Success
}

pub struct Table {
   pub num_rows: usize,
   pub pager: Pager
}

impl Table {
   /// Create the table
   pub fn open_db(file_name: String) -> Table {
      let _pager = Pager::open_pager(file_name);
      return Table{num_rows: 0, pager: _pager}
   }

   /// Return the index of the page where the row number resides
   fn get_page_idx(&self, row_num: usize) -> usize { return row_num / ROWS_PER_PAGE}

   /// Get the row within the page where the row resides
   fn get_row_idx(&self, row_num: usize) -> usize { return row_num % ROWS_PER_PAGE }

   // TODO: Fix this mess!!!!
   /// Get a reference to the row in the table based on the row number
   pub fn get_row(&mut self, row_num: usize) -> &mut Option<Vec<u8>> {
      let page_num: usize = self.get_page_idx(row_num);
      let row_idx: usize = self.get_row_idx(row_num);

      self.pager.get_row(page_num, row_idx)
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

         if let Option::Some(row) = r {
            let deserialized_r = _deserialize_row(&row);
            res.push(deserialized_r);            
         }

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
      let r = Table::open_db(String::from("test_file.txt"));
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
      let mut r = Table::open_db(String::from("test_file.txt"));
      r.insert_row(&Row::new(1, "stuff", "stuff").unwrap());
      //println!("{:?}", r.pages);
      //assert!(false)
   }
}