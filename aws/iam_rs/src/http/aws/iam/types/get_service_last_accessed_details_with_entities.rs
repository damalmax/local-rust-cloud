use crate::http::aws::iam::types;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct GetServiceLastAccessedDetailsWithEntitiesRequest {
    #[serde(rename = "Marker")]
    pub(crate) marker: Option<types::marker_type::MarkerType>,
    #[serde(rename = "ServiceNamespace")]
    pub(crate) service_namespace: Option<types::service_namespace_type::ServiceNamespaceType>,
    #[serde(rename = "JobId")]
    pub(crate) job_id: Option<types::job_id_type::JobIdType>,
    #[serde(rename = "MaxItems")]
    pub(crate) max_items: Option<types::max_items_type::MaxItemsType>,
}

impl GetServiceLastAccessedDetailsWithEntitiesRequest {
    pub(crate) fn marker(&self) -> Option<&str> {
        self.marker.as_deref()
    }
    pub(crate) fn service_namespace(&self) -> Option<&str> {
        self.service_namespace.as_deref()
    }
    pub(crate) fn job_id(&self) -> Option<&str> {
        self.job_id.as_deref()
    }
    pub(crate) fn max_items(&self) -> Option<&i32> {
        self.max_items.as_deref()
    }
}

impl validators::NamedValidator for &GetServiceLastAccessedDetailsWithEntitiesRequest {
    fn validate(&self, at: &str) -> Result<(), validators::ValidationError> {
        validators::validate_named(self.marker.as_ref(), format!("{at}.{}", "Marker").as_str())?;
        validators::validate_required(
            self.service_namespace(),
            format!("{at}.{}", "ServiceNamespace").as_str(),
        )?;
        validators::validate_named(
            self.service_namespace.as_ref(),
            format!("{at}.{}", "ServiceNamespace").as_str(),
        )?;
        validators::validate_required(self.job_id(), format!("{at}.{}", "JobId").as_str())?;
        validators::validate_named(self.job_id.as_ref(), format!("{at}.{}", "JobId").as_str())?;
        validators::validate_named(self.max_items.as_ref(), format!("{at}.{}", "MaxItems").as_str())?;
        Ok(())
    }
}
