#![allow(unused_imports, unused)]

use nom::branch::alt;
use nom::error::ErrorKind;
use nom::IResult;
use nom::bytes::complete::{tag, take_while, is_not};
use nom::character::complete::{char, space0};
use nom::combinator::{cut, map, not, opt};
use nom::multi::{fold_many0, separated_list};
use nom::sequence::{delimited, preceded, terminated};
use nom::eof;

use crate::error::CSVError;

nom::named!(end_of_line<&str,&str>, eof!());

fn consume_newlines(input: &str) -> Result<&str, CSVError> {
    let a: IResult<&str, &str> = take_while(|c| c == '\n')(input);
    match a {
        Ok((r, v)) => Ok(r),
        Err(_) => Err(CSVError::SyntaxError),
    }
}


fn cell(input: &str) -> IResult<&str, String> {
    alt((
        terminated(
            preceded(
                space0,
                delimited(
                    char('"'),
                    match_inside_quotes,
                    alt((tag("\""), end_of_line)),
                ),
            ),
            space0,
        ),
        map(opt(take_while(|c| c != ',' && c != '\n')), |s| match s {
            Some(s) => String::from(s),
            None => String::new(),
        }),
    ))(input)
}

fn many_cells(input: &str) -> IResult<&str, Vec<String>> {
    separated_list(char(','), cell)(input)
}


fn match_inside_quotes(i: &str) -> IResult<&str, String> {
    fold_many0(
        alt((is_not("\""), tag("\"\""))),
        String::from(""),
        |acc, val| {
            if val == "\"\"" {
                acc + "\""
            } else {
                acc + val
            }
        },
    )(i)
}

fn csv(input: &str) -> Result<Vec<Vec<String>>, CSVError> {
    let mut out = Vec::new();
    let mut rest;
    let mut row;
    match many_cells(input) {
        Ok((r, v)) => {
            rest = r;
            row = v;
        }
        Err(_) => {
            return Err(CSVError::SyntaxError);
        }
    }

    let len = row.len();
    out.push(row);

    while rest.len() > 0 {
        rest = consume_newlines(rest)?;
        match many_cells(rest) {
            Ok((r, v)) => {
                rest = r;
                row = v;
            }
            Err(e) => {
                return Err(CSVError::SyntaxError);
            }
        }

        if row.len() != len {
            return Err(CSVError::UnequalColumns);
        } else {
            out.push(row);
        }
    }

    Ok(out)
}


#[test]
fn test_quotes() {
    let res = match_inside_quotes("hello\"\"\"\"\n kello\"");
    assert_eq!(res, Ok(("\"", String::from("hello\"\"\n kello"))));
}

#[test]
fn cell_test() {
    assert_eq!(
        cell("\"hello world\" yes "),
        Ok(("yes ", String::from("hello world")))
    );
    assert_eq!(
        cell("hello,buffalo,2"),
        Ok((",buffalo,2", String::from("hello")))
    );
    assert_eq!(
        cell("\"Hello, Buffalo\n2,3\n"),
        Ok(("", String::from("Hello, Buffalo\n2,3\n")))
    );
}


#[test]
fn many_cells_test() {
    assert_eq!(
        many_cells("this,\"that\" ,,2,9\n"),
        Ok((
            "\n",
            vec!["this", "that", "", "2", "9"]
                .iter()
                .map(|&s| String::from(s))
                .collect()
        ))
    );
}

#[test]
fn csv_test() {
    let text = "\"ankit\", \"he\"\"llo\",8,9\n2,3,\"1\"\"\",5\n1,\"5\n\",\"6,9";
    assert_eq!(csv(text), Err(CSVError::UnequalColumns));
}
