#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) enum SortKeyType {
    #[serde(rename = "SERVICE_NAMESPACE_ASCENDING")]
    ServiceNamespaceAscending,
    #[serde(rename = "LAST_AUTHENTICATED_TIME_ASCENDING")]
    LastAuthenticatedTimeAscending,
    #[serde(rename = "SERVICE_NAMESPACE_DESCENDING")]
    ServiceNamespaceDescending,
    #[serde(rename = "LAST_AUTHENTICATED_TIME_DESCENDING")]
    LastAuthenticatedTimeDescending,
}
