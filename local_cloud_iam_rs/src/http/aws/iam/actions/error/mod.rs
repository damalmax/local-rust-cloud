use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;

use axum::http::StatusCode;
use derive_more::Display;

use local_cloud_validate::{ValidationError, ValidationErrorKind};

pub(crate) mod output;

#[derive(Debug)]
pub(crate) struct ApiError {
    pub(crate) kind: ApiErrorKind,
    pub(crate) message: Option<String>,
    pub(crate) aws_request_id: String,
    pub(crate) extras: Option<HashMap<&'static str, String>>,
}

impl ApiError {
    pub(crate) fn new(kind: ApiErrorKind, message: impl Into<String>, aws_request_id: impl Into<String>) -> Self {
        ApiError {
            kind,
            message: Some(message.into()),
            aws_request_id: aws_request_id.into(),
            extras: None,
        }
    }

    pub(crate) fn from_validation_error(error: &ValidationError, aws_request_id: &str) -> Self {
        let kind = match error.kind {
            ValidationErrorKind::Password => ApiErrorKind::PasswordPolicyViolation,
            _ => ApiErrorKind::InvalidInput,
        };

        ApiError {
            kind,
            message: Some(error.message.to_owned()),
            aws_request_id: aws_request_id.to_owned(),
            extras: None,
        }
    }

    pub(crate) fn new_with_extras(
        kind: ApiErrorKind, message: &str, aws_request_id: &str, extras: HashMap<&'static str, String>,
    ) -> Self {
        ApiError {
            kind,
            message: Some(message.to_owned()),
            aws_request_id: aws_request_id.to_owned(),
            extras: Some(extras),
        }
    }
}

#[derive(Debug, Display, PartialEq)]
pub enum ApiErrorKind {
    ConcurrentModification,
    CredentialReportExpired,
    CredentialReportNotPresent,
    CredentialReportNotReady,
    DeleteConflict,
    DuplicateCertificate,
    DuplicateSshPublicKey,
    EntityAlreadyExists,
    EntityTemporarilyUnmodifiable,
    InvalidAuthenticationCode,
    InvalidCertificate,
    InvalidInput,
    InvalidPublicKey,
    InvalidUserType,
    KeyPairMismatch,
    LimitExceeded,
    MalformedCertificate,
    MalformedPolicyDocument,
    NoSuchEntity,
    PasswordPolicyViolation,
    PolicyEvaluation,
    PolicyNotAttachable,
    ReportGenerationLimitExceeded,
    ServiceFailure,
    ServiceNotSupported,
    UnmodifiableEntity,
    UnrecognizedPublicKeyEncoding,
}

impl ApiErrorKind {
    pub(crate) fn status_code(&self) -> StatusCode {
        match self {
            ApiErrorKind::LimitExceeded => StatusCode::CONFLICT,
            ApiErrorKind::EntityAlreadyExists => StatusCode::CONFLICT,
            ApiErrorKind::ServiceFailure => StatusCode::INTERNAL_SERVER_ERROR,
            ApiErrorKind::ServiceNotSupported => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::BAD_REQUEST,
        }
    }

    pub(crate) fn as_str(&self) -> &str {
        match self {
            ApiErrorKind::ConcurrentModification => "ConcurrentModification",
            ApiErrorKind::CredentialReportExpired => "CredentialReportExpired",
            ApiErrorKind::CredentialReportNotPresent => "CredentialReportNotPresent",
            ApiErrorKind::CredentialReportNotReady => "CredentialReportNotReady",
            ApiErrorKind::DeleteConflict => "DeleteConflict",
            ApiErrorKind::DuplicateCertificate => "DuplicateCertificate",
            ApiErrorKind::DuplicateSshPublicKey => "DuplicateSshPublicKey",
            ApiErrorKind::EntityAlreadyExists => "EntityAlreadyExists",
            ApiErrorKind::EntityTemporarilyUnmodifiable => "EntityTemporarilyUnmodifiable",
            ApiErrorKind::InvalidAuthenticationCode => "InvalidAuthenticationCode",
            ApiErrorKind::InvalidCertificate => "InvalidCertificate",
            ApiErrorKind::InvalidInput => "InvalidInput",
            ApiErrorKind::InvalidPublicKey => "InvalidPublicKey",
            ApiErrorKind::InvalidUserType => "InvalidUserType",
            ApiErrorKind::KeyPairMismatch => "KeyPairMismatch",
            ApiErrorKind::LimitExceeded => "LimitExceeded",
            ApiErrorKind::MalformedCertificate => "MalformedCertificate",
            ApiErrorKind::MalformedPolicyDocument => "MalformedPolicyDocument",
            ApiErrorKind::NoSuchEntity => "NoSuchEntity",
            ApiErrorKind::PasswordPolicyViolation => "PasswordPolicyViolation",
            ApiErrorKind::PolicyEvaluation => "PolicyEvaluation",
            ApiErrorKind::PolicyNotAttachable => "PolicyNotAttachable",
            ApiErrorKind::ReportGenerationLimitExceeded => "ReportGenerationLimitExceeded",
            ApiErrorKind::ServiceFailure => "ServiceFailure",
            ApiErrorKind::ServiceNotSupported => "ServiceNotSupported",
            ApiErrorKind::UnmodifiableEntity => "UnmodifiableEntity",
            ApiErrorKind::UnrecognizedPublicKeyEncoding => "UnrecognizedPublicKeyEncoding",
        }
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {})", self.kind.as_str(), self.message.as_deref().unwrap_or(""))
    }
}

impl Error for ApiError {}
