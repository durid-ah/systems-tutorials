use super::table_ref_ext::TableRef;

pub struct Cursor {
   table: TableRef,
   pub row_num: u64,
   pub end_of_table: bool
}

