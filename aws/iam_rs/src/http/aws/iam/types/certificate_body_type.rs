use std::ops::Deref;

use regex::Regex;
use serde::{Deserialize, Deserializer};
use x509_parser::pem::parse_x509_pem;
use x509_parser::prelude::ASN1Time;

use validators::{
    validate_regexp, validate_str_length_max, validate_str_length_min, NamedValidator, ValidationError,
    ValidationErrorKind,
};

use crate::http::aws::iam::types::error::ParseError;

lazy_static::lazy_static! {
    static ref REGEX : Regex = Regex::new(r"^[\u0009\u000A\u000D\u0020-\u00FF]+$").unwrap();
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct X509Validity {
    pub(crate) not_before: ASN1Time,
    pub(crate) not_after: ASN1Time,
}

#[derive(Debug, PartialEq)]
pub(crate) struct X509Metadata {
    pub(crate) validity: X509Validity,
}

#[derive(Debug, PartialEq)]
pub(crate) struct CertificateBodyType {
    raw_value: String,
    cert: Result<X509Metadata, ParseError>,
}

impl CertificateBodyType {
    pub(crate) fn metadata(&self) -> Result<&X509Metadata, &ParseError> {
        match &self.cert {
            Ok(cert) => Ok(cert),
            Err(err) => Err(err),
        }
    }
}

impl Deref for CertificateBodyType {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.raw_value
    }
}

impl<'de> Deserialize<'de> for CertificateBodyType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw_value: String = Deserialize::deserialize(deserializer)?;

        let cert = match parse_x509_pem(raw_value.as_bytes()) {
            Ok(pem) => match pem.1.parse_x509() {
                Ok(x509cert) => {
                    let validity = &x509cert.validity;
                    let metadata = X509Metadata {
                        validity: X509Validity {
                            not_before: validity.not_before,
                            not_after: validity.not_after,
                        },
                    };
                    Ok(metadata)
                }
                Err(_err) => Err(ParseError::new("Invalid or unsupported value provided.")),
            },
            Err(_err) => Err(ParseError::new("Invalid or unsupported value provided.")),
        };

        let result = CertificateBodyType { raw_value, cert };

        Ok(result)
    }
}

impl NamedValidator for &CertificateBodyType {
    fn validate(&self, at: &str) -> Result<(), ValidationError> {
        validate_str_length_min(Some(self), 1usize, at)?;
        validate_str_length_max(Some(self), 16384usize, at)?;
        validate_regexp(Some(self), REGEX.deref(), at)?;

        if let Err(err) = &self.cert {
            return Err(ValidationError::new(
                ValidationErrorKind::Other,
                format!("Failed to parse '{at}' parameter value. {}", err),
            ));
        }

        Ok(())
    }
}
