use std::path::Display;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{one_of, space0},
    combinator::{map, recognize},
    multi::{many0, many1},
    sequence::{delimited, preceded},
    IResult,
};
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

#[derive(Debug, PartialEq)]
enum Expr {
    Number(u64),
    Symbol(String),
    List(Vec<Expr>),
}

fn parse_symbol(input: &str) -> IResult<&str, Expr> {
    map(recognize(one_of("+-/=")), |s: &str| {
        Expr::Symbol(s.to_string())
    })(input)
}

fn parse_num(input: &str) -> IResult<&str, Expr> {
    map(recognize(many1(one_of("1234567890"))), |s: &str| {
        Expr::Number(s.parse::<u64>().unwrap())
    })(input)
}

fn parse_expr(input: &str) -> IResult<&str, Expr> {
    alt((parse_symbol, parse_num, parse_list))(input)
}

fn parse_list(input: &str) -> IResult<&str, Expr> {
    map(
        delimited(
            preceded(space0, tag("(")),
            many0(preceded(space0, parse_expr)),
            preceded(space0, tag(")")),
        ),
        Expr::List,
    )(input)
}
fn parse_lisp(input: &str) -> IResult<&str, Expr> {
    alt((parse_list, parse_expr))(input)
}
use std::fmt;

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let as_string = match self {
            Expr::Number(as_num) => format!("{}", as_num),
            Expr::Symbol(sym) => sym.to_string(),
            Expr::List(expr) => {
                let inner = expr
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<String>>()
                    .join(" ");
                format!("({})", inner)
            }
        };
        write!(f, "{}", as_string)
    }
}

pub fn main() -> Result<()> {
    // `()` can be used when no completer is required
    let mut rl = DefaultEditor::new()?;
    #[cfg(feature = "with-file-history")]
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline("user> ");
        match readline {
            Ok(line) if line.len() > 0 => {
                rl.add_history_entry(line.as_str())?;
                match line.as_str() {
                    "exit" => break,
                    line => {
                        let (a, b) = parse_lisp(line).unwrap();
                        println!("{}", b);
                    }
                }
            }
            Ok(_) => {}
            Err(ReadlineError::Eof) => break,
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    #[cfg(feature = "with-file-history")]
    rl.save_history("history.txt");
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_parse_num() {
        let to_parse = "(+ 1 2 3 4)";
        let actual = parse_list(to_parse);
        match actual {
            Ok(("", Expr::List(list))) => {
                let plus_sym = "+".to_string();
                assert_eq!(
                    &list[..],
                    [
                        Expr::Symbol(plus_sym),
                        Expr::Number(1),
                        Expr::Number(2),
                        Expr::Number(3),
                        Expr::Number(4)
                    ]
                )
            }
            _ => panic!("Failed to parse: {to_parse}"),
        }
    }
    #[test]
    pub fn test_parse_num_2() {
        let to_parse = "(+ 1234444)";
        let actual = parse_list(to_parse);
        match actual {
            Ok(("", Expr::List(list))) => {
                let plus_sym = "+".to_string();
                assert_eq!([Expr::Symbol(plus_sym), Expr::Number(1234444)], &list[..])
            }
            _ => panic!("Failed to parse: {to_parse}"),
        }
    }
    #[test]
    pub fn test_parse_num_3() {
        let to_parse = "(+  (- 3 2 ) 4)";
        let actual = parse_list(to_parse);
        match actual {
            Ok(("", Expr::List(list))) => {
                let plus_sym = "+".to_string();
                let minus_sym = "-".to_string();
                assert_eq!(
                    [
                        Expr::Symbol(plus_sym),
                        Expr::List(vec![
                            Expr::Symbol(minus_sym),
                            Expr::Number(3),
                            Expr::Number(2)
                        ]),
                        Expr::Number(4)
                    ],
                    &list[..]
                )
            }
            _ => panic!("Failed to parse: {to_parse}"),
        }
    }
}
