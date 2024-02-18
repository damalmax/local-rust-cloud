use sqlx::FromRow;

#[derive(Debug)]
pub(crate) struct InsertMfaDevice {
    pub(crate) id: Option<i64>,
    pub(crate) account_id: i64,
    pub(crate) serial_number: String,
    pub(crate) path: String,
    pub(crate) name: String,
    pub(crate) seed: Vec<u8>,
    pub(crate) create_date: i64,
}

#[derive(Debug, FromRow)]
pub(crate) struct SelectMfaDevice {
    pub(crate) id: i64,
    pub(crate) account_id: i64,
    pub(crate) serial_number: String,
    pub(crate) path: String,
    pub(crate) name: String,
    pub(crate) seed: Vec<u8>,
    pub(crate) create_date: i64,
    pub(crate) enable_date: Option<i64>,
    pub(crate) user_id: Option<i64>,
    pub(crate) user_name: Option<String>,
}

#[derive(Debug)]
pub(crate) struct EnableMfaDeviceQuery {
    pub(crate) id: i64,
    pub(crate) enable_date: i64,
    pub(crate) user_id: i64,
    pub(crate) code1: String,
    pub(crate) code2: String,
}
