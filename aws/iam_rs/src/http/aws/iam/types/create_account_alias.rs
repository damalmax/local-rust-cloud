use crate::http::aws::iam::types;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub(crate) struct CreateAccountAliasRequest {
    #[serde(rename = "AccountAlias")]
    pub(crate) account_alias: Option<types::account_alias_type::AccountAliasType>,
}

impl CreateAccountAliasRequest {
    pub(crate) fn account_alias(&self) -> Option<&str> {
        self.account_alias.as_deref()
    }
}

impl validators::NamedValidator for &CreateAccountAliasRequest {
    fn validate(&self, at: &str) -> Result<(), validators::ValidationError> {
        validators::validate_required(self.account_alias(), format!("{at}.{}", "AccountAlias").as_str())?;
        validators::validate_named(self.account_alias.as_ref(), format!("{at}.{}", "AccountAlias").as_str())?;
        Ok(())
    }
}
