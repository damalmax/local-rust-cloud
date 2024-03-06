use sqlx::FromRow;

#[derive(Debug)]
pub(crate) struct InsertOpenIdConnectProvider {
    pub(crate) id: Option<i64>,
    pub(crate) account_id: i64,
    pub(crate) arn: String,
    pub(crate) url: String,
    pub(crate) create_date: i64,
}

#[derive(Debug, FromRow)]
pub(crate) struct SelectOpenIdConnectProvider {
    pub(crate) id: i64,
    pub(crate) account_id: i64,
    pub(crate) arn: String,
    pub(crate) url: String,
    pub(crate) create_date: i64,
}

#[derive(Debug, FromRow)]
pub(crate) struct SelectOpenIdConnectProviderClientId {
    pub(crate) id: i64,
    pub(crate) client_id: String,
}

#[derive(Debug, FromRow)]
pub(crate) struct SelectOpenIdConnectProviderThumbprint {
    pub(crate) id: i64,
    pub(crate) thumbprint: String,
}
