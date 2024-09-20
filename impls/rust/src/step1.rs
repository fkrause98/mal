use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_until},
    character::complete::{alpha1, alphanumeric1, one_of, space0, space1},
    combinator::{map, not, recognize},
    error::{context, convert_error, VerboseError},
    multi::{many0, many1},
    sequence::{delimited, preceded, terminated},
    IResult,
};
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};
use std::fmt;

#[derive(Debug, PartialEq)]
enum Expr {
    Number(i64),
    String(String),
    Symbol(String),
    List(Vec<Expr>),
}

fn parse_operator(input: &str) -> IResult<&str, Expr, VerboseError<&str>> {
    context(
        "operator",
        map(
            terminated(recognize(one_of("*+-/=")), not(space0)),
            |s: &str| Expr::Symbol(s.to_string()),
        ),
    )(input)
}

fn parse_deref(input: &str) -> IResult<&str, Expr, VerboseError<&str>> {
    let parser = preceded(tag("@"), alphanumeric1);
    context(
        "deref",
        map(parser, |s: &str| {
            Expr::List(vec![
                Expr::Symbol("deref".to_string()),
                Expr::Symbol(s.to_string()),
            ])
        }),
    )(input)
}

fn parse_alpha(input: &str) -> IResult<&str, Expr, VerboseError<&str>> {
    context(
        "alpha",
        map(recognize(alphanumeric1), |s: &str| {
            Expr::Symbol(s.to_string())
        }),
    )(input)
}

fn parse_alpha_and_stop(input: &str) -> IResult<&str, Expr, VerboseError<&str>> {
    context("parse alpha and stop", terminated(parse_alpha, not(space0)))(input)
}

fn parse_symbol(input: &str) -> IResult<&str, Expr, VerboseError<&str>> {
    context("symbol", alt((parse_operator, parse_alpha_and_stop)))(input)
}

fn parse_string(input: &str) -> IResult<&str, Expr, VerboseError<&str>> {
    context(
        "string",
        map(
            delimited(tag("\""), take_until("\""), tag("\"")),
            |s: &str| Expr::String(s.to_string()),
        ),
    )(input)
}

fn parse_positive_num(input: &str) -> IResult<&str, Expr, VerboseError<&str>> {
    context(
        "number",
        map(recognize(many1(one_of("1234567890"))), |s: &str| {
            Expr::Number(s.parse::<i64>().unwrap())
        }),
    )(input)
}

fn parse_negative_num(input: &str) -> IResult<&str, Expr, VerboseError<&str>> {
    context(
        "number",
        map(
            recognize(preceded(tag("-"), parse_positive_num)),
            |s: &str| Expr::Number(s.parse::<i64>().unwrap()),
        ),
    )(input)
}

fn parse_num(input: &str) -> IResult<&str, Expr, VerboseError<&str>> {
    let num_parsers = (parse_negative_num, parse_positive_num);
    alt(num_parsers)(input)
}

fn parse_expr(input: &str) -> IResult<&str, Expr, VerboseError<&str>> {
    context(
        "expression",
        alt((
            parse_deref,
            parse_num,
            parse_string,
            parse_symbol,
            parse_list,
        )),
    )(input)
}

fn parse_list(input: &str) -> IResult<&str, Expr, VerboseError<&str>> {
    context(
        "list",
        map(
            delimited(
                preceded(space0, tag("(")),
                many0(preceded(space0, parse_expr)),
                preceded(space0, tag(")")),
            ),
            Expr::List,
        ),
    )(input)
}

fn parse_lisp(input: &str) -> IResult<&str, Expr, VerboseError<&str>> {
    context(
        "lisp expression",
        delimited(space0, alt((parse_expr, parse_list)), space0),
    )(input)
}

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
            Expr::String(string) => format!("\"{}\"", string),
        };
        write!(f, "{}", as_string)
    }
}

pub fn main() -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    #[cfg(feature = "with-file-history")]
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline("user> ");
        match readline {
            Ok(line) if !line.is_empty() => {
                rl.add_history_entry(line.as_str())?;
                if line == "exit" {
                    break;
                } else {
                    let line = line.replace(',', " ");
                    match (parse_lisp(&line.trim())) {
                        Ok((_, expr)) => println!("{}", expr),
                        Err(_) => println!("{}", line),
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
        let to_parse = "(+ (- 3 2) 4)";
        let actual = parse_lisp(to_parse);
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
