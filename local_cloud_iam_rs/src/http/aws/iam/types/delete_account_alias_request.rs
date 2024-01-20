use crate::http::aws::iam::types;
#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct DeleteAccountAliasRequest {
    #[serde(rename = "AccountAlias")]
    pub(crate) account_alias: Option<types::account_alias_type::AccountAliasType>,
}
impl DeleteAccountAliasRequest {
    pub(crate) fn account_alias(&self) -> Option<&str> {
        self.account_alias.as_deref()
    }
}
impl local_cloud_validate::NamedValidator for &DeleteAccountAliasRequest {
    fn validate(&self, at: &str) -> Result<(), local_cloud_validate::ValidationError> {
        local_cloud_validate::validate_required(
            self.account_alias(),
            format!("{at}.{}", "AccountAlias").as_str(),
        )?;
        local_cloud_validate::validate_named(
            self.account_alias.as_ref(),
            format!("{at}.{}", "AccountAlias").as_str(),
        )?;
        Ok(())
    }
}
