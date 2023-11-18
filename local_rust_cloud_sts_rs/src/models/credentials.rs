use aws_smithy_types::DateTime;
use sqlx::FromRow;

#[derive(Clone, FromRow, Debug)]
pub struct Credentials {
    pub id: Option<i64>,
    pub access_key_id: String,
    pub secret_access_key: String,
    pub session_token: String,
    pub expiration: i64, // in seconds
    pub account_id: i64,
    pub region_id: i64,
}

impl Credentials {
    pub fn builder() -> crate::models::credentials::CredentialsBuilder {
        crate::models::credentials::CredentialsBuilder::default()
    }

    pub fn as_aws(&self) -> aws_sdk_sts::types::Credentials {
        aws_sdk_sts::types::Credentials::builder()
            .access_key_id(&self.access_key_id)
            .secret_access_key(&self.secret_access_key)
            .session_token(&self.session_token)
            .expiration(DateTime::from_secs(self.expiration))
            .build()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
pub struct CredentialsBuilder {
    pub(crate) id: Option<i64>,
    pub(crate) access_key_id: Option<String>,
    pub(crate) secret_access_key: Option<String>,
    pub(crate) session_token: Option<String>,
    pub(crate) expiration: Option<i64>,
    pub(crate) account_id: Option<i64>,
    pub(crate) region_id: Option<i64>,
}

impl CredentialsBuilder {
    /// <p>The access key ID that identifies the temporary security credentials.</p>
    pub fn access_key_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.access_key_id = Some(input.into());
        self
    }

    /// <p>The secret access key that can be used to sign requests.</p>
    pub fn secret_access_key(mut self, input: impl Into<std::string::String>) -> Self {
        self.secret_access_key = Some(input.into());
        self
    }

    /// <p>The token that users must pass to the service API to use the temporary credentials.</p>
    pub fn session_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.session_token = Some(input.into());
        self
    }

    /// <p>The date on which the current credentials expire.</p>
    pub fn expiration(mut self, input: i64) -> Self {
        self.expiration = Some(input);
        self
    }

    pub fn account_id(mut self, input: i64) -> Self {
        self.account_id = Some(input);
        self
    }

    pub fn region_id(mut self, input: i64) -> Self {
        self.region_id = Some(input);
        self
    }

    /// Consumes the builder and constructs a [`Credentials`](crate::types::Credentials).
    pub fn build(self) -> crate::models::Credentials {
        crate::models::credentials::Credentials {
            id: self.id,
            access_key_id: self.access_key_id.unwrap(),
            secret_access_key: self.secret_access_key.unwrap(),
            session_token: self.session_token.unwrap(),
            expiration: self.expiration.unwrap(),
            account_id: self.account_id.unwrap(),
            region_id: self.region_id.unwrap(),
        }
    }
}
