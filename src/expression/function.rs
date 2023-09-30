use std::{cell::RefCell, rc::Rc};

use crate::{callable::Callable, value::Value};

use super::{Expression, Interpreter};

pub struct Function {
    pub argument_names: Vec<String>,
    pub body: Rc<Box<dyn Expression>>,
}

impl Expression for Function {
    fn interpret(&self, interpreter: &mut Interpreter) -> Result<Value, String> {
        Ok(Value::Function(Rc::new(RefCell::new(FunctionInstance {
            argument_names: self.argument_names.clone(),
            body: Rc::clone(&self.body),
            interpreter: interpreter.clone(),
        }))))
    }

    fn to_string(&self) -> String {
        format!(
            "{{ \"type\": \"Function\", \"argument_names\": {:#?}, \"body\": {:#?} }}",
            self.argument_names,
            self.body.to_string()
        )
    }
}

pub struct FunctionInstance {
    pub argument_names: Vec<String>,
    pub body: Rc<Box<dyn Expression>>,
    pub interpreter: Interpreter,
}

impl Callable for FunctionInstance {
    fn signature(&self) -> String {
        String::from("Function")
    }

    fn clone(&self) -> Rc<RefCell<dyn Callable>> {
        return Rc::new(RefCell::new(FunctionInstance {
            argument_names: self.argument_names.clone(),
            body: Rc::clone(&self.body),
            interpreter: self.interpreter.clone(),
        }));
    }

    fn call(&mut self, arguments: Vec<crate::value::Value>) -> Result<Value, String> {
        if self.argument_names.len() != arguments.len() {
            return Err(format!("Arguments for function mismatch"));
        }

        self.interpreter.push_environment();
        for (index, argument_name) in self.argument_names.iter().enumerate() {
            self.interpreter
                .set(argument_name.to_string(), arguments[index].clone());
        }

        let result = self.body.interpret(&mut self.interpreter);
        self.interpreter.pop_environment()?;

        result
    }
}
