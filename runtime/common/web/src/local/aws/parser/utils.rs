use nom::branch::alt;
use nom::bytes::complete::{tag, take_till, take_while1};
use nom::IResult;

pub(crate) fn text(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| {
        return c.is_alphanumeric() || c == '_' || c == '-';
    })(input)
}

pub(crate) fn till_comma(input: &str) -> IResult<&str, &str> {
    let (input, value) = take_till(|c| c == ',')(input)?;
    let (input, _) = alt((tag(","), tag("")))(input.trim())?;

    Ok((input, value))
}

pub(crate) fn till_equals(input: &str) -> IResult<&str, &str> {
    let (input, value) = take_till(|c| c == '=')(input)?;
    let (input, _) = tag("=")(input.trim())?;
    Ok((input, value))
}
