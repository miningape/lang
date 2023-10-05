use std::{
    env, fs,
    io::{self, stdout, Write},
};

use expression::Interpreter;

use crate::{types::Type, value::Value};

pub mod callable;
pub mod environment;
pub mod expression;
pub mod parser;
pub mod tokeniser;
pub mod types;
pub mod value;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut interpreter = Interpreter::<Value>::new();
    interpreter.seed();

    if args.len() > 1 {
        let filepath = &args[1];
        return interpret_file(filepath, &mut interpreter);
    }

    repl(&mut interpreter);
}

fn repl(interpreter: &mut Interpreter<Value>) {
    let stdin = io::stdin();
    let mut type_checker = Interpreter::<Type>::new();
    type_checker.seed();

    print!("> ");
    stdout().flush().unwrap();
    for line in stdin.lines() {
        let source = &line.unwrap();

        let tokens = tokeniser::scan(source).unwrap();
        let expressions = parser::parse(tokens).unwrap();

        let mut last_value = Value::Null;
        for expression in expressions.iter() {
            println!(
                "Type - {:#?}",
                expression.check_type(&mut type_checker).unwrap()
            );
            last_value = expression.interpret(interpreter).unwrap();
        }

        print!("{}\n> ", last_value.to_log_string());
        stdout().flush().unwrap();
    }
}

fn interpret_file(filepath: &String, interpreter: &mut Interpreter<Value>) {
    let source = &fs::read_to_string(filepath).unwrap();
    let tokens = tokeniser::scan(source);

    let mut type_checker = Interpreter::<Type>::new();
    type_checker.seed();

    match tokens {
        Err(err) => panic!("An error occured while scanning:\n-\t{}", err),
        Ok(vec) => {
            // println!("{:#?}", vec);
            let expressions = parser::parse(vec).unwrap();

            for expression in expressions.iter() {
                expression.check_type(&mut type_checker).unwrap();
                expression.interpret(interpreter).unwrap();
                // println!(
                //     "--- OUTPUT ---\ntree:\n {}\nresult: {:#?}\nenvironment: {}\n",
                //     expression.to_string(),
                //     expression.interpret(&mut interpreter).unwrap(),
                //     interpreter.print_environment(),
                // );
            }

            println!("")
        }
    }
}
