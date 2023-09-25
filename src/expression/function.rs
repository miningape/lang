use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{callable::Callable, environment::Environment, value::Value};

use super::{Expression, Interpreter};

pub struct Function {
    pub argument_names: Vec<String>,
    pub body: Rc<Vec<Box<dyn Expression>>>,
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
            self.body.iter().map(|e| e.to_string()).collect::<Vec<_>>()
        )
    }
}

pub struct FunctionInstance {
    pub argument_names: Vec<String>,
    pub body: Rc<Vec<Box<dyn Expression>>>,
    pub interpreter: Interpreter,
}

impl Callable for FunctionInstance {
    fn signature(&self) -> String {
        String::from("Function")
    }

    fn call(&mut self, arguments: Vec<crate::value::Value>) -> Result<Value, String> {
        if self.argument_names.len() != arguments.len() {
            return Err(format!("Arguments for function mismatch"));
        }

        self.interpreter.environment = &mut Environment {
            variables: HashMap::new(),
            parent: self.interpreter.environment,
        } as *mut Environment;
        for (index, argument_name) in self.argument_names.iter().enumerate() {
            self.interpreter
                .set(argument_name.to_string(), arguments[index].clone());
        }

        let mut last_result = Value::Number(-1.0);
        for expression in self.body.iter() {
            last_result = expression.interpret(&mut self.interpreter)?;
        }

        unsafe {
            self.interpreter.environment = self.interpreter.environment.as_mut().unwrap().parent;
        }

        Ok(last_result)
    }
}
