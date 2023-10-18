use std::{cell::RefCell, rc::Rc};

use crate::callable::Callable;

#[derive(Clone)]
pub enum Value {
    Null,
    String(String),
    Number(f32),
    Boolean(bool),
    Return(Box<Value>),
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
            Value::Return(_) => panic!("Cannot stringify return value"),
            Value::Null => String::from("null"),
            Value::Number(number) => number.to_string(),
            Value::String(string) => format!("{}", string),
            Value::Boolean(boolean) => format!(
                "{}",
                match boolean {
                    true => "true",
                    _ => "false",
                }
            ),
            Value::Function(function) => format!("(fn:{})", function.borrow().signature()),
        }
    }

    pub fn to_log_string(&self) -> String {
        match self {
            Value::Return(_) => panic!("Cannot stringify return value"),
            Value::Null => String::from("null"),
            Value::Number(number) => number.to_string(),
            Value::String(string) => format!("\"{}\"", string),
            Value::Boolean(boolean) => format!(
                "{}",
                match boolean {
                    true => "true",
                    _ => "false",
                }
            ),
            Value::Function(function) => format!("Function: \"{}\"", function.borrow().signature()),
        }
    }

    pub fn not(self) -> Result<Value, String> {
        if let Value::Boolean(boolean) = self {
            return Ok(Value::Boolean(!boolean));
        }

        return Err("Cannot negate non bool".to_owned());
    }

    pub fn add(self, right: Value) -> Result<Value, String> {
        Ok(match self {
            Value::Number(left_num) => match right {
                Value::Number(right_num) => Value::Number(left_num + right_num),
                value => Value::String(string_add(left_num.to_string(), value.to_string())),
            },
            value => Value::String(string_add(value.to_string(), right.to_string())),
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

    pub fn mul(self, right: Value) -> Result<Value, String> {
        if let Value::Number(left_number) = self {
            if let Value::Number(right_number) = right {
                return Ok(Value::Number(left_number * right_number));
            }
        }

        return Err("Cannot subtract non number values".to_owned());
    }

    pub fn div(self, right: Value) -> Result<Value, String> {
        if let Value::Number(left_number) = self {
            if let Value::Number(right_number) = right {
                return Ok(Value::Number(left_number / right_number));
            }
        }

        return Err("Cannot subtract non number values".to_owned());
    }

    pub fn equals(self, right: Value) -> Result<Value, String> {
        if let Value::Number(left_number) = self {
            if let Value::Number(right_number) = right {
                return Ok(Value::Boolean(left_number == right_number));
            }
        }

        if let Value::Boolean(left_bool) = self {
            if let Value::Boolean(right_bool) = right {
                return Ok(Value::Boolean(left_bool == right_bool));
            }
        }

        if let Value::String(left_string) = self.clone() {
            if let Value::String(right_string) = right {
                return Ok(Value::Boolean(left_string == right_string));
            }
        }

        return Ok(Value::Boolean(false));
    }

    pub fn greater(self, right: Value) -> Result<Value, String> {
        if let Value::Number(left_number) = self {
            if let Value::Number(right_number) = right {
                return Ok(Value::Boolean(left_number > right_number));
            }
        }

        return Err("Cannot compare (>) non number values".to_owned());
    }

    pub fn lesser(self, right: Value) -> Result<Value, String> {
        if let Value::Number(left_number) = self {
            if let Value::Number(right_number) = right {
                return Ok(Value::Boolean(left_number < right_number));
            }
        }

        return Err("Cannot compare (<) non number values".to_owned());
    }

    pub fn and(self, right: Value) -> Result<Value, String> {
        if let Value::Boolean(left_boolean) = self {
            if let Value::Boolean(right_boolean) = right {
                return Ok(Value::Boolean(left_boolean && right_boolean));
            }
        }

        return Err("Cannot and non boolean values".to_owned());
    }

    pub fn or(self, right: Value) -> Result<Value, String> {
        if let Value::Boolean(left_boolean) = self {
            if let Value::Boolean(right_boolean) = right {
                return Ok(Value::Boolean(left_boolean || right_boolean));
            }
        }

        return Err("Cannot or non boolean values".to_owned());
    }
}
