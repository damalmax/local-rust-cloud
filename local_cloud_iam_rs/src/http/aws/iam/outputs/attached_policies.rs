use aws_sdk_iam::types::AttachedPolicy;
use aws_smithy_xml::encode::ScopeWriter;

use local_cloud_xml::write_tag_with_value;

pub(crate) fn write_slice(parent_tag: &mut ScopeWriter, wrapper_tag_name: &str, policies: &[AttachedPolicy]) {
    let mut policies_tag = parent_tag.start_el(wrapper_tag_name).finish();
    for policy in policies {
        write(&mut policies_tag, "member", policy);
    }
    policies_tag.finish()
}

pub(crate) fn write(parent_tag: &mut ScopeWriter, wrapper_tag_name: &str, policy: &AttachedPolicy) {
    let mut wrapper_tag = parent_tag.start_el(wrapper_tag_name).finish();
    write_tag_with_value(&mut wrapper_tag, "PolicyName", policy.policy_name());
    write_tag_with_value(&mut wrapper_tag, "PolicyArn", policy.policy_arn());
    wrapper_tag.finish();
}
