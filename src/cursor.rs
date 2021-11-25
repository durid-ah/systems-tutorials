use crate::table::TableRef;

pub struct Cursor {
   table: TableRef,
   pub row_num: u64,
   pub end_of_table: bool
}

