pub mod action;
mod assume_role;
mod constants;
mod query;
// mod assume_role_with_web_identity;
// mod get_session_token;
// mod get_federation_token;

pub struct OutputWrapper<T: Sized> {
    pub inner: T,
    pub request_id: String,
}

impl<T: Sized> OutputWrapper<T> {
    pub fn new(inner: T, request_id: String) -> OutputWrapper<T> {
        OutputWrapper { inner, request_id }
    }
}
