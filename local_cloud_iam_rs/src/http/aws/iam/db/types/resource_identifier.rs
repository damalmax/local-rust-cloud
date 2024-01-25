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
}

impl Into<i16> for ResourceType {
    fn into(self) -> i16 {
        match self {
            ResourceType::Policy => 1,
            ResourceType::PolicyVersion => 2,
            ResourceType::User => 3,
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
