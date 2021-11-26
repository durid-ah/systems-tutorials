pub use cursor::Cursor;
pub use table_ref_ext::TableRef;
pub use row::Row;

use crate::pager::RowRef;
use crate::pager::Pager;
use crate::size_constants::TABLE_MAX_ROWS;
use row::{
   row_util::serialize_row, 
   row_util::deserialize_row
};

mod cursor;
mod table_ref_ext;
mod row;

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
   pub fn init_table(pager: Pager) -> Table {
      let num_rows = pager.calculate_num_rows();

      Table{num_rows, pager}
   }

   /// Get a reference to the row in the table based on the row number
   pub fn get_row(&mut self, row_num: u64) -> RowRef {
      self.pager.get_row(row_num)
   }

   /// Insert the row into the next available slot
   pub fn insert_row(&mut self, row: &Row) -> ExecuteResult {
      if self.num_rows >= TABLE_MAX_ROWS {
         return ExecuteResult::TableFull;
      }

      let bin_row = serialize_row(row);
      let table_row = self.get_row(self.num_rows + 1);
      let mut row = table_row.borrow_mut();

      *row = Some(bin_row);
      self.num_rows += 1;
      
      ExecuteResult::Success
   }

   pub fn select_rows(&mut self) -> Vec<Row> {
      let mut res: Vec<Row> = Vec::new();
      for i in 1..(self.num_rows + 1) {
         let row_ref = self.get_row(i);
         let borrowed_row = row_ref.borrow();
         let r = borrowed_row.as_ref();

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
}