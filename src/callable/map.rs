use crate::{
    data::list::List,
    types::{BaseType, FunctionType, Type},
    value::Value,
};

use super::Callable;

#[derive(Debug)]
pub struct Map {}

impl Callable for Map {
    fn call(&mut self, arguments: Vec<Value>) -> Result<Value, String> {
        if let Some(Value::List(list)) = arguments.get(0) {
            if let Some(Value::Function(callable)) = arguments.get(1) {
                return Ok(Value::List(List {
                    vector: list
                        .vector
                        .iter()
                        .map(|elem| callable.borrow().clone().call([elem.clone()].to_vec()))
                        .collect::<Result<Vec<Value>, String>>()?,
                }));
            }
        }

        Err(format!("Dumbass"))
    }

    fn get_type(&mut self) -> Result<FunctionType, String> {
        Ok(FunctionType::Literal(
            [
                Type::List(Box::from(Type::BaseType(BaseType::Any))),
                Type::Function(Box::from(FunctionType::Literal(
                    [Type::BaseType(BaseType::Any)].to_vec(),
                    Type::BaseType(BaseType::Any),
                ))),
            ]
            .to_vec(),
            Type::List(Box::from(Type::BaseType(BaseType::Any))),
        ))
    }

    fn clone(&self) -> Box<dyn Callable> {
        return Box::from(Map {});
    }

    fn signature(&self) -> String {
        String::from("Map")
    }
}
