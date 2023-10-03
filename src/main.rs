use std::{
    env, fs,
    io::{self, stdout, BufRead, Write},
};

use expression::Interpreter;

use crate::value::Value;

pub mod callable;
pub mod environment;
pub mod expression;
pub mod parser;
pub mod tokeniser;
pub mod value;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut interpreter = Interpreter::new();

    if args.len() > 1 {
        let filepath = &args[1];
        return interpret_file(filepath, &mut interpreter);
    }

    repl(&mut interpreter);
}

fn repl(interpreter: &mut Interpreter) {
    let stdin = io::stdin();

    print!("> ");
    stdout().flush().unwrap();
    for line in stdin.lines() {
        let source = &line.unwrap();

        let tokens = tokeniser::scan(source).unwrap();
        let expressions = parser::parse(tokens).unwrap();

        let mut last_value = Value::Null;
        for expression in expressions.iter() {
            last_value = expression.interpret(interpreter).unwrap();
        }

        print!("{}\n> ", last_value.to_log_string());
        stdout().flush().unwrap();
    }
}

fn interpret_file(filepath: &String, interpreter: &mut Interpreter) {
    let source = &fs::read_to_string(filepath).unwrap();
    let tokens = tokeniser::scan(source);

    match tokens {
        Err(err) => panic!("An error occured while scanning:\n-\t{}", err),
        Ok(vec) => {
            // println!("{:#?}", vec);
            let expressions = parser::parse(vec).unwrap();

            for expression in expressions.iter() {
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
