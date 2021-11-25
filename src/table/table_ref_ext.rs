use crate::table::Cursor;
use std::cell::RefCell;
use std::rc::Rc;

use super::Table;

pub type TableRef = Rc<RefCell<Table>>;

pub trait TableRefExt {
   fn start_cursor(&mut self) -> Cursor;
   fn end_cursor(&mut self) -> Cursor;
}

impl TableRefExt for TableRef {
   fn start_cursor(&mut self) -> Cursor { todo!() }
   fn end_cursor(&mut self) -> Cursor { todo!() }
}