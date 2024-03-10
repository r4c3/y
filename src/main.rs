mod lexer;
mod token;

use crate::lexer::Lexer;

use std::{
    env, fs,
    io::{self, BufRead, Write},
};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut lox = Lox::new();

    if args.len() > 2 {
        println!("Usage: rlox <script>");
        std::process::exit(64);
    }

    if args.len() == 2 {
        lox.run_file(&args[1])?;
    } else {
        lox.run_prompt()?;
    }

    if lox.had_error {
        std::process::exit(65);
    }

    Ok(())
}

struct Lox {
    had_error: bool,
}

impl Lox {
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
                for token in tokens {
                    println!("{}", token);
                }
            }
            Err(e) => {
                eprintln!("{:?}", e);
                self.had_error = true;
            }
        }
    }
}
