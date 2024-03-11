mod ast;
mod environment;
mod error;
mod interpreter;
mod lexer;
mod parser;
mod token;
mod value;

use crate::environment::Environment;
use crate::{lexer::Lexer, parser::Parser};
use std::{cell::RefCell, rc::Rc};

use std::{
    env, fs,
    io::{self, BufRead, Write},
};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut y = Y::new();

    if args.len() > 2 {
        println!("Usage: y <script>");
        std::process::exit(64);
    }

    if args.len() == 2 {
        y.run_file(&args[1])?;
    } else {
        y.run_prompt()?;
    }

    if y.had_error {
        std::process::exit(65);
    }

    Ok(())
}

struct Y {
    had_error: bool,
}

impl Y {
    fn new() -> Self {
        Self { had_error: false }
    }

    fn run_file(&mut self, path: &String) -> io::Result<()> {
        let source = fs::read_to_string(path)?;
        self.run(&source);
        if self.had_error {
            std::process::exit(65);
        }

        Ok(())
    }

    fn run_prompt(&mut self) -> io::Result<()> {
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        loop {
            print!("> ");
            io::stdout().flush()?;
            let mut buffer = String::new();
            handle.read_line(&mut buffer)?;
            if buffer.trim().is_empty() {
                break;
            }
            self.run(&buffer);
            self.had_error = false;
        }

        Ok(())
    }

    fn run(&mut self, source: &str) {
        let mut lexer = Lexer::new(source);
        match lexer.scan_tokens() {
            Ok(tokens) => {
                let mut parser = Parser::new(&tokens);
                match parser.parse() {
                    Ok(statements) => {
                        if !self.had_error {
                            let global_env = Rc::new(RefCell::new(Environment::new()));

                            for statement in statements {
                                match statement.execute(global_env.clone()) {
                                    Ok(_) => {}
                                    Err(runtime_error) => {
                                        eprintln!("{:?}", runtime_error);
                                        self.had_error = true;
                                    }
                                }
                            }
                        }
                    }
                    Err(parser_error) => {
                        eprintln!("{:?}", parser_error);
                        self.had_error = true;
                    }
                }
            }
            Err(lexer_error) => {
                eprintln!("{:?}", lexer_error);
                self.had_error = true;
            }
        }
    }
}
