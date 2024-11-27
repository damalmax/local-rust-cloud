use aws_sdk_iam::operation::update_access_key::UpdateAccessKeyOutput;

use web::local::XmlResponse;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalUpdateAccessKeyOutput = OutputWrapper<UpdateAccessKeyOutput>;

impl From<LocalUpdateAccessKeyOutput> for XmlResponse {
    fn from(val: LocalUpdateAccessKeyOutput) -> Self {
        super::confirmation::xml_response("UpdateAccessKeyResponse", &val.request_id)
    }
}
