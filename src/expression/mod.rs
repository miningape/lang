use std::{collections::HashMap, ptr::null_mut};

use crate::{environment::Environment, value::Value};

pub mod assign;
pub mod binary;
pub mod call;
pub mod function;
pub mod literal;
pub mod unary;
pub mod variable;

pub struct Interpreter {
    pub environment: *mut Environment,
}

impl Interpreter {
    pub fn get(&mut self, key: String) -> Option<Value> {
        unsafe { self.environment.as_mut().unwrap().get(key) }
    }

    pub fn set(&mut self, key: String, value: Value) -> Option<Value> {
        unsafe { self.environment.as_mut().unwrap().set(key, value) }
    }

    pub fn push_environment(&mut self) {
        self.environment = &mut Environment {
            variables: HashMap::new(),
            parent: self.environment,
        } as *mut Environment
    }

    pub fn pop_environment(&mut self) -> Result<(), String> {
        unsafe {
            Ok(
                self.environment = match self.environment.as_mut().unwrap().pop() {
                    Some(environment) => environment,
                    None => return Err(String::from("Cannot pop base environment")),
                },
            )
        }
    }

    pub fn print_environment(&self) -> String {
        unsafe { self.environment.as_ref().unwrap().print() }
    }
}

impl Clone for Interpreter {
    fn clone(&self) -> Self {
        Interpreter {
            environment: self.environment,
        }
    }
}

pub trait Expression {
    fn interpret(&self, interpreter: &mut Interpreter) -> Result<Value, String>;
    fn to_string(&self) -> String;
}
