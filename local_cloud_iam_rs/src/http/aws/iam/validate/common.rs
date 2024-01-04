use crate::http::aws::iam::validate::error::{ValidationError, ValidationErrorKind};

pub(crate) fn validate_property_present<T: Sized>(
    input: Option<T>, error_msg_supplier: fn() -> &'static str,
) -> Result<(), ValidationError> {
    match input {
        None => Err(ValidationError::new(ValidationErrorKind::InvalidInput, error_msg_supplier())),
        Some(_) => Ok(()),
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::http::aws::iam::validate::common::validate_property_present;
    use crate::http::aws::iam::validate::error::{ValidationError, ValidationErrorKind};

    #[test]
    fn test_validate_policy_document_present_not_provided() {
        let result = validate_property_present::<&str>(None, || "some error.");
        assert!(result.is_err());
        let expected_error = ValidationError::new(ValidationErrorKind::InvalidInput, "some error.");
        assert_eq!(result.unwrap_err(), expected_error);
    }

    #[test]
    fn test_validate_policy_document_present_present() {
        let result = validate_property_present(
            Some(
                json!({
                    "Version":"2012-10-17",
                    "Statement":[]
                })
                .as_str(),
            ),
            || "some error",
        );
        assert!(result.is_ok());
    }
}
