use std::{cell::RefCell, rc::Rc};

use crate::callable::Callable;

#[derive(Clone)]
pub enum Value {
    String(String),
    Number(f32),
    Function(Rc<RefCell<dyn Callable>>),
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = self.to_log_string();
        f.write_str(out.as_str())
    }
}

fn string_add<L: std::fmt::Display, R: std::fmt::Display>(left: L, right: R) -> String {
    format!("{}{}", left, right)
}

impl Value {
    pub fn to_string(&self) -> String {
        match self {
            Value::Number(number) => number.to_string(),
            Value::String(string) => format!("{}", string),
            Value::Function(function) => format!("{}", function.borrow().signature()),
        }
    }

    pub fn to_log_string(&self) -> String {
        match self {
            Value::Number(number) => number.to_string(),
            Value::String(string) => format!("\"{}\"", string),
            Value::Function(function) => format!("{}", function.borrow().signature()),
        }
    }

    pub fn add(self, right: Value) -> Result<Value, String> {
        Ok(match self {
            Value::Number(left_num) => match right {
                Value::Number(right_num) => Value::Number(left_num + right_num),
                Value::String(right_string) => Value::String(string_add(left_num, right_string)),
                Value::Function(right_function) => {
                    Value::String(string_add(left_num, right_function.borrow().signature()))
                }
            },
            Value::String(left_string) => match right {
                Value::Number(right_num) => Value::String(string_add(left_string, right_num)),
                Value::String(right_string) => Value::String(string_add(left_string, right_string)),
                Value::Function(right_function) => {
                    Value::String(string_add(left_string, right_function.borrow().signature()))
                }
            },
            Value::Function(left_function) => match right {
                Value::Number(right_num) => {
                    Value::String(string_add(left_function.borrow().signature(), right_num))
                }
                Value::String(right_string) => {
                    Value::String(string_add(left_function.borrow().signature(), right_string))
                }
                Value::Function(right_function) => Value::String(string_add(
                    left_function.borrow().signature(),
                    right_function.borrow().signature(),
                )),
            },
        })
    }

    pub fn sub(self, right: Value) -> Result<Value, String> {
        if let Value::Number(left_number) = self {
            if let Value::Number(right_number) = right {
                return Ok(Value::Number(left_number - right_number));
            }
        }

        return Err("Cannot subtract non number values".to_owned());
    }
}
