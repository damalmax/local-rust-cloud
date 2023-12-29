pub struct OutputWrapper<T: Sized> {
    pub inner: T,
    pub request_id: String,
}

impl<T: Sized> OutputWrapper<T> {
    pub fn new(inner: T, request_id: impl Into<String>) -> Self {
        OutputWrapper {
            inner,
            request_id: request_id.into(),
        }
    }
}
