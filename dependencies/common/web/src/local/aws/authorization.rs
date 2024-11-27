#[derive(Debug, PartialEq)]
pub enum AuthAlgorithm {
    Aws4HmacSha256,
    Unknown,
}

impl AuthAlgorithm {
    pub fn from_str(input: &str) -> AuthAlgorithm {
        return if input == "AWS4-HMAC-SHA256" {
            AuthAlgorithm::Aws4HmacSha256
        } else {
            AuthAlgorithm::Unknown
        };
    }
}

#[derive(Debug, PartialEq)]
pub struct Credential<'a> {
    pub access_key: &'a str,
    pub time: &'a str,
    pub region: Option<&'a str>,
    pub service: &'a str,
}

#[derive(Debug, PartialEq)]
pub struct Authorization<'a> {
    pub algorithm: AuthAlgorithm,
    pub credential: Credential<'a>,
    pub signed_headers: Option<Vec<&'a str>>,
    pub signature: &'a str,
}