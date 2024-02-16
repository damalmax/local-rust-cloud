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
