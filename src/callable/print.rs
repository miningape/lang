use crate::{
    types::{BaseType, FunctionType, Type},
    value::Value,
};

use super::Callable;

#[derive(Debug)]
pub struct Print {}

impl Callable for Print {
    fn call(&mut self, arguments: Vec<Value>) -> Result<Value, String> {
        let s: String = arguments.iter().map(|v| v.to_string()).collect();
        println!("{}", s);
        Ok(Value::String(s))
    }

    fn get_type(&mut self) -> Result<FunctionType, String> {
        Ok(FunctionType::ArrayArgs(
            Type::BaseType(BaseType::Any),
            Type::BaseType(BaseType::String),
        ))
    }

    fn clone(&self) -> Box<dyn Callable> {
        return Box::from(Print {});
    }

    fn signature(&self) -> String {
        String::from("Print")
    }
}
