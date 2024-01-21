#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) enum EntityType {
    #[serde(rename = "User")]
    User,
    #[serde(rename = "LocalManagedPolicy")]
    LocalManagedPolicy,
    #[serde(rename = "Group")]
    Group,
    #[serde(rename = "Role")]
    Role,
    #[serde(rename = "AWSManagedPolicy")]
    AwsManagedPolicy,
}
