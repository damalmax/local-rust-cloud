#[derive(Debug, Clone, sqlx::Type)]
#[repr(i32)]
pub(crate) enum PolicyType {
    LocalCloudManaged,
    // alternative for 'AWS Managed'
    CustomerManaged,
}

impl PolicyType {
    pub(crate) fn as_i32(&self) -> i32 {
        match self {
            PolicyType::LocalCloudManaged => 1,
            PolicyType::CustomerManaged => 2,
        }
    }

    pub(crate) fn as_str(&self) -> &str {
        match self {
            Self::LocalCloudManaged => "Local Cloud Managed",
            Self::CustomerManaged => "Customer Managed",
        }
    }
}

impl Into<i32> for PolicyType {
    fn into(self) -> i32 {
        match self {
            Self::LocalCloudManaged => 1,
            Self::CustomerManaged => 2,
        }
    }
}

impl From<i32> for PolicyType {
    fn from(value: i32) -> Self {
        if value == 1 {
            Self::LocalCloudManaged
        } else {
            Self::CustomerManaged
        }
    }
}
