use aws_sdk_iam::operation::tag_instance_profile::TagInstanceProfileOutput;

use local_cloud_actix::local::web::XmlResponse;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalTagInstanceProfileOutput = OutputWrapper<TagInstanceProfileOutput>;

impl From<LocalTagInstanceProfileOutput> for XmlResponse {
    fn from(val: LocalTagInstanceProfileOutput) -> Self {
        super::confirmation::xml_response("TagInstanceProfileResponse", &val.request_id)
    }
}
