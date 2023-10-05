pub mod print;

use crate::value::Value;

pub trait Callable {
    fn signature(&self) -> String;
    fn call(&mut self, arguments: Vec<Value>) -> Result<Value, String>;
    fn clone(&self) -> Box<dyn Callable>;
}
