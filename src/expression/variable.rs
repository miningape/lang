use crate::{types::Type, value::Value};

use super::{Expression, Interpreter};

pub struct Variable {
    pub name: String,
}

impl Expression for Variable {
    fn interpret(&self, interpreter: &mut Interpreter<Value>) -> Result<Value, String> {
        match interpreter.get(self.name.clone()) {
            Some(variable) => Ok(variable.value),
            None => Err(format!(
                "Could not access variable: {}. It was never created.",
                self.name
            )),
        }
    }

    fn check_type(&self, type_interpreter: &mut Interpreter<Type>) -> Result<Type, String> {
        match type_interpreter.get(self.name.clone()) {
            Some(variable) => Ok(variable.value),
            None => Err(format!(
                "Cannot get type of variable with name - {}",
                self.name
            )),
        }
    }

    fn to_string(&self) -> String {
        format!("{{ \"type\": \"Variable\", \"name\": \"{}\" }}", self.name)
    }
}
