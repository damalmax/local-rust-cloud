#[derive(Debug, Clone, sqlx::Type)]
#[repr(i32)]
pub enum SigningCertificateStatusType {
    Active,
    Inactive,
    Expired,
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
