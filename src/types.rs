use core::fmt::Debug;

use crate::expression::{function::Function, Interpreter};

trait ApplyType: Debug + Clone {
    fn apply(&self, argument_types: Vec<Type>) -> bool;
}

#[derive(Debug, Clone)]
pub enum FunctionType {
    Literal(Vec<Type>, Type),
    Unrefined(Vec<Type>, Type, Box<Function>),
    ArrayArgs(Type, Type),
}

impl FunctionType {
    pub fn check_body_and_refine_type(
        &self,
        type_checker: &mut Interpreter<Type>,
    ) -> Result<Type, String> {
        match self {
            Self::Unrefined(expected_argument_types, expected_return_type, function) => {
                type_checker.push_environment();
                for function_argument in function.arguments.iter() {
                    type_checker.set(
                        function_argument.name.clone(),
                        function_argument.type_annotation.clone(),
                    );
                }

                let return_type = function.body.check_type(type_checker)?;
                type_checker.pop_environment()?;

                print!("{:#?} - {:#?}", return_type, expected_return_type);

                if let Type::BaseType(BaseType::Any) = expected_return_type {
                    return Ok(Type::Function(Box::from(FunctionType::Literal(
                        expected_argument_types.clone(),
                        return_type,
                    ))));
                }

                if !return_type.is_sub_type_of(expected_return_type) {
                    return Err(format!(
                        "Actual return type ({:#?}) does not match the return type of the body ({:#?})",
                        return_type,
                        expected_return_type
                    ));
                }

                Ok(Type::Function(Box::from(FunctionType::Literal(
                    expected_argument_types.clone(),
                    expected_return_type.clone(),
                ))))
            }
            literal => Ok(Type::Function(Box::from(literal.clone()))),
        }
    }

    pub fn apply(&self, argument_types: Vec<Type>) -> Result<Type, String> {
        match self {
            Self::Unrefined(expected_argument_types, expected_return_type, _) => {
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

            if let FunctionType::Unrefined(other_argument_types, other_return_type, _) = other {
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

        if let FunctionType::Unrefined(argument_types, return_type, _) = self {
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

            if let FunctionType::Unrefined(other_argument_types, other_return_type, _) = other {
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
}

impl Type {
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
        }

        return false;
    }
}
