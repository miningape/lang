use crate::{
    types::{BaseType, Type},
    value::Value,
};

use super::{Expression, Interpreter};

pub struct Return {
    pub expression: Option<Box<dyn Expression>>,
}

impl Expression for Return {
    fn check_type(&self, type_interpreter: &mut Interpreter<Type>) -> Result<Type, String> {
        match &self.expression {
            None => Ok(Type::Return(Box::from(Type::BaseType(BaseType::Null)))),
            Some(expression) => Ok(Type::Return(Box::from(
                expression.check_type(type_interpreter)?,
            ))),
        }
    }

    fn interpret(&self, interpreter: &mut Interpreter<Value>) -> Result<Value, String> {
        let mut return_value = Value::Null;

        if let Some(expression) = &self.expression {
            return_value = expression.interpret(interpreter)?;
        }

        Ok(Value::Return(Box::from(return_value)))
    }

    fn to_string(&self) -> String {
        format!(
            "{{ \"type\": \"Return\", \"expression\": \"{}\" }}",
            match &self.expression {
                None => "null".to_owned(),
                Some(expression) => expression.to_string(),
            }
        )
    }
}
