use std::io;
use std::process::exit;

use crate::scanner::Scanner;

pub struct Lox {
    had_error: bool,
}

impl Lox {
    pub fn new() -> Self {
        Self { had_error: false }
    }

    pub fn run_program(&mut self) {
        let args: Vec<String> = std::env::args().collect();

        match args.len() {
            2 => {
                match self.run_file(args.get(1).expect("lox source code file path")) {
                    Ok(_) => (),
                    Err(_) => {
                        eprintln!("error reading bytes from supplied file");
                        exit(66);
                    }
                };
            }
            3..=usize::MAX => {
                println!("Usage: rlox [script]");
                exit(64); // exits with "incorrect command usage" code
            }
            _ => {
                self.run_prompt();
            }
        }
    }

    fn run_file(&self, file_name: &str) -> Result<(), io::Error> {
        let file_string = std::fs::read_to_string(file_name)?;
        let source: Vec<u8> = file_string.bytes().collect();
        self.run(std::str::from_utf8(&source).unwrap().to_string());

        if self.had_error {
            exit(65)
        };

        Ok(())
    }

    fn run_prompt(&mut self) {
        let stdin = io::stdin();

        for line in stdin.lines() {
            print!("> ");
            self.run(line.unwrap());
            self.had_error = false;
        }
    }

    fn run(&self, source: String) {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();

        for token in tokens {
            println!("{token:#?}");
        }
    }
}

impl Default for Lox {
    fn default() -> Self {
        Self::new()
    }
}
