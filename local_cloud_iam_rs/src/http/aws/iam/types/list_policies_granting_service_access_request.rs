use crate::http::aws::iam::types;
#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct ListPoliciesGrantingServiceAccessRequest {
    #[serde(rename = "Marker")]
    pub(crate) marker: Option<types::marker_type::MarkerType>,
    #[serde(rename = "Arn")]
    pub(crate) arn: Option<types::arn_type::ArnType>,
    #[serde(rename = "ServiceNamespaces")]
    pub(crate) service_namespaces: Option<Vec<types::service_namespace_type::ServiceNamespaceType>>,
}
impl ListPoliciesGrantingServiceAccessRequest {
    pub(crate) fn marker(&self) -> Option<&str> {
        self.marker.as_deref()
    }
    pub(crate) fn arn(&self) -> Option<&str> {
        self.arn.as_deref()
    }
    pub(crate) fn service_namespaces(&self) -> Option<&[types::service_namespace_type::ServiceNamespaceType]> {
        self.service_namespaces.as_deref()
    }
}
impl local_cloud_validate::NamedValidator for &ListPoliciesGrantingServiceAccessRequest {
    fn validate(&self, at: &str) -> Result<(), local_cloud_validate::ValidationError> {
        local_cloud_validate::validate_named(self.marker.as_ref(), format!("{at}.{}", "Marker").as_str())?;
        local_cloud_validate::validate_required(self.arn(), format!("{at}.{}", "Arn").as_str())?;
        local_cloud_validate::validate_named(self.arn.as_ref(), format!("{at}.{}", "Arn").as_str())?;
        local_cloud_validate::validate_required(
            self.service_namespaces(),
            format!("{at}.{}", "ServiceNamespaces").as_str(),
        )?;
        local_cloud_validate::validate_array_size_min(
            self.service_namespaces(),
            1usize,
            format!("{at}.{}", "ServiceNamespaces").as_str(),
        )?;
        local_cloud_validate::validate_array_size_max(
            self.service_namespaces(),
            200usize,
            format!("{at}.{}", "ServiceNamespaces").as_str(),
        )?;
        if let Some(service_namespaces) = self.service_namespaces() {
            for (id, member) in service_namespaces.iter().enumerate() {
                local_cloud_validate::validate_named(
                    Some(member),
                    format!("{at}.{}.member.{id}", "ServiceNamespaces").as_str(),
                )?;
            }
        }
        Ok(())
    }
}
