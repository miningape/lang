pub mod print;

use crate::{types::FunctionType, value::Value};

pub trait Callable: std::fmt::Debug {
    fn signature(&self) -> String;
    fn call(&mut self, arguments: Vec<Value>) -> Result<Value, String>;
    fn get_type(&mut self) -> Result<FunctionType, String>;
    fn clone(&self) -> Box<dyn Callable>;
}
