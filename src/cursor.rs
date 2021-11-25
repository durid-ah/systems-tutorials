use std::rc::Rc;
use std::cell::RefCell;
use crate::table::Table;

pub type TableRef = Rc<RefCell<Table>>;

pub struct Cursor {
   table: TableRef,
   pub row_num: u64,
   pub end_of_table: bool
}

