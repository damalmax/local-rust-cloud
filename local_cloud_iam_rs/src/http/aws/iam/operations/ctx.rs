#[derive(Debug)]
pub struct OperationCtx {
    pub(crate) account_id: i64,
    pub(crate) aws_request_id: String,
}

impl OperationCtx {
    pub fn new(account_id: i64, aws_request_id: impl Into<String>) -> OperationCtx {
        OperationCtx {
            account_id,
            aws_request_id: aws_request_id.into(),
        }
    }
}
