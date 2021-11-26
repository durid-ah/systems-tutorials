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
   fn start_cursor(&mut self) -> Cursor {
      Cursor::new(1, self.clone(), self.borrow().num_rows == 0)
   }
   
   fn end_cursor(&mut self) -> Cursor {
      Cursor::new(self.borrow().num_rows + 1, self.clone(), true)
   }
}