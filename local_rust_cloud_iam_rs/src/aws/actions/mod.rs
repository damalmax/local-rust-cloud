pub mod action;
pub mod constants;
pub mod create_policy_action;
pub mod create_policy_request;
pub mod create_policy_response;
pub mod create_user;
pub mod create_user_response;
pub mod errors;
mod query;
pub mod response;
pub mod validators;

pub struct OutputWrapper<T: Sized> {
    pub inner: T,
    pub request_id: String,
}

impl<T: Sized> OutputWrapper<T> {
    pub fn new(inner: T, request_id: impl Into<String>) -> OutputWrapper<T> {
        OutputWrapper {
            inner,
            request_id: request_id.into(),
        }
    }
}
