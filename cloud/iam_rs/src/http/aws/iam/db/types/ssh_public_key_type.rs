use crate::http::aws::iam::types::status_type::StatusType;

#[derive(Debug, Clone, sqlx::Type)]
#[repr(i32)]
pub enum SshPublicKeyStatusType {
    Active,
    Inactive,
}

impl SshPublicKeyStatusType {
    pub(crate) fn as_i32(&self) -> i32 {
        match self {
            SshPublicKeyStatusType::Active => 1,
            SshPublicKeyStatusType::Inactive => 2,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            SshPublicKeyStatusType::Active => "Active",
            SshPublicKeyStatusType::Inactive => "Inactive",
        }
    }
}

impl Into<i32> for SshPublicKeyStatusType {
    fn into(self) -> i32 {
        match self {
            Self::Active => 1,
            Self::Inactive => 2,
        }
    }
}

impl From<i32> for SshPublicKeyStatusType {
    fn from(value: i32) -> Self {
        if value == 1 {
            Self::Active
        } else {
            Self::Inactive
        }
    }
}

impl From<&StatusType> for SshPublicKeyStatusType {
    fn from(value: &StatusType) -> Self {
        match value {
            StatusType::Inactive => Self::Inactive,
            StatusType::Active => Self::Active,
        }
    }
}

impl From<&SshPublicKeyStatusType> for aws_sdk_iam::types::StatusType {
    fn from(value: &SshPublicKeyStatusType) -> Self {
        match value {
            SshPublicKeyStatusType::Active => aws_sdk_iam::types::StatusType::Active,
            SshPublicKeyStatusType::Inactive => aws_sdk_iam::types::StatusType::Inactive,
        }
    }
}
