use std::{cell::RefCell, collections::HashMap, fs, ptr::null_mut, rc::Rc};

use callable::print::Print;
use environment::Environment;
use expression::Interpreter;

pub mod callable;
pub mod environment;
pub mod expression;
pub mod parser;
pub mod tokeniser;
pub mod value;

fn main() {
    let source = fs::read_to_string("text.aa").unwrap();
    let tokens = tokeniser::scan(source);
    let mut interpreter = Interpreter::new();

    interpreter.set(
        "print".to_owned(),
        value::Value::Function(Rc::new(RefCell::new(Print {}))),
    );

    match tokens {
        Err(err) => panic!("An error occured while scanning:\n-\t{}", err),
        Ok(vec) => {
            println!("{:#?}", vec);
            let expressions = parser::parse(vec).unwrap();

            for expression in expressions.iter() {
                println!(
                    "--- OUTPUT ---\ntree:\n {}\nresult: {:#?}\nenvironment: {}",
                    expression.to_string(),
                    expression.interpret(&mut interpreter).unwrap(),
                    interpreter.print_environment(),
                );
            }

            // match expressions {
            //     Err(err) => panic!("An error occured while parsing:\n-\t{}", err),
            //     Ok(exprs) => print!("{:#?}", exprs)
            // }
        }
    }
}
