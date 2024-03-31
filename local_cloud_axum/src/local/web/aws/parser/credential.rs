use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::all_consuming;
use nom::sequence::tuple;
use nom::IResult;

use crate::local::web::aws::models::Credential;
use crate::local::web::aws::parser::utils::text;

pub(crate) fn parse_credential(input: &str) -> IResult<&str, Credential> {
    let (input, (access_key, _, time, _)) = tuple((text, tag("/"), text, tag("/")))(input)?;

    let (input, (region, _, service, _)) = all_consuming(alt((
        tuple((text, tag("/"), text, tag("/aws4_request"))),
        tuple((tag(""), tag(""), text, tag("/aws4_request"))),
    )))(input)?;

    let region = if region.is_empty() { None } else { Some(region) };
    Ok((
        input,
        Credential {
            access_key,
            time,
            region,
            service,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::parse_credential;

    #[test]
    fn test_parse_credential_five_params() {
        let input = "AKIAIOSFODNN201ADMIN/20240308/eu-local-1/iam/aws4_request";

        let (_, credential) = parse_credential(input).unwrap();

        assert_eq!(credential.access_key, "AKIAIOSFODNN201ADMIN");
        assert_eq!(credential.time, "20240308");
        assert!(credential.region.is_some());
        assert_eq!(credential.region.unwrap(), "eu-local-1");
        assert_eq!(credential.service, "iam");
    }

    #[test]
    fn test_parse_invalid_credential() {
        let input = "AKIAIOSFODNN201ADMIN/20240308/eu-local-1/iam/aws4_request1";

        let result = parse_credential(input);

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_invalid_credential_request() {
        let input = "AKIAIOSFODNN201ADMIN";

        let result = parse_credential(input);

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_credential_four_params() {
        let input = "AKIAIOSFODNN201ADMIN/20240308/iam/aws4_request";

        let (_, credential) = parse_credential(input).unwrap();

        assert_eq!(credential.access_key, "AKIAIOSFODNN201ADMIN");
        assert_eq!(credential.time, "20240308");
        assert!(credential.region.is_none());
        assert_eq!(credential.service, "iam");
    }

    #[test]
    fn test_parse_credential_six_params() {
        let input = "AKIAIOSFODNN201ADMIN/20240308/eu-local-1/iam/some_other_value/aws4_request";

        let result = parse_credential(input);

        assert!(result.is_err());
    }
}
