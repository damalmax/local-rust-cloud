use std::ops::Deref;

pub struct XmlResponse(pub String);

impl Default for XmlResponse {
    fn default() -> Self {
        XmlResponse(String::new())
    }
}

impl Deref for XmlResponse {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
