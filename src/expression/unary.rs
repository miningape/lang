use crate::{tokeniser::Operator, value::Value};

use super::{Expression, Interpreter};

pub struct Unary {
    pub operator: Operator,
    pub value: Box<dyn Expression>,
}

impl Expression for Unary {
    fn interpret(&self, interpreter: &mut Interpreter) -> Result<Value, String> {
        if self.operator == Operator::Not {
            return self.value.interpret(interpreter)?.not();
        }

        if self.operator != Operator::Minus {
            return Err(format!("Trying to perform a unary operation without minus"));
        }

        match self.value.interpret(interpreter)? {
            Value::Number(number) => Ok(Value::Number(-number)),
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
