use std::rc::Rc;
use core::cell::RefCell;

use super::Table;

pub type TableRef = Rc<RefCell<Table>>;
