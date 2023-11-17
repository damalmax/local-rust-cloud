pub mod action;
pub mod create_policy;
pub mod create_user;
pub mod constants;
mod query;


pub struct OutputWrapper<T: Sized> {
    pub inner: T,
    pub request_id: String,
}

impl<T: Sized> OutputWrapper<T> {
    pub fn new(inner: T, request_id: String) -> OutputWrapper<T> {
        OutputWrapper { inner, request_id }
    }

    pub fn write_tags() {
        
    }
}
