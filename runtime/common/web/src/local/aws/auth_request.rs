use super::authorization::Authorization;

#[derive(Debug, PartialEq)]
pub struct AwsSdkRequest {
    pub attempt: u8,
    pub max: u8,
}

/// Represents main properties of a request to any AWS service. It is used for permissions validation.
///
/// Below you can find a list of headers that could be sent by AWS SDK client:
/// * content-type: application/x-www-form-urlencoded
/// * content-length: 108
/// * user-agent: aws-sdk-rust/1.1.4 os/linux lang/rust/1.76.0
/// * x-amz-user-agent: aws-sdk-rust/1.1.4 api/iam/1.12.0 os/linux lang/rust/1.76.0
/// * x-amz-date: 20240308T203241Z
/// * authorization: AWS4-HMAC-SHA256 Credential=AKIAIOSFODNN201ADMIN/20240308/eu-local-1/iam/aws4_request, SignedHeaders=content-length;content-type;host;x-amz-date;x-amz-user-agent, Signature=bfe62e91fe38262f162d7517aa342dab00207b2106b3b715a91f685acfa2e076
/// * amz-sdk-request: attempt=1; max=1
/// * amz-sdk-invocation-id: 6cb84b17-6a0a-4948-9c0b-84d7b1411d63
/// * host: localhost:4500
#[derive(Debug, PartialEq)]
pub struct AwsAuthRequest<'a> {
    pub authorization: Authorization<'a>,
    pub aws_sdk_request: AwsSdkRequest,
    pub aws_sdk_invocation_id: Option<&'a str>,
}

// impl<'a> AwsAuthRequest<'a> {
//     pub fn parse() -> std::result::Result<AwsAuthRequest<'a>, String> {
//         let (_, authorization) = parse_authorization(input).map_err(|err| err.to_string())?;

//         return Ok(AwsAuthRequest {
//             authorization,
//             aws_sdk_request: todo!(),
//             aws_sdk_invocation_id: todo!(),
//         });
//     }

//     pub fn is_valid(&self) -> bool {
//         return true;
//     }
// }

// #[async_trait]
// impl<S, 'a> FromRequest<S> for AwsAuthRequest<'a>
// where
//     S: Send + Sync,
// {
//     type Rejection = AwsAuthRequestRejection;

//     async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
//         Ok(AwsAuthRequest {
//             authorization: todo!(),
//             aws_sdk_request: todo!(),
//             aws_sdk_invocation_id: todo!(),
//         })
//     }
// }

#[derive(Debug)]
pub enum AwsAuthRequestRejection {
    Validation,
}

// impl IntoResponse for AwsAuthRequestRejection {
//     fn into_response(self) -> Response {
//         let body = format!("{}", self);

//         match self {
//             AwsAuthRequestRejection::Validation => (StatusCode::BAD_REQUEST, body).into_response(),
//         }
//     }
// }

#[cfg(test)]
mod tests {}
