use crate::pager::RowRef;
use super::table_ref_ext::TableRef;

pub struct Cursor {
   table: TableRef,
   pub row_num: u64,
   pub end_of_table: bool
}

impl Cursor {
   pub fn new(row_num: u64, table: TableRef, end_of_table: bool) -> Cursor {
      Cursor{row_num, table, end_of_table}
   }

   pub fn get_val(&mut self) -> RowRef {
      self.table.borrow_mut().get_row(self.row_num)
   }

   pub fn set_val(&mut self, bin_row: Vec<u8>) {
      let table_row = self.table.borrow_mut().get_row(self.row_num);
      let mut row = table_row.borrow_mut();
      *row = Some(bin_row);
      self.table.borrow_mut().num_rows += 1;
   }

   pub fn advance(&mut self) {
      if !self.end_of_table {
         self.row_num += 1;
         self.end_of_table =
            self.row_num >= self.table.borrow().num_rows;
      }
   }
}