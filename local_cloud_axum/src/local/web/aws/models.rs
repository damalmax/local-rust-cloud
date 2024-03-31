// content-type: application/x-www-form-urlencoded
// content-length: 108
// user-agent: aws-sdk-rust/1.1.4 os/linux lang/rust/1.76.0
// x-amz-user-agent: aws-sdk-rust/1.1.4 api/iam/1.12.0 os/linux lang/rust/1.76.0
// x-amz-date: 20240308T203241Z
// authorization: AWS4-HMAC-SHA256 Credential=AKIAIOSFODNN201ADMIN/20240308/eu-local-1/iam/aws4_request, SignedHeaders=content-length;content-type;host;x-amz-date;x-amz-user-agent, Signature=bfe62e91fe38262f162d7517aa342dab00207b2106b3b715a91f685acfa2e076
// amz-sdk-request: attempt=1; max=1
// amz-sdk-invocation-id: 6cb84b17-6a0a-4948-9c0b-84d7b1411d63
// host: localhost:4500
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
