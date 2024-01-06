use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) enum LocalPolicyScopeType {
    All,
    AWS,
    Local,
}
