use crate::pager::Pager;
use super::row::{Row, serialize_row, deserialize_row};

use super::size_constants::{
   TABLE_MAX_ROWS,
   ROW_SIZE
};

pub enum ExecuteResult {
   TableFull,
   Success
}

pub struct Table {
   pub num_rows: u64,
   pub pager: Pager
}

impl Table {
   /// Create the table
   pub fn open_db(file_name: String) -> Table {
      let _pager = Pager::open_pager(file_name);
      let num_rows = _pager.file_length / ROW_SIZE;
      return Table{num_rows, pager: _pager}
   }

   /// Get a reference to the row in the table based on the row number
   pub fn get_row(&mut self, row_num: u64) -> &mut Option<Vec<u8>> { self.pager.get_row(row_num)}

   /// Insert the row into the next available slot
   pub fn insert_row(&mut self, row: &Row) -> ExecuteResult {
      if self.num_rows >= TABLE_MAX_ROWS {
         return ExecuteResult::TableFull;
      }

      println!("Executing INSERT");

      let bin_row = serialize_row(row);
      let table_row = self.get_row(self.num_rows);
      *table_row = Some(bin_row);
      self.num_rows += 1;
      
      ExecuteResult::Success
   }

   pub fn select_rows(&mut self) -> Vec<Row> {
      let mut res: Vec<Row> = Vec::new();
      for i in 0..(self.num_rows + 1) {
         let r = self.get_row(i).clone(); //TODO: Change to borrow?

         if let Option::Some(row) = r {
            let deserialized_r = deserialize_row(&row);
            res.push(deserialized_r);            
         }
      }

      res
   }

   pub fn close_table(&mut self) {
      self.pager.close_pager(self.num_rows);
   }
}

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
   fn insert_into_table() {
      let mut r = Table::open_db(String::from("test_file.txt"));
      r.insert_row(&Row::new(1, "stuff", "stuff").unwrap());
   }
}