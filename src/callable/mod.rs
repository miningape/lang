pub mod print;

use std::{cell::RefCell, rc::Rc};

use crate::value::Value;

pub trait Callable {
    fn signature(&self) -> String;
    fn call(&mut self, arguments: Vec<Value>) -> Result<Value, String>;
    fn clone(&self) -> Rc<RefCell<dyn Callable>>;
}
