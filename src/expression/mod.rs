use std::{cell::RefCell, rc::Rc};

use crate::{
    callable::{map::Map, print::Print},
    environment::{Environment, Variable},
    types::{FunctionType, Type},
    value::Value,
};

pub mod assign;
pub mod binary;
pub mod body;
pub mod call;
pub mod declare;
pub mod function;
pub mod if_expression;
pub mod list;
pub mod literal;
pub mod return_expression;
pub mod unary;
pub mod variable;

pub struct Interpreter<T> {
    pub environment: Rc<RefCell<Environment<T>>>,
}

impl<T: std::clone::Clone + std::fmt::Debug> Interpreter<T> {
    pub fn new() -> Interpreter<T> {
        return Interpreter {
            environment: Environment::new(None),
        };
    }

    pub fn create(&self, key: String, value: Variable<T>) -> Result<Variable<T>, String> {
        self.environment.borrow_mut().create(key, value)
    }

    pub fn get(&self, key: String) -> Option<Variable<T>> {
        self.environment.borrow().get(key)
    }

    pub fn set(&self, key: String, value: T) -> Result<Variable<T>, String> {
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

impl<T> Clone for Interpreter<T> {
    fn clone(&self) -> Self {
        Interpreter {
            environment: Rc::clone(&self.environment),
        }
    }
}

impl Interpreter<Value> {
    pub fn seed(&mut self) {
        self.create(
            "print".to_owned(),
            Variable {
                mutable: false,
                value: Value::Function(Rc::new(RefCell::new(Print {}))),
            },
        )
        .unwrap();

        self.create(
            "map".to_owned(),
            Variable {
                mutable: false,
                value: Value::Function(Rc::new(RefCell::new(Map {}))),
            },
        )
        .unwrap();
    }
}

impl Interpreter<Type> {
    pub fn seed(&mut self) {
        self.create(
            "print".to_owned(),
            Variable {
                mutable: false,
                value: Type::Function(Box::from(FunctionType::WithBody(Rc::from(RefCell::from(
                    Print {},
                ))))),
            },
        )
        .unwrap();

        self.create(
            "map".to_owned(),
            Variable {
                mutable: false,
                value: Type::Function(Box::from(FunctionType::WithBody(Rc::from(RefCell::from(
                    Map {},
                ))))),
            },
        )
        .unwrap();
    }
}

pub trait Expression {
    fn interpret(&self, interpreter: &mut Interpreter<Value>) -> Result<Value, String>;
    fn check_type(&self, type_interpreter: &mut Interpreter<Type>) -> Result<Type, String>;
    fn to_string(&self) -> String;
}
