use core::fmt::Debug;
use std::{cell::RefCell, rc::Rc};

use crate::callable::Callable;

trait ApplyType: Debug + Clone {
    fn apply(&self, argument_types: Vec<Type>) -> bool;
}

#[derive(Debug, Clone)]
pub enum FunctionType {
    Literal(Vec<Type>, Type),
    ArrayArgs(Type, Type),
    WithBody(Rc<RefCell<dyn Callable>>),
}

impl FunctionType {
    pub fn apply(&self, argument_types: Vec<Type>) -> Result<Type, String> {
        match self {
            Self::WithBody(function_instance) => function_instance
                .borrow()
                .clone()
                .get_type()?
                .apply(argument_types),
            Self::Literal(expected_argument_types, expected_return_type) => {
                if argument_types.len() != expected_argument_types.len() {
                    return Err(String::from("Argument supplied length doesn't match"));
                }

                for (i, argument_type) in argument_types.iter().enumerate() {
                    let expected_argument_type = &expected_argument_types[i];

                    if !argument_type.is_sub_type_of(expected_argument_type) {
                        return Err(String::from("Argument supplied type doesn't match"));
                    }
                }

                Ok(expected_return_type.clone())
            }
            Self::ArrayArgs(expected_array_argument_type, return_type) => {
                for argument_type in argument_types.iter() {
                    if !argument_type.is_sub_type_of(expected_array_argument_type) {
                        return Err(String::from("Argument list doesn't match contract"));
                    }
                }

                return Ok(return_type.clone());
            }
        }
    }

    pub fn is_sub_type_of(&self, other: &FunctionType) -> bool {
        if let FunctionType::WithBody(function_instance) = self {
            return match function_instance.borrow().clone().get_type() {
                Err(_) => false,
                Ok(type_) => type_.is_sub_type_of(other),
            };
        }

        if let FunctionType::WithBody(function_instance) = other {
            return match function_instance.borrow().clone().get_type() {
                Err(_) => false,
                Ok(type_) => type_.is_sub_type_of(other),
            };
        }

        if let FunctionType::Literal(argument_types, return_type) = self {
            if let FunctionType::Literal(other_argument_types, other_return_type) = other {
                if argument_types.len() != other_argument_types.len() {
                    return false;
                }

                for (i, argument_type) in argument_types.iter().enumerate() {
                    let other_argument_type = &other_argument_types[i];

                    if !argument_type.is_sub_type_of(other_argument_type) {
                        return false;
                    }
                }

                return return_type.is_sub_type_of(other_return_type);
            }
        }

        if let FunctionType::ArrayArgs(argument_array_type, return_type) = self {
            if let FunctionType::ArrayArgs(other_argument_array_type, other_return_type) = other {
                if !argument_array_type.is_sub_type_of(other_argument_array_type) {
                    return false;
                }

                return return_type.is_sub_type_of(other_return_type);
            }
        }

        return false;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BaseType {
    Infer,
    Any,
    Null,
    String,
    Number,
    Boolean,
}

#[derive(Debug, Clone)]
pub enum Type {
    BaseType(BaseType),
    Or(Box<Type>, Box<Type>),
    Function(Box<FunctionType>),
    Return(Box<Type>),
}

impl Type {
    pub fn get_return_type(&self) -> Option<Type> {
        match self {
            Self::BaseType(_) => None,
            Self::Function(_) => None,
            Self::Return(return_type) => Some((**return_type).clone()),
            Self::Or(left, right) => match left.get_return_type() {
                None => right.get_return_type(),
                Some(return_type) => match right.get_return_type() {
                    None => Some((return_type).clone()),
                    Some(right_return_type) => Some(Type::Or(
                        Box::from(return_type),
                        Box::from(right_return_type),
                    )),
                },
            },
        }
    }

    pub fn is_sub_type_of(&self, other: &Type) -> bool {
        if let Type::BaseType(BaseType::Any) = other {
            return true;
        }

        if let Type::BaseType(BaseType::Any) = self {
            return false;
        }

        if let Type::Or(left, right) = other {
            if let Type::Or(self_left, self_right) = self {
                return (self_left.is_sub_type_of(left) || self_left.is_sub_type_of(right))
                    && (self_right.is_sub_type_of(left) || self_right.is_sub_type_of(right));
            }

            return self.is_sub_type_of(left) || self.is_sub_type_of(right);
        }

        if let Type::Or(left, right) = self {
            return left.is_sub_type_of(other) && right.is_sub_type_of(other);
        }

        if let Type::Function(function_type) = self {
            if let Type::Function(other_function_type) = other {
                return function_type.is_sub_type_of(other_function_type);
            }

            return false;
        }

        if let Type::BaseType(base_type) = self {
            if let Type::BaseType(other_base_type) = other {
                return base_type == other_base_type;
            }

            return false;
        }

        return false;
    }
}
