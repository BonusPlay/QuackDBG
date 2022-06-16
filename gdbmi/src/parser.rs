use nom::IResult;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::sequence::delimited;
use nom::combinator::map;
use nom::character::complete::char;

use crate::quoted_string::parse_quoted_string;

#[derive(Debug, PartialEq)]
pub enum GdbOutput {
    Console(String),
    Target(String),
    Log(String),
}

fn parse_console(input: &str) -> IResult<&str, GdbOutput> {
    delimited(
        char('~'),
        map(parse_quoted_string, GdbOutput::Console),
        tag("\r\n"),
    )(input)
}

fn parse_target(input: &str) -> IResult<&str, GdbOutput> {
    delimited(
        char('@'),
        map(parse_quoted_string, GdbOutput::Target),
        tag("\r\n"),
    )(input)
}

fn parse_log(input: &str) -> IResult<&str, GdbOutput> {
    delimited(
        char('&'),
        map(parse_quoted_string, GdbOutput::Log),
        tag("\r\n"),
    )(input)
}

pub fn parse(input: &str) -> IResult<&str, GdbOutput> {
    alt((
        parse_console,
        parse_target,
        parse_log,
    ))(input)
}

#[test]
fn parse_test() {
    assert_eq!(parse("~\"test\"\r\n"), Ok(("", GdbOutput::Console { 0: "test".into() })));
    assert_eq!(parse("@\"test\"\r\n"), Ok(("", GdbOutput::Target { 0: "test".into() })));
    assert_eq!(parse("&\"test\"\r\n"), Ok(("", GdbOutput::Log { 0: "test".into() })));
}
