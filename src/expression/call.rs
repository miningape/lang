use crate::{types::Type, value::Value};

use super::{Expression, Interpreter};

pub struct Call {
    pub target: Box<dyn Expression>,
    pub arguments: Vec<Box<dyn Expression>>,
}

impl Expression for Call {
    fn check_type(&self, type_interpreter: &mut Interpreter<Type>) -> Result<Type, String> {
        let target = self.target.check_type(type_interpreter)?;
        let argument_types = self
            .arguments
            .iter()
            .map(|arg| arg.check_type(type_interpreter))
            .collect::<Result<Vec<Type>, String>>()?;

        if let Type::Function(function_type) = target {
            return function_type.apply(argument_types);
        }

        print!("{:#?}", target);
        Err(String::from("Cannot call non function"))
    }

    fn interpret(&self, interpreter: &mut Interpreter<Value>) -> Result<Value, String> {
        if let Value::Function(callee) = self.target.interpret(interpreter)? {
            let mut arguments = Vec::new();

            for argument in self.arguments.iter() {
                let value = argument.interpret(interpreter)?;
                arguments.push(value);
            }

            return callee.borrow().clone().call(arguments);
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
