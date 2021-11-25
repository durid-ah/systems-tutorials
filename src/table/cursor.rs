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
}