use local_rust_cloud_common::request::LocalTag;

use crate::aws::actions::errors::IamApiError;

pub fn validate(request_id: &str, tags: &[LocalTag]) -> Result<(), IamApiError> {
    if tags.len() == 0 {
        return Result::Ok(());
    }
    if tags.len() > 50 {
        return Result::Err(IamApiError::too_many_tags(request_id, tags, "Tags"));
    }

    for tag in tags {
        if tag.value().len() > 256 {
            return Result::Err(IamApiError::tag_value_too_big(request_id, tag.value(), tag.tag_index()));
        }
    }
    return Result::Ok(());
}
