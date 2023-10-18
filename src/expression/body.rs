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

        let mut return_types: Vec<Type> = Vec::new();

        let mut last_type = None;
        for expression in self.body.iter() {
            let expression_type = expression.check_type(type_checker)?;

            if let Some(return_type) = expression_type.get_return_type() {
                return_types.push(return_type.clone());
                last_type = None;
                continue;
            }

            last_type = Some(expression_type);
        }

        type_checker.pop_environment()?;

        let mut block_return_type: Option<Type> = last_type;
        for return_type in return_types {
            if let None = block_return_type.clone() {
                block_return_type = Some(return_type);
                continue;
            }

            if return_type.is_sub_type_of(&block_return_type.clone().unwrap()) {
                continue;
            }

            block_return_type = Some(Type::Or(
                Box::from(block_return_type.clone().unwrap()),
                Box::from(return_type),
            ))
        }

        Ok(match block_return_type.clone() {
            None => Type::BaseType(BaseType::Null),
            Some(return_type) => return_type,
        })
    }

    fn interpret(&self, interpreter: &mut Interpreter<Value>) -> Result<Value, String> {
        let mut last_result = Value::Null;

        interpreter.push_environment();
        for expression in self.body.iter() {
            last_result = expression.interpret(interpreter)?;

            if let Value::Return(return_value) = last_result {
                interpreter.pop_environment()?;
                return Ok(*return_value);
            }
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
