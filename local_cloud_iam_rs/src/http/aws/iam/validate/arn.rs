use crate::http::aws::iam::validate::error::ValidationError;

pub(crate) fn validate_arn(arn: Option<&str>) -> Result<(), ValidationError> {
    match arn {
        None => Ok(()),
        Some(arn) => Ok(()),
    }
}
