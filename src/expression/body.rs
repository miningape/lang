use crate::{
    types::{BaseType, Type},
    value::Value,
};

use super::{Expression, Interpreter};

pub struct Body {
    pub body: Vec<Box<dyn Expression>>,
}

impl Expression for Body {
    fn check_type(&self, type_checker: &mut Interpreter<Type>) -> Result<Type, String> {
        type_checker.push_environment();

        let mut result = Type::BaseType(BaseType::Null);
        for expression in self.body.iter() {
            result = expression.check_type(type_checker)?;
        }

        type_checker.pop_environment()?;

        Ok(result)
    }

    fn interpret(&self, interpreter: &mut Interpreter<Value>) -> Result<Value, String> {
        let mut last_result = Value::Null;

        interpreter.push_environment();
        for expression in self.body.iter() {
            last_result = expression.interpret(interpreter)?;
        }
        interpreter.pop_environment()?;

        return Ok(last_result);
    }

    fn to_string(&self) -> String {
        format!(
            "{{ \"type\": \"Body\", \"body\": [{}]}}",
            self.body
                .iter()
                .map(|expr| expr.to_string())
                .fold("".to_owned(), |acc, expr| acc + &expr)
        )
    }
}
