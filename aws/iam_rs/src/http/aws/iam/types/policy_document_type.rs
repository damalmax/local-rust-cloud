use std::ops::Deref;

use serde::{Deserialize, Deserializer};
use validator::Validate;

use iam_policy::types::LocalPolicyDocument;
use validators::{ValidationError, ValidationErrorKind};

lazy_static::lazy_static! {
    static ref REGEX : regex::Regex =
    regex::Regex::new(r"^[\u0009\u000A\u000D\u0020-\u00FF]+$").unwrap();
}
#[derive(Debug, PartialEq)]
pub(crate) struct PolicyDocumentType {
    raw_value: String,
    minified_value: Result<String, ValidationError>,
}

impl PolicyDocumentType {
    fn new(raw_value: String, minified_value: Result<String, ValidationError>) -> Self {
        PolicyDocumentType {
            raw_value,
            minified_value,
        }
    }

    pub(crate) fn document(&self) -> Result<&str, ValidationError> {
        match &self.minified_value {
            Ok(doc) => Ok(doc),
            Err(err) => Err(err.clone()),
        }
    }
}

impl<'de> Deserialize<'de> for PolicyDocumentType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw_value: String = Deserialize::deserialize(deserializer)?;

        let policy_document: Result<LocalPolicyDocument, ValidationError> = serde_json::from_str(&raw_value)
            .map_err(|_err| ValidationError::new(ValidationErrorKind::Other, "Malformed Policy Document."));

        let result = match policy_document {
            Ok(document) => match document.validate() {
                Ok(_) => {
                    let minified_value = serde_json::to_string(&document).map_err(|_err| {
                        ValidationError::new(ValidationErrorKind::Other, "Failed to minimize Policy Document.")
                    });
                    PolicyDocumentType::new(raw_value, minified_value)
                }
                Err(err) => {
                    log::debug!("Policy Document validation failed. Errors: {:?}", err);
                    let error = ValidationError::new(ValidationErrorKind::Other, "Malformed policy document.");
                    PolicyDocumentType::new(raw_value, Err(error))
                }
            },
            Err(err) => PolicyDocumentType::new(raw_value, Err(err)),
        };
        Ok(result)
    }
}

impl Deref for PolicyDocumentType {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.raw_value
    }
}

impl validators::NamedValidator for &PolicyDocumentType {
    fn validate(&self, at: &str) -> Result<(), ValidationError> {
        validators::validate_str_length_min(Some(self), 1usize, at)?;
        validators::validate_str_length_max(Some(self), 131072usize, at)?;
        validators::validate_regexp(Some(self), REGEX.deref(), at)?;
        match self.document() {
            Err(err) => Err(err),
            Ok(_) => Ok(()),
        }
    }
}
