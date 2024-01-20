#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) enum EntityType {
    #[serde(rename = "AWSManagedPolicy")]
    AwsManagedPolicy,
    #[serde(rename = "Role")]
    Role,
    #[serde(rename = "User")]
    User,
    #[serde(rename = "Group")]
    Group,
    #[serde(rename = "LocalManagedPolicy")]
    LocalManagedPolicy,
}
