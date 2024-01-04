#[derive(Debug, Clone)]
pub(crate) enum PolicyType {
    LocalCloudManaged, // alternative for 'AWS Managed'
    CustomerManaged,
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

impl Into<&'static str> for PolicyType {
    fn into(self) -> &'static str {
        match self {
            Self::LocalCloudManaged => "Local Cloud Managed",
            Self::CustomerManaged => "Customer Managed",
        }
    }
}
