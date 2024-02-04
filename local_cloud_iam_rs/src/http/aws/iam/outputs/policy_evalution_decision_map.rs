use std::collections::HashMap;

use aws_sdk_iam::types::PolicyEvaluationDecisionType;
use aws_smithy_xml::encode::ScopeWriter;
use local_cloud_xml::write_tag_with_value;

pub(crate) fn write_map(
    parent_tag: &mut ScopeWriter, wrapper_tag_name: &str, map: &HashMap<String, PolicyEvaluationDecisionType>,
) {
    let mut items_tag = parent_tag.start_el(wrapper_tag_name).finish();
    for (key, decision_type) in map.into_iter() {
        write(&mut items_tag, "entry", key, decision_type);
    }
    items_tag.finish()
}

pub(crate) fn write(
    parent_tag: &mut ScopeWriter, wrapper_tag_name: &str, key: &str, value: &PolicyEvaluationDecisionType,
) {
    let mut wrapper_tag = parent_tag.start_el(wrapper_tag_name).finish();
    write_tag_with_value(&mut wrapper_tag, "key", Some(key));
    write_tag_with_value(&mut wrapper_tag, "value", Some(value.as_str()));
    wrapper_tag.finish();
}
