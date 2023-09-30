use crate::{tokeniser, value::Value};

use super::{Expression, Interpreter};

pub struct Literal {
    pub value: tokeniser::Literal,
}

impl Expression for Literal {
    fn interpret(&self, _interpreter: &mut Interpreter) -> Result<Value, String> {
        return Ok(match self.value.clone() {
            tokeniser::Literal::Number(number) => Value::Number(number),
            tokeniser::Literal::String(string) => Value::String(string),
            tokeniser::Literal::Boolean(boolean) => Value::Boolean(boolean),
        });
    }

    fn to_string(&self) -> String {
        format!(
            "{{ \"type\": \"Literal\", \"value\": {} }}",
            self.value.to_owned().to_string()
        )
    }
}
