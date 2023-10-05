use std::{cell::RefCell, fmt::Debug, rc::Rc};

use crate::{
    callable::Callable,
    types::{FunctionType, Type},
    value::Value,
};

use super::{Expression, Interpreter};

#[derive(Clone)]
pub struct FunctionArgument {
    pub name: String,
    pub type_annotation: Type,
}

pub struct Function {
    pub arguments: Vec<FunctionArgument>,
    pub return_type: Type,
    pub body: Rc<Box<dyn Expression>>,
}

impl Debug for Function {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

impl Clone for Function {
    fn clone(&self) -> Self {
        Function {
            arguments: self.arguments.clone(),
            return_type: self.return_type.clone(),
            body: Rc::clone(&self.body),
        }
    }
}

impl Expression for Function {
    fn check_type(&self, _: &mut Interpreter<Type>) -> Result<Type, String> {
        let argument_types: Vec<Type> = self
            .arguments
            .iter()
            .map(|arg| arg.type_annotation.clone())
            .collect();

        Ok(Type::Function(Box::from(FunctionType::Unrefined(
            argument_types,
            self.return_type.clone(),
            Box::from(self.clone()),
        ))))
    }

    fn interpret(&self, interpreter: &mut Interpreter<Value>) -> Result<Value, String> {
        Ok(Value::Function(Rc::new(RefCell::new(FunctionInstance {
            argument_names: self.arguments.iter().map(|arg| arg.name.clone()).collect(),
            body: Rc::clone(&self.body),
            interpreter: interpreter.clone(),
        }))))
    }

    fn to_string(&self) -> String {
        format!(
            "{{ \"type\": \"Function\", \"argument_names\": {:#?}, \"argument_types\": {:#?}, \"body\": {:#?} }}",
            self.arguments.iter().map(|arg| arg.name.clone()).collect::<Vec<String>>(),
            self.arguments.iter().map(|arg| arg.type_annotation.clone()).collect::<Vec<Type>>(),
            self.body.to_string()
        )
    }
}

pub struct FunctionInstance {
    pub argument_names: Vec<String>,
    pub body: Rc<Box<dyn Expression>>,
    pub interpreter: Interpreter<Value>,
}

impl Callable for FunctionInstance {
    fn signature(&self) -> String {
        String::from("Function")
    }

    fn clone(&self) -> Box<dyn Callable> {
        return Box::new(FunctionInstance {
            argument_names: self.argument_names.clone(),
            body: Rc::clone(&self.body),
            interpreter: self.interpreter.clone(),
        });
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
