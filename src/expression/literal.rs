use crate::{
    tokeniser,
    types::{BaseType, Type},
    value::Value,
};

use super::{Expression, Interpreter};

pub struct Literal {
    pub value: tokeniser::Literal,
}

impl Expression for Literal {
    fn check_type(&self, _type_interpreter: &mut Interpreter<Type>) -> Result<Type, String> {
        return Ok(match self.value {
            tokeniser::Literal::Null => Type::BaseType(BaseType::Null),
            tokeniser::Literal::Number(_) => Type::BaseType(BaseType::Number),
            tokeniser::Literal::String(_) => Type::BaseType(BaseType::String),
            tokeniser::Literal::Boolean(_) => Type::BaseType(BaseType::Boolean),
        });
    }

    fn interpret(&self, _interpreter: &mut Interpreter<Value>) -> Result<Value, String> {
        return Ok(match self.value.clone() {
            tokeniser::Literal::Null => Value::Null,
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
