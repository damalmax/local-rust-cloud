use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct RequestId(pub String);

impl Default for RequestId {
    fn default() -> Self {
        RequestId(Uuid::new_v4().to_string())
    }
}
