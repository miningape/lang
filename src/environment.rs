use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug, Clone)]
pub struct Variable<T> {
    pub mutable: bool,
    pub value: T,
}

pub struct Environment<T> {
    pub variables: HashMap<String, Variable<T>>,
    pub parent: Option<Rc<RefCell<Environment<T>>>>,
}

impl<T: std::clone::Clone + std::fmt::Debug> Environment<T> {
    pub fn new(parent: Option<&Rc<RefCell<Environment<T>>>>) -> Rc<RefCell<Environment<T>>> {
        Rc::new(RefCell::new(Environment {
            variables: HashMap::new(),
            parent: match parent {
                None => None,
                Some(rc) => Some(Rc::clone(rc)),
            },
        }))
    }

    pub fn pop(&self) -> Option<Rc<RefCell<Environment<T>>>> {
        match &self.parent {
            None => None,
            Some(rc) => Some(Rc::clone(&rc)),
        }
    }

    pub fn create(&mut self, key: String, value: Variable<T>) -> Result<Variable<T>, String> {
        if self.variables.contains_key(&key) {
            return Err(String::from("Cannot create variable that already exists"));
        }

        self.variables.insert(key.clone(), value.clone());

        Ok(value)
    }

    pub fn set(&mut self, key: String, value: T) -> Result<Variable<T>, String> {
        // Hack - should be done at compile time not runtime
        match self.get_with_depth(key.clone(), 0) {
            None => Err(format!(
                "Cannot set a variable ({}) that does not exist",
                key.clone()
            )),
            Some((variable, 0)) => {
                if !variable.mutable {
                    return Err(String::from("Cannot mutate const variable"));
                }

                let new_variable = Variable {
                    mutable: variable.mutable,
                    value,
                };

                self.variables.insert(key.clone(), new_variable.clone());
                Ok(new_variable)
            }
            Some((_, depth)) => self.set_at_depth(key.clone(), value, depth),
        }
    }

    fn set_at_depth(&mut self, key: String, value: T, depth: u16) -> Result<Variable<T>, String> {
        let mut i = 1;
        let mut env = self.pop();
        while i < depth {
            env = env.and_then(|e| e.borrow().pop());
            i += 1;
        }

        match env {
            None => Err(String::from("Cannot set in a parent that does not exist")),
            Some(e) => e.borrow_mut().set(key, value),
        }
    }

    fn get_with_depth(&self, key: String, depth: u16) -> Option<(Variable<T>, u16)> {
        match self.variables.get(&key).cloned() {
            None => match self.pop() {
                Some(env) => env.borrow().get_with_depth(key, depth + 1),
                None => None,
            },
            Some(some) => Some((some, depth)),
        }
    }

    pub fn get(&self, key: String) -> Option<Variable<T>> {
        match self.variables.get(&key).cloned() {
            None => match self.pop() {
                Some(env) => env.borrow().get(key),
                None => None,
            },
            some => some,
        }
    }

    pub fn print(&self) -> String {
        format!(
            "{{\n\t\"variables\": {:#?},\n\t\"parent\": {}\n}}",
            self.variables,
            match self.pop() {
                Some(env) => env.borrow().print(),
                None => "null".to_owned(),
            }
        )
    }
}

#[cfg(test)]
mod test {
    use crate::{environment::Variable, value::Value};

    use super::Environment;

    #[test]
    fn t() {
        let env = Environment::new(None);
        env.borrow_mut()
            .create(
                "ass".to_owned(),
                Variable {
                    mutable: true,
                    value: Value::Number(10.0),
                },
            )
            .unwrap();

        let new_env = Environment::new(Some(&env));
        new_env
            .borrow_mut()
            .create(
                "key".to_owned(),
                Variable {
                    mutable: false,
                    value: Value::String("()".to_owned()),
                },
            )
            .unwrap();

        let other_new_env = Environment::new(Some(&env));
        other_new_env
            .borrow_mut()
            .create(
                "jeff".to_owned(),
                Variable {
                    mutable: false,
                    value: Value::Number(20.0),
                },
            )
            .unwrap();

        env.borrow_mut()
            .set("ass".to_owned(), Value::Number(20.0))
            .unwrap();

        println!("{}", other_new_env.borrow().print())
    }
}
