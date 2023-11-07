use std::collections::HashMap;

use actix_web::{HttpRequest, HttpResponse};

pub trait ServiceHandler {
    fn handle(self, req: &HttpRequest, params: HashMap<String, String>) -> HttpResponse;
}
