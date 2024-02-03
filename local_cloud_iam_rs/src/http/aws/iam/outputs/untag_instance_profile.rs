use aws_sdk_iam::operation::untag_instance_profile::UntagInstanceProfileOutput;

use local_cloud_actix::local::web::XmlResponse;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalUntagInstanceProfileOutput = OutputWrapper<UntagInstanceProfileOutput>;

impl From<LocalUntagInstanceProfileOutput> for XmlResponse {
    fn from(val: LocalUntagInstanceProfileOutput) -> Self {
        super::confirmation::xml_response("UntagInstanceProfileResponse", &val.request_id)
    }
}
