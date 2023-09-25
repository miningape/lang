use super::{Expression, Interpreter};

pub struct Assign {
    pub key: String,
    pub value: Box<dyn Expression>,
}

impl Expression for Assign {
    fn interpret(&self, interpreter: &mut Interpreter) -> Result<crate::value::Value, String> {
        let actual_value = self.value.interpret(interpreter)?;
        interpreter.set(self.key.clone(), actual_value.clone());
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
