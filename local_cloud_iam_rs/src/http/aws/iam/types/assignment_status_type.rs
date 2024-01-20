#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) enum AssignmentStatusType {
    #[serde(rename = "Unassigned")]
    Unassigned,
    #[serde(rename = "Any")]
    Any,
    #[serde(rename = "Assigned")]
    Assigned,
}
