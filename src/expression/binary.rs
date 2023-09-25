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
        let right = self.right.interpret(interpreter)?;

        match self.operator {
            Operator::Plus => left.add(right),
            Operator::Minus => left.sub(right),
            _ => Err(format!("Cannot use the operation: {:#?}", self.operator)),
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
