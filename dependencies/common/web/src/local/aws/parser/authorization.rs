use nom::bytes::complete::{tag, take_till};
use nom::combinator::{all_consuming, fail};
use nom::multi::separated_list1;
use nom::IResult;

use crate::local::aws::authorization::{AuthAlgorithm, Authorization};
use crate::local::aws::parser::credential::parse_credential;
use crate::local::aws::parser::utils::text;

use super::utils;

fn parse_auth_algorithm(input: &str) -> IResult<&str, AuthAlgorithm> {
    let (input, algorithm_str) = take_till(|c| c == ' ')(input)?;

    Ok((input, AuthAlgorithm::from_str(algorithm_str)))
}

pub(crate) fn parse_authorization(input: &str) -> IResult<&str, Authorization> {
    let (input, algorithm) = parse_auth_algorithm(input)?;

    let mut credential_parsed = false;
    let mut signed_headers_parsed = false;
    let mut signature_parsed = false;

    let mut credential = None;
    let mut signed_headers = None;
    let mut signature = None;

    let mut input = input;
    while !input.is_empty() {
        let (input_right, param_key) = utils::till_equals(input.trim())?;
        match param_key {
            "Credential" => {
                if credential_parsed {
                    return fail(input_right);
                }
                let (input_str, credential_str) = utils::till_comma(input_right)?;
                let (_, credential_value) = parse_credential(credential_str)?;
                credential = Some(credential_value);
                input = input_str.trim();
                credential_parsed = true;
            }
            "SignedHeaders" => {
                if signed_headers_parsed {
                    return fail(input_right);
                }
                let (input_str, signed_header_names_str) = utils::till_comma(input_right)?;
                let (_, mut signed_header_names) =
                    all_consuming(separated_list1(tag(";"), text))(signed_header_names_str)?;
                input = input_str.trim();
                signed_headers_parsed = true;
                signed_header_names.sort();
                signed_headers = Some(signed_header_names);
            }
            "Signature" => {
                if signature_parsed {
                    return fail(input_right);
                }
                let (input_str, signature_str) = utils::till_comma(input_right)?;
                signature = Some(signature_str);
                input = input_str.trim();
                signature_parsed = true;
            }
            _ => return fail(input_right),
        }
    }

    if !credential_parsed || !signed_headers_parsed || !signature_parsed {
        return fail(input);
    }

    Ok((
        input,
        Authorization {
            algorithm,
            credential: credential.unwrap(),
            signed_headers,
            signature: signature.unwrap(),
        },
    ))
}

#[cfg(test)]
mod tests {

    use crate::local::aws::authorization::{AuthAlgorithm, Credential};

    use super::parse_authorization;

    #[test]
    fn test_parse_authorization() {
        let input = "AWS4-HMAC-SHA256 Credential=AKIAIOSFODNN201ADMIN/20240308/eu-local-1/iam/aws4_request, SignedHeaders=content-length;content-type;host;x-amz-date;x-amz-user-agent, Signature=bfe62e91fe38262f162d7517aa342dab00207b2106b3b715a91f685acfa2e076";

        let (_, authorization) = parse_authorization(input).unwrap();

        assert_eq!(authorization.algorithm, AuthAlgorithm::Aws4HmacSha256);
        assert_eq!(
            authorization.credential,
            Credential {
                access_key: "AKIAIOSFODNN201ADMIN",
                time: "20240308",
                region: Some("eu-local-1"),
                service: "iam",
            }
        );
        assert_eq!(
            authorization.signed_headers.unwrap(),
            vec![
                "content-length",
                "content-type",
                "host",
                "x-amz-date",
                "x-amz-user-agent"
            ]
        );
        assert_eq!(authorization.signature, "bfe62e91fe38262f162d7517aa342dab00207b2106b3b715a91f685acfa2e076");
    }

    #[test]
    fn test_parse_authorization_mixed_params() {
        let input = "AWS4-HMAC-SHA256 Signature=bfe62e91fe38262f162d7517aa342dab00207b2106b3b715a91f685acfa2e076, SignedHeaders=content-length;content-type;host;x-amz-date;x-amz-user-agent, Credential=AKIAIOSFODNN201ADMIN/20240308/eu-local-1/iam/aws4_request";

        let (_, authorization) = parse_authorization(input).unwrap();

        assert_eq!(authorization.algorithm, AuthAlgorithm::Aws4HmacSha256);
        assert_eq!(
            authorization.credential,
            Credential {
                access_key: "AKIAIOSFODNN201ADMIN",
                time: "20240308",
                region: Some("eu-local-1"),
                service: "iam",
            }
        );
        assert_eq!(
            authorization.signed_headers.unwrap(),
            vec![
                "content-length",
                "content-type",
                "host",
                "x-amz-date",
                "x-amz-user-agent"
            ]
        );
        assert_eq!(authorization.signature, "bfe62e91fe38262f162d7517aa342dab00207b2106b3b715a91f685acfa2e076");
    }
}
