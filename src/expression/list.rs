use crate::{data::list::List, types::Type, value::Value};

use super::{Expression, Interpreter};

pub struct ListLiteral {
    pub elements: Vec<Box<dyn Expression>>,
}

impl Expression for ListLiteral {
    fn check_type(
        &self,
        type_interpreter: &mut Interpreter<Type>,
    ) -> Result<crate::types::Type, String> {
        self.elements
            .iter()
            .map(|expression| expression.check_type(type_interpreter))
            .collect::<Result<Vec<Type>, String>>()?
            .iter()
            .cloned()
            .reduce(|acc, cur| {
                if cur.is_sub_type_of(&acc) {
                    acc
                } else {
                    Type::Or(Box::from(acc.clone()), Box::from(cur.clone()))
                }
            })
            .map(Box::from)
            .map(Type::List)
            .ok_or(String::from("Cannot infer type of empty arrays (yet)"))
    }

    fn interpret(
        &self,
        interpreter: &mut super::Interpreter<crate::value::Value>,
    ) -> Result<crate::value::Value, String> {
        Ok(Value::List(List {
            vector: self
                .elements
                .iter()
                .map(|expression| expression.interpret(interpreter))
                .collect::<Result<Vec<_>, _>>()?,
        }))
    }

    fn to_string(&self) -> String {
        format!(
            "{{ \"type\": \"ListLiteral\", \"elements\": [{}] }}",
            self.elements
                .iter()
                .map(|elem| elem.to_string())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}
