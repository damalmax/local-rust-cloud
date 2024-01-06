use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) enum LocalPolicyUsageType {
    PermissionsPolicy,
    PermissionsBoundary,
}
