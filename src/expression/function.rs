use std::{cell::RefCell, fmt::Debug, rc::Rc};

use crate::{
    callable::Callable,
    types::{BaseType, FunctionType, Type},
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
    fn check_type(&self, type_checker: &mut Interpreter<Type>) -> Result<Type, String> {
        Ok(Type::Function(Box::from(FunctionType::WithBody(Rc::from(
            RefCell::from(FunctionInstance {
                arguments: self.arguments.clone(),
                return_type: self.return_type.clone(),
                actual_type: Rc::from(RefCell::from(None)),
                body: Rc::clone(&self.body),
                interpreter: type_checker.clone(),
            }),
        )))))
    }

    fn interpret(&self, interpreter: &mut Interpreter<Value>) -> Result<Value, String> {
        Ok(Value::Function(Rc::new(RefCell::new(FunctionInstance {
            arguments: self.arguments.clone(), //self.arguments.iter().map(|arg| arg.name.clone()).collect(),
            return_type: self.return_type.clone(),
            actual_type: Rc::from(RefCell::from(None)),
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

pub struct FunctionInstance<T> {
    pub arguments: Vec<FunctionArgument>,
    pub return_type: Type,
    pub actual_type: Rc<RefCell<Option<FunctionType>>>,

    pub body: Rc<Box<dyn Expression>>,
    pub interpreter: Interpreter<T>,
}

impl<T: Debug> Debug for FunctionInstance<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("Function {:#?}", self.body.to_string()).as_str())
    }
}

impl Callable for FunctionInstance<Type> {
    fn signature(&self) -> String {
        String::from("Function")
    }

    fn get_type(&mut self) -> Result<FunctionType, String> {
        if let Some(function_type) = self.actual_type.borrow().clone() {
            return Ok(function_type);
        }

        let argument_types: Vec<Type> = self
            .arguments
            .iter()
            .map(|arg| arg.type_annotation.clone())
            .collect();

        *self.actual_type.borrow_mut() = Some(FunctionType::Literal(
            argument_types.clone(),
            self.return_type.clone(),
        ));

        self.interpreter.push_environment();
        for function_argument in self.arguments.iter() {
            self.interpreter.create(
                function_argument.name.clone(),
                crate::environment::Variable {
                    mutable: false,
                    value: function_argument.type_annotation.clone(),
                },
            )?;
        }

        let return_type = self.body.check_type(&mut self.interpreter)?;
        self.interpreter.pop_environment()?;

        if let Type::BaseType(BaseType::Infer) = self.return_type {
            self.return_type = return_type;
        } else if !return_type.is_sub_type_of(&self.return_type) {
            return Err(format!(
                "Actual return type ({:#?}) does not match the return type of the body ({:#?})",
                return_type, self.return_type
            ));
        }

        *self.actual_type.borrow_mut() = Some(FunctionType::Literal(
            argument_types,
            self.return_type.clone(),
        ));

        self.get_type()
    }

    fn clone(&self) -> Box<dyn Callable> {
        return Box::new(FunctionInstance {
            arguments: self.arguments.clone(),
            return_type: self.return_type.clone(),
            actual_type: Rc::clone(&self.actual_type),
            body: Rc::clone(&self.body),
            interpreter: self.interpreter.clone(),
        });
    }

    fn call(&mut self, _: Vec<crate::value::Value>) -> Result<Value, String> {
        panic!("Cannot compute value in type environment")
    }
}

impl Callable for FunctionInstance<Value> {
    fn signature(&self) -> String {
        String::from("Function")
    }

    fn get_type(&mut self) -> Result<FunctionType, String> {
        panic!("Cannot get type of function instance");
    }

    fn clone(&self) -> Box<dyn Callable> {
        return Box::new(FunctionInstance {
            arguments: self.arguments.clone(),
            return_type: self.return_type.clone(),
            actual_type: self.actual_type.clone(),
            body: Rc::clone(&self.body),
            interpreter: self.interpreter.clone(),
        });
    }

    fn call(&mut self, arguments: Vec<crate::value::Value>) -> Result<Value, String> {
        if self.arguments.len() != arguments.len() {
            return Err(format!("Arguments for function mismatch"));
        }

        self.interpreter.push_environment();
        for (index, argument) in self.arguments.iter().enumerate() {
            self.interpreter.create(
                argument.name.to_string(),
                crate::environment::Variable {
                    mutable: false,
                    value: arguments[index].clone(),
                },
            )?;
        }

        let result = self.body.interpret(&mut self.interpreter);
        self.interpreter.pop_environment()?;

        result
    }
}
