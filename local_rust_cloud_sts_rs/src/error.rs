use core::fmt;

#[derive(Debug, Clone)]
pub struct ApiError {
    pub http_code: i16,
    pub error: String,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

#[allow(dead_code)]
impl ApiError {
    pub fn bad_request(msg: impl Into<String>) -> Self {
        ApiError {
            http_code: 400,
            error: msg.into(),
        }
    }

    pub fn internal_server_error(msg: impl Into<String>) -> Self {
        ApiError {
            http_code: 500,
            error: msg.into(),
        }
    }
}
