use crate::http::aws::iam::types::status_type::StatusType;

#[derive(Debug, Clone, sqlx::Type)]
#[repr(i32)]
pub enum SigningCertificateStatusType {
    Active,
    Inactive,
    Expired,
}

impl From<&SigningCertificateStatusType> for aws_sdk_iam::types::StatusType {
    fn from(value: &SigningCertificateStatusType) -> Self {
        match value {
            SigningCertificateStatusType::Active => aws_sdk_iam::types::StatusType::Active,
            SigningCertificateStatusType::Inactive => aws_sdk_iam::types::StatusType::Inactive,
            SigningCertificateStatusType::Expired => aws_sdk_iam::types::StatusType::Inactive,
        }
    }
}

impl SigningCertificateStatusType {
    pub(crate) fn as_i32(&self) -> i32 {
        match self {
            SigningCertificateStatusType::Active => 1,
            SigningCertificateStatusType::Inactive => 2,
            SigningCertificateStatusType::Expired => 3,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            SigningCertificateStatusType::Active => "Active",
            SigningCertificateStatusType::Inactive => "Inactive",
            SigningCertificateStatusType::Expired => "Expired",
        }
    }
}

impl Into<i32> for SigningCertificateStatusType {
    fn into(self) -> i32 {
        match self {
            Self::Active => 1,
            Self::Inactive => 2,
            Self::Expired => 3,
        }
    }
}

impl From<i32> for SigningCertificateStatusType {
    fn from(value: i32) -> Self {
        match value {
            1 => Self::Active,
            2 => Self::Inactive,
            _ => Self::Expired,
        }
    }
}

impl From<&StatusType> for SigningCertificateStatusType {
    fn from(value: &StatusType) -> Self {
        match value {
            StatusType::Inactive => Self::Inactive,
            StatusType::Active => Self::Active,
        }
    }
}
