use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{one_of, space0},
    combinator::{map, recognize},
    multi::{many0, many1},
    sequence::{delimited, preceded},
    IResult,
};

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
pub fn main() {}

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
