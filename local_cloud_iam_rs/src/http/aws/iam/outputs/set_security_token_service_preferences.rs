use aws_sdk_iam::operation::set_security_token_service_preferences::SetSecurityTokenServicePreferencesOutput;

use local_cloud_actix::local::web::XmlResponse;

use crate::http::aws::iam::outputs::wrapper::OutputWrapper;

pub type LocalSetSecurityTokenServicePreferencesOutput = OutputWrapper<SetSecurityTokenServicePreferencesOutput>;

impl From<LocalSetSecurityTokenServicePreferencesOutput> for XmlResponse {
    fn from(val: LocalSetSecurityTokenServicePreferencesOutput) -> Self {
        super::confirmation::xml_response("SetSecurityTokenServicePreferencesResponse", &val.request_id)
    }
}
