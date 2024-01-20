/**<p>The policy usage type that indicates whether the policy is used as a permissions policy
         or as the permissions boundary for an entity.</p>
         <p>For more information about permissions boundaries, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies_boundaries.html">Permissions boundaries for IAM
            identities </a> in the <i>IAM User Guide</i>.</p>*/
#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) enum PolicyUsageType {
    #[serde(rename = "PermissionsPolicy")]
    PermissionsPolicy,
    #[serde(rename = "PermissionsBoundary")]
    PermissionsBoundary,
}
