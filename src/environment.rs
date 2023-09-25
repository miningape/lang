use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::value::Value;

pub struct Environment {
    pub variables: HashMap<String, Value>,
    pub parent: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new(parent: Option<&Rc<RefCell<Environment>>>) -> Rc<RefCell<Environment>> {
        Rc::new(RefCell::new(Environment {
            variables: HashMap::new(),
            parent: match parent {
                None => None,
                Some(rc) => Some(Rc::clone(rc)),
            },
        }))
    }

    pub fn pop(&self) -> Option<Rc<RefCell<Environment>>> {
        match &self.parent {
            None => None,
            Some(rc) => Some(Rc::clone(&rc)),
        }
    }

    pub fn set(&mut self, key: String, value: Value) -> Option<Value> {
        // Hack - should be done at compile time not runtime
        match self.get_with_depth(key.clone(), 0) {
            None => self.variables.insert(key.clone(), value),
            Some((_, 0)) => self.variables.insert(key.clone(), value),
            Some((_, depth)) => self.set_at_depth(key.clone(), value, depth),
        }
    }

    fn set_at_depth(&mut self, key: String, value: Value, depth: u16) -> Option<Value> {
        let mut i = 1;
        let mut env = self.pop();
        while i < depth {
            env = env.and_then(|e| e.borrow().pop());
            i += 1;
        }

        match env {
            None => None,
            Some(e) => e.borrow_mut().set(key, value),
        }
    }

    fn get_with_depth(&self, key: String, depth: u16) -> Option<(Value, u16)> {
        match self.variables.get(&key).cloned() {
            None => match self.pop() {
                Some(env) => env.borrow().get_with_depth(key, depth + 1),
                None => None,
            },
            Some(some) => Some((some, depth)),
        }
    }

    pub fn get(&self, key: String) -> Option<Value> {
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
    use crate::value::Value;

    use super::Environment;

    #[test]
    fn t() {
        let env = Environment::new(None);
        env.borrow_mut().set("ass".to_owned(), Value::Number(10.0));

        let new_env = Environment::new(Some(&env));
        new_env
            .borrow_mut()
            .set("key".to_owned(), Value::String("()".to_owned()));

        let other_new_env = Environment::new(Some(&env));
        other_new_env
            .borrow_mut()
            .set("jeff".to_owned(), Value::Number(20.0));

        env.borrow_mut().set("ass".to_owned(), Value::Number(20.0));

        println!("{}", other_new_env.borrow().print())
    }
}
