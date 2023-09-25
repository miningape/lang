use std::{collections::HashMap, ptr::null_mut};

use crate::value::Value;

pub struct Environment {
    pub variables: HashMap<String, Value>,
    pub parent: *mut Environment,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            variables: HashMap::new(),
            parent: null_mut(),
        }
    }

    pub fn push(&mut self) -> Environment {
        Environment {
            variables: HashMap::new(),
            parent: self,
        }
    }

    pub fn pop(&self) -> Option<&mut Environment> {
        unsafe { self.parent.as_mut() }
    }

    pub fn set(&mut self, key: String, value: Value) -> Option<Value> {
        self.variables.insert(key, value)
    }

    pub fn get(&mut self, key: String) -> Option<Value> {
        match self.variables.get(&key).cloned() {
            None => match self.pop() {
                Some(env) => env.get(key),
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
                Some(env) => env.print(),
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
        let mut env = Environment::new();
        env.set("ass".to_owned(), Value::Number(10.0));

        let mut new_env = env.push();
        new_env.set("key".to_owned(), Value::String("()".to_owned()));

        let mut other_new_env = env.push();
        other_new_env.set("jeff".to_owned(), Value::Number(20.0));

        env.set("ass".to_owned(), Value::Number(20.0));

        println!("{}", other_new_env.print())
    }

    #[test]
    fn t2() {
        unsafe {
            let env = &mut Environment::new() as *mut Environment;
            env.as_mut()
                .unwrap()
                .set("ass".to_owned(), Value::Number(10.0));

            let mut new_env = env.as_mut().unwrap().push();
            new_env.set("key".to_owned(), Value::String("()".to_owned()));

            let mut other_new_env = env.as_mut().unwrap().push();
            other_new_env.set("jeff".to_owned(), Value::Number(20.0));

            env.as_mut()
                .unwrap()
                .set("ass".to_owned(), Value::Number(20.0));

            println!("{}", other_new_env.print())
        }
    }
}
