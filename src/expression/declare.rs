use crate::{
    environment::Variable,
    types::{BaseType, Type},
    value::Value,
};

use super::{Expression, Interpreter};

pub struct Declare {
    pub key: String,
    pub assigned_type: Option<Variable<Type>>,
    pub value: Box<dyn Expression>,
}

impl Expression for Declare {
    fn check_type(&self, type_interpreter: &mut Interpreter<Type>) -> Result<Type, String> {
        let mut actual_type = Variable {
            mutable: false,
            value: self.value.check_type(type_interpreter)?,
        };

        if let Some(assigned_type) = self.assigned_type.as_ref() {
            if assigned_type
                .value
                .is_sub_type_of(&Type::BaseType(BaseType::Infer))
            {
                actual_type.mutable = assigned_type.mutable;
            } else {
                if !actual_type.value.is_sub_type_of(&assigned_type.value) {
                    return Err(format!(
                        "Actual type assigned to \"{}\" was {:#?} which doesn't subtype {:#?}",
                        self.key, actual_type, assigned_type
                    ));
                }

                if let Some(_) = actual_type.value.get_return_type() {
                    return Err(format!("Cannot assign variable to return value"));
                }

                actual_type = assigned_type.clone()
            }
        }

        type_interpreter.create(self.key.clone(), actual_type.clone())?;
        Ok(actual_type.value)
    }

    fn interpret(&self, interpreter: &mut Interpreter<Value>) -> Result<Value, String> {
        let actual_value = Variable {
            mutable: self.assigned_type.as_ref().is_some_and(|t| t.mutable),
            value: self.value.interpret(interpreter)?,
        };

        interpreter.create(self.key.clone(), actual_value.clone())?;
        Ok(actual_value.value)
    }

    fn to_string(&self) -> String {
        format!(
            "{{ \"type\": \"Declare\", \"key\": \"{}\", \"value\": {}, \"type\": {:#?} }}",
            self.key,
            self.value.to_string(),
            self.assigned_type
        )
    }
}
