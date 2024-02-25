use aws_sdk_iam::operation::untag_user::UntagUserOutput;

use local_cloud_axum::local::web::XmlResponse;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalUntagUserOutput = OutputWrapper<UntagUserOutput>;

impl From<LocalUntagUserOutput> for XmlResponse {
    fn from(val: LocalUntagUserOutput) -> Self {
        super::confirmation::xml_response("UntagUserResponse", &val.request_id)
    }
}
