#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) enum AssignmentStatusType {
    #[serde(rename = "Assigned")]
    Assigned,
    #[serde(rename = "Unassigned")]
    Unassigned,
    #[serde(rename = "Any")]
    Any,
}
