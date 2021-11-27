pub use cursor::Cursor;
pub use table_ref_ext::{TableRef, TableRefExt};
pub use row::{Row, row_util};

use crate::pager::RowRef;
use crate::pager::Pager;

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