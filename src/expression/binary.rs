use crate::{tokeniser::Operator, value::Value};

use super::{Expression, Interpreter};

pub struct Binary {
    pub left: Box<dyn Expression>,
    pub operator: Operator,
    pub right: Box<dyn Expression>,
}

impl Expression for Binary {
    fn interpret(&self, interpreter: &mut Interpreter) -> Result<Value, String> {
        let left = self.left.interpret(interpreter)?;

        if let Value::Boolean(boolean) = left {
            if self.operator == Operator::And && boolean == false {
                return Ok(Value::Boolean(false));
            }

            if self.operator == Operator::Or && boolean == true {
                return Ok(Value::Boolean(true));
            }
        }

        let right = self.right.interpret(interpreter)?;

        match self.operator {
            Operator::Plus => left.add(right),
            Operator::Minus => left.sub(right),
            Operator::Star => left.mul(right),
            Operator::Slash => left.div(right),
            Operator::Equal => left.equals(right),
            Operator::NotEqual => left.equals(right)?.not(),
            Operator::GreaterThan => left.greater(right),
            Operator::GreaterThanOrEqual => left.lesser(right)?.not(),
            Operator::LesserThan => left.lesser(right),
            Operator::LesserThanOrEqual => left.greater(right)?.not(),
            Operator::And => left.and(right),
            Operator::Or => left.or(right),
            Operator::Not => Err("Cannot use ! (not) in a binary expression".to_owned()),
        }
    }

    fn to_string(&self) -> String {
        format!(
            "{{ \"type\": \"Binary\", \"left\": {}, \"operator\": \"{:#?}\", \"right\": {}}}",
            self.left.to_string(),
            self.operator,
            self.right.to_string()
        )
    }
}
