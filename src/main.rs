use std::{
    env, fs,
    io::{self, BufRead},
};

mod lexer;
use crate::lexer::Lexer;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: rlox <script>");
        std::process::exit(64);
    }
    if args.len() == 2 {
        run_file(&args[1])?;
    } else {
        run_prompt()?;
    }
    Ok(())
}

fn run_file(path: &String) -> io::Result<()> {
    let source = fs::read_to_string(path)?;
    run(&source);
    Ok(())
}

fn run_prompt() -> io::Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    loop {
        print!("> ");
        handle.read_line(&mut buffer)?;
        if buffer.is_empty() {
            break;
        }
        run(&buffer);
    }
    Ok(())
}

fn run(source: &str) {
    let lexer = Lexer::new(source);
    for token in lexer.scan_tokens() {
        println!("{}", token);
    }
}
