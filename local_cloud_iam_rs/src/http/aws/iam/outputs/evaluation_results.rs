use aws_sdk_iam::types::EvaluationResult;
use aws_smithy_xml::encode::ScopeWriter;

use local_cloud_xml::write_tag_with_value;

pub(crate) fn write_slice(parent_tag: &mut ScopeWriter, wrapper_tag_name: &str, items: &[EvaluationResult]) {
    let mut items_tag = parent_tag.start_el(wrapper_tag_name).finish();
    for item in items {
        write(&mut items_tag, "member", item);
    }
    items_tag.finish()
}

pub(crate) fn write(parent_tag: &mut ScopeWriter, wrapper_tag_name: &str, item: &EvaluationResult) {
    let mut wrapper_tag = parent_tag.start_el(wrapper_tag_name).finish();
    write_tag_with_value(&mut wrapper_tag, "EvalActionName", Some(item.eval_action_name()));
    write_tag_with_value(&mut wrapper_tag, "EvalResourceName", item.eval_resource_name());
    write_tag_with_value(&mut wrapper_tag, "EvalDecision", Some(item.eval_decision().as_str()));
    super::statements::write_slice(&mut wrapper_tag, "MatchedStatements", item.matched_statements());
    super::strings::write_slice(&mut wrapper_tag, "MissingContextValues", item.missing_context_values());

    if let Some(org_decision_detail) = item.organizations_decision_detail() {
        super::organizations_decision_details::write(
            &mut wrapper_tag,
            "OrganizationDecisionDetail",
            org_decision_detail,
        );
    }
    if let Some(permissions_boundary_decision_detail) = item.permissions_boundary_decision_detail() {
        super::permissions_boundary_decision_details::write(
            &mut wrapper_tag,
            "PermissionsBoundaryDecisionDetail",
            permissions_boundary_decision_detail,
        );
    }
    if let Some(eval_decision_details) = item.eval_decision_details() {
        super::policy_evalution_decision_map::write_map(&mut wrapper_tag, "EvalDecisionDetails", eval_decision_details);
    }
    super::resource_specific_results::write_slice(
        &mut wrapper_tag,
        "ResourceSpecificResults",
        item.resource_specific_results(),
    );
    wrapper_tag.finish();
}
