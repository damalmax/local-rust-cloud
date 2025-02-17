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

impl validators::NamedValidator for &ListPoliciesGrantingServiceAccessRequest {
    fn validate(&self, at: &str) -> Result<(), validators::ValidationError> {
        validators::validate_named(self.marker.as_ref(), format!("{at}.{}", "Marker").as_str())?;
        validators::validate_required(self.arn(), format!("{at}.{}", "Arn").as_str())?;
        validators::validate_named(self.arn.as_ref(), format!("{at}.{}", "Arn").as_str())?;
        validators::validate_required(
            self.service_namespaces(),
            format!("{at}.{}", "ServiceNamespaces").as_str(),
        )?;
        validators::validate_array_size_min(
            self.service_namespaces(),
            1usize,
            format!("{at}.{}", "ServiceNamespaces").as_str(),
        )?;
        validators::validate_array_size_max(
            self.service_namespaces(),
            200usize,
            format!("{at}.{}", "ServiceNamespaces").as_str(),
        )?;
        if let Some(service_namespaces) = self.service_namespaces() {
            for (id, member) in service_namespaces.iter().enumerate() {
                validators::validate_named(
                    Some(member),
                    format!("{at}.{}.member.{id}", "ServiceNamespaces").as_str(),
                )?;
            }
        }
        Ok(())
    }
}
