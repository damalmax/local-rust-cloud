#[derive(Debug)]
pub(crate) struct ResourceIdentifier {
    pub(crate) id: Option<i64>,
    pub(crate) unique_id: String,
    pub(crate) resource_type: ResourceType,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum ResourceType {
    Policy,
    PolicyVersion,
    User,
    Role,
    Group,
    InstanceProfile,
    SamlProvider,
    SshPublicKey,
    ServerCertificate,
}

impl Into<i16> for ResourceType {
    fn into(self) -> i16 {
        match self {
            Self::Policy => 1,
            Self::PolicyVersion => 2,
            Self::User => 3,
            Self::Role => 4,
            Self::Group => 5,
            Self::InstanceProfile => 6,
            Self::SamlProvider => 7,
            Self::SshPublicKey => 8,
            Self::ServerCertificate => 9,
        }
    }
}

impl ResourceIdentifier {
    pub(crate) fn new(unique_id: &str, resource_type: ResourceType) -> ResourceIdentifier {
        ResourceIdentifier {
            id: None,
            unique_id: unique_id.to_owned(),
            resource_type,
        }
    }
}
