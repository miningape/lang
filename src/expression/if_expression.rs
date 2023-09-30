use crate::value::Value;

use super::{Expression, Interpreter};

pub struct If {
    pub condition: Box<dyn Expression>,
    pub body: Box<dyn Expression>,
    pub else_body: Option<Box<dyn Expression>>,
}

impl Expression for If {
    fn interpret(&self, interpreter: &mut Interpreter) -> Result<Value, String> {
        let condition = self.condition.interpret(interpreter)?;

        if let Value::Boolean(boolean) = condition {
            if boolean {
                return self.body.interpret(interpreter);
            } else {
                return match &self.else_body {
                    Some(body) => body.interpret(interpreter),
                    None => Ok(Value::Null),
                };
            }
        }

        Err(format!("Condition for `if` did not resolve to a boolean"))
    }

    fn to_string(&self) -> String {
        format!(
            "{{ \"type\": \"If\", \"condition\": {}, \"body\": {}, \"else_body\": {} }}",
            self.condition.to_string(),
            self.body.to_string(),
            match &self.else_body {
                Some(body) => body.to_string(),
                None => "null".to_owned(),
            }
        )
    }
}
