use crate::{types::Type, value::Value};

use super::{Expression, Interpreter};

pub struct Assign {
    pub key: String,
    pub value: Box<dyn Expression>,
}

impl Expression for Assign {
    fn check_type(&self, type_interpreter: &mut Interpreter<Type>) -> Result<Type, String> {
        let assigned_type = self.value.check_type(type_interpreter)?;
        let variable = match type_interpreter.get(self.key.clone()) {
            None => {
                return Err(format!(
                    "Assigning variable that does not exist {}",
                    self.key,
                ))
            }
            Some(value) => value,
        };

        if !variable.mutable {
            return Err(format!("Variable \"{}\" is not mutable", self.key));
        }

        if !assigned_type.is_sub_type_of(&variable.value) {
            return Err(format!(
                "Actual type assigned to \"{}\" was {:#?} which doesn't subtype {:#?}",
                self.key, assigned_type, variable.value
            ));
        }

        Ok(assigned_type)
    }

    fn interpret(&self, interpreter: &mut Interpreter<Value>) -> Result<Value, String> {
        let actual_value = self.value.interpret(interpreter)?;
        interpreter.set(self.key.clone(), actual_value.clone())?;
        Ok(actual_value)
    }

    fn to_string(&self) -> String {
        format!(
            "{{ \"type\": \"Assign\", \"key\": \"{}\", \"value\": {} }}",
            self.key,
            self.value.to_string()
        )
    }
}
