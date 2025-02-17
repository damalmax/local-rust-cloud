use crate::http::aws::iam::types;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct GenerateOrganizationsAccessReportRequest {
    #[serde(rename = "OrganizationsPolicyId")]
    pub(crate) organizations_policy_id: Option<types::organizations_policy_id_type::OrganizationsPolicyIdType>,
    #[serde(rename = "EntityPath")]
    pub(crate) entity_path: Option<types::organizations_entity_path_type::OrganizationsEntityPathType>,
}

impl GenerateOrganizationsAccessReportRequest {
    pub(crate) fn organizations_policy_id(&self) -> Option<&str> {
        self.organizations_policy_id.as_deref()
    }
    pub(crate) fn entity_path(&self) -> Option<&str> {
        self.entity_path.as_deref()
    }
}

impl validators::NamedValidator for &GenerateOrganizationsAccessReportRequest {
    fn validate(&self, at: &str) -> Result<(), validators::ValidationError> {
        validators::validate_named(
            self.organizations_policy_id.as_ref(),
            format!("{at}.{}", "OrganizationsPolicyId").as_str(),
        )?;
        validators::validate_required(self.entity_path(), format!("{at}.{}", "EntityPath").as_str())?;
        validators::validate_named(self.entity_path.as_ref(), format!("{at}.{}", "EntityPath").as_str())?;
        Ok(())
    }
}
