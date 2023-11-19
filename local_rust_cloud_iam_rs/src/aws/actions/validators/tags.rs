use aws_sdk_iam::types::Tag;

use crate::aws::actions::errors::IamApiError;

pub fn validate(request_id: &str, tags: Option<&[Tag]>) -> Result<(), IamApiError> {
    if tags.is_none() {
        return Result::Ok(());
    }
    let tags = tags.unwrap();
    if tags.len() > 50 {
        return Result::Err(IamApiError::too_many_tags(request_id, tags, "tags"));
    }

    for tag in tags {
        if tag.value().unwrap_or("").len() > 256 {
            return Result::Err(IamApiError::tag_value_too_big(request_id, tag.value().unwrap()));
        }
    }
    return Result::Ok(());
}
