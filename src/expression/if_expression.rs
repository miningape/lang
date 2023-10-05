use crate::{
    types::{BaseType, Type},
    value::Value,
};

use super::{Expression, Interpreter};

pub struct If {
    pub condition: Box<dyn Expression>,
    pub body: Box<dyn Expression>,
    pub else_body: Option<Box<dyn Expression>>,
}

impl Expression for If {
    fn check_type(&self, type_interpreter: &mut Interpreter<Type>) -> Result<Type, String> {
        let typeof_condition = self.condition.check_type(type_interpreter)?;

        if !typeof_condition.is_sub_type_of(&Type::BaseType(BaseType::Boolean)) {
            return Err(String::from("Tried to use an expression that evaluated to non boolean as the condition in an `if` expression"));
        }

        let typeof_body = self.body.check_type(type_interpreter)?;
        let typeof_else_body = match &self.else_body {
            None => Type::BaseType(BaseType::Null),
            Some(body) => body.check_type(type_interpreter)?,
        };

        if typeof_body.is_sub_type_of(&typeof_else_body)
            || typeof_else_body.is_sub_type_of(&typeof_body)
        {
            return Ok(typeof_body);
        }

        Ok(Type::Or(
            Box::from(typeof_body),
            Box::from(typeof_else_body),
        ))
    }

    fn interpret(&self, interpreter: &mut Interpreter<Value>) -> Result<Value, String> {
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
