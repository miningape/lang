use crate::{types::Type, value::Value};

use super::{Expression, Interpreter};

pub struct Assign {
    pub key: String,
    pub value: Box<dyn Expression>,
}

impl Expression for Assign {
    fn check_type(&self, type_interpreter: &mut Interpreter<Type>) -> Result<Type, String> {
        let actual_type = self.value.check_type(type_interpreter)?;
        type_interpreter.set(self.key.clone(), actual_type.clone());
        Ok(actual_type)
    }

    fn interpret(&self, interpreter: &mut Interpreter<Value>) -> Result<Value, String> {
        let actual_value = self.value.interpret(interpreter)?;
        interpreter.set(self.key.clone(), actual_value.clone());
        Ok(actual_value)
    }

    fn to_string(&self) -> String {
        format!(
            "{{ \"type\": \"Assign\", \"key\": \"{}\", \"value\": {} }}",
            self.key,
            self.value.to_string()
        )
    }
}
