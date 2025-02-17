use nom::branch::alt;
use nom::bytes::complete::{tag, take_till, take_while1};
use nom::{IResult, Parser};

pub(crate) fn text(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| {
        return c.is_alphanumeric() || c == '_' || c == '-';
    })(input)
}

pub(crate) fn till_comma(input: &str) -> IResult<&str, &str> {
    let (input, value) = take_till(|c| c == ',').parse(input)?;
    let (input, _) = alt((tag(","), tag(""))).parse(input.trim())?;

    Ok((input, value))
}

pub(crate) fn till_equals(input: &str) -> IResult<&str, &str> {
    let (input, value) = take_till(|c| c == '=').parse(input)?;
    let (input, _) = tag("=").parse(input.trim())?;
    Ok((input, value))
}
