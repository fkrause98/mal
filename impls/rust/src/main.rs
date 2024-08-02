use core::fmt;

use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};
#[derive(Debug)]
pub enum Token {
    LParen,
    RParen,
    Integer(i64),
    Symbol(String),
}

pub struct Tokens(Vec<Token>);

impl From<&str> for Token {
    fn from(input: &str) -> Token {
        use Token::*;
        match input {
            "(" => LParen,
            ")" => RParen,
            _maybe_int if input.parse::<i64>().is_ok() => Integer(_maybe_int.parse().unwrap()),
            _ => Symbol(input.to_string()),
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let as_string = match self {
            Token::LParen => "(",
            Token::RParen => ")",
            Token::Integer(val) => &val.to_string(),
            Token::Symbol(literal) => &literal,
        };
        write!(f, "{}", as_string)
    }
}

impl fmt::Display for Tokens {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buff = String::new();
        let mut tokens_iter = self.0.iter().peekable();
        while let Some(token) = tokens_iter.next() {
            match token {
                Token::LParen => write!(f, "{}(", buff)?,
                Token::RParen => write!(f, "{})", buff)?,
                _ => {
                    if let Some(Token::RParen) = tokens_iter.peek() {
                        write!(f, "{}{}", buff, token)?
                    } else {
                        write!(f, "{}{} ", buff, token)?
                    }
                }
            }
        }
        write!(f, "{}", buff)
    }
}

fn main() -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    // rl.load_history("history.txt")?;
    loop {
        let readline = rl.readline("lispy> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;
                let line = Tokens(
                    line.replace("(", " (  ")
                        .replace(")", " ) ")
                        .split_whitespace()
                        .map(Token::from)
                        .collect::<Vec<Token>>(),
                );
                println!("{}", line);
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    // rl.save_history("history.txt")?;
    Ok(())
}
