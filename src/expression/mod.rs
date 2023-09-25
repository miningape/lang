use std::{cell::RefCell, rc::Rc};

use crate::{environment::Environment, value::Value};

pub mod assign;
pub mod binary;
pub mod call;
pub mod function;
pub mod literal;
pub mod unary;
pub mod variable;

pub struct Interpreter {
    pub environment: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            environment: Environment::new(None),
        }
    }

    pub fn get(&self, key: String) -> Option<Value> {
        self.environment.borrow().get(key)
    }

    pub fn set(&self, key: String, value: Value) -> Option<Value> {
        self.environment.borrow_mut().set(key, value)
    }

    pub fn push_environment(&mut self) {
        self.environment = Environment::new(Some(&self.environment))
    }

    pub fn pop_environment(&mut self) -> Result<(), String> {
        self.environment = match Rc::clone(&self.environment).borrow().pop() {
            Some(environment) => environment,
            None => return Err(String::from("Cannot pop base environment")),
        };

        Ok(())
    }

    pub fn print_environment(&self) -> String {
        self.environment.borrow().print()
    }
}

impl Clone for Interpreter {
    fn clone(&self) -> Self {
        Interpreter {
            environment: Rc::clone(&self.environment),
        }
    }
}

pub trait Expression {
    fn interpret(&self, interpreter: &mut Interpreter) -> Result<Value, String>;
    fn to_string(&self) -> String;
}
