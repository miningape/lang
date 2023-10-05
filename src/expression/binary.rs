use crate::{
    tokeniser::Operator,
    types::{BaseType, Type},
    value::Value,
};

use super::{Expression, Interpreter};

pub struct Binary {
    pub left: Box<dyn Expression>,
    pub operator: Operator,
    pub right: Box<dyn Expression>,
}

fn typeof_add(left: Type, right: Type) -> Type {
    if let Type::BaseType(BaseType::Any) = left {
        return Type::Or(
            Box::from(Type::BaseType(BaseType::String)),
            Box::from(Type::BaseType(BaseType::Number)),
        );
    }

    if let Type::BaseType(BaseType::Any) = right {
        return Type::Or(
            Box::from(Type::BaseType(BaseType::String)),
            Box::from(Type::BaseType(BaseType::Number)),
        );
    }

    if let Type::BaseType(BaseType::Any) = right {}

    if left.is_sub_type_of(&Type::BaseType(BaseType::Number))
        && right.is_sub_type_of(&Type::BaseType(BaseType::Number))
    {
        return Type::BaseType(BaseType::Number);
    }

    return Type::BaseType(BaseType::String);
}

fn assert_type_for(
    symbol: &str,
    type_: Type,
    left: Type,
    right: Type,
    return_type: Type,
) -> Result<Type, String> {
    if left.is_sub_type_of(&type_) && right.is_sub_type_of(&type_) {
        return Ok(return_type);
    }

    return Err(format!("Cannot not {} non-{:?} values", symbol, type_));
}

impl Expression for Binary {
    fn check_type(&self, type_interpreter: &mut Interpreter<Type>) -> Result<Type, String> {
        let left = self.left.check_type(type_interpreter)?;
        let right = self.right.check_type(type_interpreter)?;

        match self.operator {
            Operator::Plus => Ok(typeof_add(left, right)),
            Operator::Minus => assert_type_for(
                "subtract",
                Type::BaseType(BaseType::Number),
                left,
                right,
                Type::BaseType(BaseType::Number),
            ),
            Operator::Star => assert_type_for(
                "multiply",
                Type::BaseType(BaseType::Number),
                left,
                right,
                Type::BaseType(BaseType::Number),
            ),
            Operator::Slash => assert_type_for(
                "divide",
                Type::BaseType(BaseType::Number),
                left,
                right,
                Type::BaseType(BaseType::Number),
            ),
            Operator::Equal => Ok(Type::BaseType(BaseType::Boolean)),
            Operator::NotEqual => Ok(Type::BaseType(BaseType::Boolean)),
            Operator::GreaterThan => assert_type_for(
                "compare >",
                Type::BaseType(BaseType::Number),
                left,
                right,
                Type::BaseType(BaseType::Boolean),
            ),
            Operator::GreaterThanOrEqual => assert_type_for(
                "compare >=",
                Type::BaseType(BaseType::Number),
                left,
                right,
                Type::BaseType(BaseType::Boolean),
            ),
            Operator::LesserThan => assert_type_for(
                "compare <",
                Type::BaseType(BaseType::Number),
                left,
                right,
                Type::BaseType(BaseType::Boolean),
            ),
            Operator::LesserThanOrEqual => assert_type_for(
                "compare <=",
                Type::BaseType(BaseType::Number),
                left,
                right,
                Type::BaseType(BaseType::Boolean),
            ),
            Operator::And => assert_type_for(
                "`and` (&)",
                Type::BaseType(BaseType::Boolean),
                left,
                right,
                Type::BaseType(BaseType::Boolean),
            ),
            Operator::Or => assert_type_for(
                "`or` (|)",
                Type::BaseType(BaseType::Boolean),
                left,
                right,
                Type::BaseType(BaseType::Boolean),
            ),
            Operator::Not => Err("Cannot use ! (not) in a binary expression".to_owned()),
        }
    }

    fn interpret(&self, interpreter: &mut Interpreter<Value>) -> Result<Value, String> {
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
