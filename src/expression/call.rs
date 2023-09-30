use crate::value::Value;

use super::{Expression, Interpreter};

pub struct Call {
    pub target: Box<dyn Expression>,
    pub arguments: Vec<Box<dyn Expression>>,
}

impl Expression for Call {
    fn interpret(&self, interpreter: &mut Interpreter) -> Result<Value, String> {
        if let Value::Function(callee) = self.target.interpret(interpreter)? {
            let mut arguments = Vec::new();

            for argument in self.arguments.iter() {
                let value = argument.interpret(interpreter)?;
                arguments.push(value);
            }

            return callee.borrow().clone().borrow_mut().call(arguments);
        }
        panic!("Expression(Call).interpret - not implemented!")
    }

    fn to_string(&self) -> String {
        let target = self.target.to_string();
        let arguments = self
            .arguments
            .iter()
            .map(|arg| arg.to_string())
            .collect::<Vec<_>>();
        format!(
            "{{ \"type\": \"Call\", \"target\": {}, \"arguments\": {:#?} }}",
            target, arguments
        )
    }
}
