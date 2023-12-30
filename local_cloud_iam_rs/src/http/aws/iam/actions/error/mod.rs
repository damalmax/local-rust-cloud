use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;

use actix_web::http::StatusCode;
use derive_more::Display;

pub(crate) mod output;

#[derive(Debug)]
pub(crate) struct IamError {
    pub(crate) kind: IamErrorKind,
    pub(crate) message: Option<String>,
    pub(crate) aws_request_id: String,
    pub(crate) extras: Option<HashMap<&'static str, String>>,
}

impl IamError {
    pub(crate) fn new(kind: IamErrorKind, message: &str, aws_request_id: &str) -> Self {
        IamError {
            kind,
            message: Some(message.to_owned()),
            aws_request_id: aws_request_id.to_owned(),
            extras: None,
        }
    }

    pub(crate) fn new_with_extras(
        kind: IamErrorKind, message: &str, aws_request_id: &str, extras: HashMap<&'static str, String>,
    ) -> Self {
        IamError {
            kind,
            message: Some(message.to_owned()),
            aws_request_id: aws_request_id.to_owned(),
            extras: Some(extras),
        }
    }
}

#[derive(Debug, Display)]
pub enum IamErrorKind {
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

impl IamErrorKind {
    pub(crate) fn status_code(&self) -> StatusCode {
        match self {
            IamErrorKind::ServiceFailure => StatusCode::INTERNAL_SERVER_ERROR,
            IamErrorKind::ServiceNotSupported => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::BAD_REQUEST,
        }
    }

    pub(crate) fn as_str(&self) -> &str {
        match self {
            IamErrorKind::ConcurrentModification => "ConcurrentModification",
            IamErrorKind::CredentialReportExpired => "CredentialReportExpired",
            IamErrorKind::CredentialReportNotPresent => "CredentialReportNotPresent",
            IamErrorKind::CredentialReportNotReady => "CredentialReportNotReady",
            IamErrorKind::DeleteConflict => "DeleteConflict",
            IamErrorKind::DuplicateCertificate => "DuplicateCertificate",
            IamErrorKind::DuplicateSshPublicKey => "DuplicateSshPublicKey",
            IamErrorKind::EntityAlreadyExists => "EntityAlreadyExists",
            IamErrorKind::EntityTemporarilyUnmodifiable => "EntityTemporarilyUnmodifiable",
            IamErrorKind::InvalidAuthenticationCode => "InvalidAuthenticationCode",
            IamErrorKind::InvalidCertificate => "InvalidCertificate",
            IamErrorKind::InvalidInput => "InvalidInput",
            IamErrorKind::InvalidPublicKey => "InvalidPublicKey",
            IamErrorKind::InvalidUserType => "InvalidUserType",
            IamErrorKind::KeyPairMismatch => "KeyPairMismatch",
            IamErrorKind::LimitExceeded => "LimitExceeded",
            IamErrorKind::MalformedCertificate => "MalformedCertificate",
            IamErrorKind::MalformedPolicyDocument => "MalformedPolicyDocument",
            IamErrorKind::NoSuchEntity => "NoSuchEntity",
            IamErrorKind::PasswordPolicyViolation => "PasswordPolicyViolation",
            IamErrorKind::PolicyEvaluation => "PolicyEvaluation",
            IamErrorKind::PolicyNotAttachable => "PolicyNotAttachable",
            IamErrorKind::ReportGenerationLimitExceeded => "ReportGenerationLimitExceeded",
            IamErrorKind::ServiceFailure => "ServiceFailure",
            IamErrorKind::ServiceNotSupported => "ServiceNotSupported",
            IamErrorKind::UnmodifiableEntity => "UnmodifiableEntity",
            IamErrorKind::UnrecognizedPublicKeyEncoding => "UnrecognizedPublicKeyEncoding",
        }
    }
}

impl Display for IamError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {})", self.kind.as_str(), self.message.as_deref().unwrap_or(""))
    }
}

impl Error for IamError {}
