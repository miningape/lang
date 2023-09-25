use crate::{tokeniser::Operator, value::Value};

use super::{Expression, Interpreter};

pub struct Unary {
    operator: Operator,
    value: dyn Expression,
}

impl Expression for Unary {
    fn interpret(&self, interpreter: &mut Interpreter) -> Result<Value, String> {
        if self.operator != Operator::Minus {
            return Err(format!("Trying to perform a unary operation without minus"));
        }

        match self.value.interpret(interpreter)? {
            Value::Number(number) => Ok(Value::Number(-number)),
            Value::String(string) => Err(format!("Cannot negate string: {}", string)),
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
