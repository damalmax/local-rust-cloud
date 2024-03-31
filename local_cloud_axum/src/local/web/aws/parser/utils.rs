use nom::bytes::complete::take_while1;
use nom::IResult;

pub(crate) fn text(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| {
        return c.is_alphanumeric() || c == '_' || c == '-';
    })(input)
}
