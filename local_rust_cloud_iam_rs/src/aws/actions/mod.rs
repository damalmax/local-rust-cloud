pub mod action;
pub mod constants;
pub mod create_policy;
pub mod create_user;
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

    pub fn write_tags() {}
}
