use crate::{
    tokeniser::Operator,
    types::{BaseType, Type},
    value::Value,
};

use super::{Expression, Interpreter};

pub struct Unary {
    pub operator: Operator,
    pub value: Box<dyn Expression>,
}

impl Expression for Unary {
    fn check_type(&self, type_interpreter: &mut Interpreter<Type>) -> Result<Type, String> {
        let value_type = self.value.check_type(type_interpreter)?;

        if self.operator == Operator::Not {
            if !value_type.is_sub_type_of(&Type::BaseType(BaseType::Boolean)) {
                return Err(String::from("Cannot not use ! on a non boolean value"));
            }

            return Ok(Type::BaseType(BaseType::Boolean));
        }

        if self.operator == Operator::Minus {
            if !value_type.is_sub_type_of(&Type::BaseType(BaseType::Number)) {
                return Err(String::from(
                    "Cannot not use - (minus / negation) on a non number value",
                ));
            }

            return Ok(Type::BaseType(BaseType::Number));
        }

        Err(format!(
            "Could not use operator {:?} on value with type {:?}",
            self.operator, value_type
        ))
    }

    fn interpret(&self, interpreter: &mut Interpreter<Value>) -> Result<Value, String> {
        if self.operator == Operator::Not {
            return self.value.interpret(interpreter)?.not();
        }

        if self.operator != Operator::Minus {
            return Err(format!("Trying to perform a unary operation without minus"));
        }

        match self.value.interpret(interpreter)? {
            Value::Number(number) => Ok(Value::Number(-number)),
            Value::Return(_) => Err(format!("Cannot negate return")),
            Value::Null => Err(format!("Cannot negate null")),
            Value::String(string) => Err(format!("Cannot negate string: {}", string)),
            Value::Boolean(boolean) => Err(format!("Cannot negate boolean: {}", boolean)),
            Value::Function(function) => Err(format!(
                "Cannot negate function: {}",
                function.borrow().signature()
            )),
        }
    }

    fn to_string(&self) -> String {
        format!(
            "{{ \"type\": \"Unary\", \"operator\": \"{:#?}\", \"value\": {}}}",
            self.operator,
            self.value.to_string()
        )
    }
}
