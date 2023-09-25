use crate::value::Value;

use super::{Expression, Interpreter};

pub struct Variable {
    pub name: String,
}

impl Expression for Variable {
    fn interpret(&self, interpreter: &mut Interpreter) -> Result<Value, String> {
        match interpreter.get(self.name.clone()) {
            Some(value) => Ok(value),
            None => Err(format!(
                "Could not access variable: {}. It was never created.",
                self.name
            )),
        }
    }

    fn to_string(&self) -> String {
        format!("{{ \"type\": \"Variable\", \"name\": \"{}\" }}", self.name)
    }
}
