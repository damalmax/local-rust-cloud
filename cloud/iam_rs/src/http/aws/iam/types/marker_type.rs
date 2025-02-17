use std::io::ErrorKind;
use std::ops::Deref;

use data_encoding::BASE64;
use serde::{Deserialize, Deserializer};

use validators::{ValidationError, ValidationErrorKind};

lazy_static::lazy_static! {
    static ref REGEX : regex::Regex = regex::Regex::new(r"^[\u0020-\u00FF]+$").unwrap();
}

const DECODE_MARKER_ERROR_MSG: &str = "Invalid argument value: Marker must be Base64 encoded JSON string.";

#[derive(Debug, PartialEq)]
pub(crate) struct MarkerType {
    raw_value: String,
    marker: Result<Marker, ValidationError>,
}

impl<'de> Deserialize<'de> for MarkerType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let input: String = Deserialize::deserialize(deserializer)?;
        let bytes = BASE64
            .decode(input.as_bytes())
            .map_err(|_err| ValidationError::new(ValidationErrorKind::Other, DECODE_MARKER_ERROR_MSG));

        let marker = match bytes {
            Ok(bytes) => serde_json::from_slice::<Marker>(&bytes)
                .map_err(|_err| ValidationError::new(ValidationErrorKind::Other, DECODE_MARKER_ERROR_MSG)),
            Err(err) => Err(err),
        };
        Ok(MarkerType {
            raw_value: input.to_owned(),
            marker,
        })
    }
}

impl MarkerType {
    pub(crate) fn marker(&self) -> Result<Marker, ValidationError> {
        self.marker.clone()
    }
}

impl Deref for MarkerType {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.raw_value
    }
}

impl validators::NamedValidator for &MarkerType {
    fn validate(&self, at: &str) -> Result<(), ValidationError> {
        validators::validate_str_length_min(Some(self), 1usize, at)?;
        validators::validate_str_length_max(Some(self), 320usize, at)?;
        validators::validate_regexp(Some(self), REGEX.deref(), at)?;
        let marker = self.marker().map_err(|_err| {
            ValidationError::new(ValidationErrorKind::Other, format!("Invalid value provided for '{at}'."))
        })?;
        if marker.truncate_amount < 0 {
            return Err(ValidationError::new(
                ValidationErrorKind::Other,
                format!("Invalid value provided for '{at}'."),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq, Clone)]
pub(crate) struct Marker {
    #[serde(rename = "Marker")]
    pub(crate) marker: Option<String>,
    #[serde(rename = "boto_truncate_amount")]
    pub(crate) truncate_amount: i32,
}

impl Marker {
    pub(crate) fn encode(&self) -> Result<String, std::io::Error> {
        let json = serde_json::to_string(self)
            .map_err(|_err| std::io::Error::new(ErrorKind::Other, "Failed to generate Marker"))?;
        Ok(BASE64.encode(json.as_bytes()))
    }

    pub(crate) fn new(truncate_amount: i32) -> Marker {
        Marker {
            marker: None,
            truncate_amount,
        }
    }
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use crate::http::aws::iam::types::marker_type::{Marker, MarkerType};

    const TOKEN: &str = "eyJNYXJrZXIiOm51bGwsImJvdG9fdHJ1bmNhdGVfYW1vdW50IjoxfQ==";

    #[test]
    fn test_encode_marker() {
        let marker = Marker {
            marker: None,
            truncate_amount: 1,
        };

        let token = marker.encode().unwrap();
        assert_eq!(token.as_str(), TOKEN);
    }

    #[test]
    fn test_decode_marker() {
        #[derive(Debug, Deserialize)]
        struct Request {
            #[serde(rename = "marker")]
            marker_type: MarkerType,
        }

        let params = &[("marker", TOKEN)];

        let query_str = serde_urlencoded::to_string(params).unwrap();

        let result = local_aws_query_protocol::from_str::<Request>(&query_str);

        assert!(result.is_ok());
        let request = result.unwrap();
        assert_eq!(
            request.marker_type.marker().unwrap(),
            Marker {
                marker: None,
                truncate_amount: 1,
            }
        );
    }
}
