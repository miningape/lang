use crate::value::Value;

use super::Callable;

#[derive(Clone)]
pub struct Print {}

impl Callable for Print {
    fn call(&mut self, arguments: Vec<Value>) -> Result<Value, String> {
        let s: String = arguments.iter().map(|v| v.to_string()).collect();
        println!("{}", s);
        Ok(Value::String(s))
    }

    fn clone(&self) -> Box<dyn Callable> {
        return Box::from(Print {});
    }

    fn signature(&self) -> String {
        String::from("Print")
    }
}
