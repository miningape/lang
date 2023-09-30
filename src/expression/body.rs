use crate::value::Value;

use super::{Expression, Interpreter};

pub struct Body {
    pub body: Vec<Box<dyn Expression>>,
}

impl Expression for Body {
    fn interpret(&self, interpreter: &mut Interpreter) -> Result<Value, String> {
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
