use std::cell::RefCell;
use std::rc::Rc;
use crate::table::Table;

type TableRef = Rc<RefCell<Table>>;

pub struct Cursor {
   table: TableRef,
   pub row_num: u64,
   pub end_of_table: bool
}

