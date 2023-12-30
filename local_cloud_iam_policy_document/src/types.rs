use std::collections::HashMap;

use serde::{Deserialize, Serialize};
// A trait that the Validate derive will impl
use validator::{Validate, ValidationError};

const VALID_CONDITION_PREFIXES: [&'static str; 2] = ["ForAnyValue:", "ForAllValues:"];
const VALID_CONDITION_POSTFIXES: [&'static str; 1] = ["IfExists"];

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum FlexiString {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct Principal {
    #[serde(rename = "AWS", skip_serializing_if = "Option::is_none")]
    pub aws: Option<FlexiString>,
    #[serde(rename = "CanonicalUser", skip_serializing_if = "Option::is_none")]
    pub canonical_user: Option<FlexiString>,
    #[serde(rename = "Federated", skip_serializing_if = "Option::is_none")]
    pub federated: Option<FlexiString>,
    #[serde(rename = "Service", skip_serializing_if = "Option::is_none")]
    pub service: Option<FlexiString>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum PrincipalData {
    Str(String),
    Principal(Principal),
}

#[derive(Debug, Validate, Serialize, Deserialize)]
pub struct Condition {
    #[serde(rename = "StringEquals", skip_serializing_if = "Option::is_none")]
    pub string_equals: Option<HashMap<String, FlexiString>>,
    #[serde(rename = "StringNotEquals", skip_serializing_if = "Option::is_none")]
    pub string_not_equals: Option<HashMap<String, FlexiString>>,
    #[serde(rename = "StringEqualsIgnoreCase", skip_serializing_if = "Option::is_none")]
    pub string_equals_ignore_case: Option<HashMap<String, FlexiString>>,
    #[serde(rename = "StringNotEqualsIgnoreCase", skip_serializing_if = "Option::is_none")]
    pub string_not_equals_ignore_case: Option<HashMap<String, FlexiString>>,
    #[serde(rename = "StringLike", skip_serializing_if = "Option::is_none")]
    pub string_like: Option<HashMap<String, FlexiString>>,
    #[serde(rename = "StringNotLike", skip_serializing_if = "Option::is_none")]
    pub string_not_like: Option<HashMap<String, FlexiString>>,
    #[serde(rename = "NumericEquals", skip_serializing_if = "Option::is_none")]
    pub numeric_equals: Option<HashMap<String, FlexiString>>,
    #[serde(rename = "NumericNotEquals", skip_serializing_if = "Option::is_none")]
    pub numeric_not_equals: Option<HashMap<String, FlexiString>>,
    #[serde(rename = "NumericLessThan", skip_serializing_if = "Option::is_none")]
    pub numeric_less_than: Option<HashMap<String, FlexiString>>,
    #[serde(rename = "NumericLessThanEquals", skip_serializing_if = "Option::is_none")]
    pub numeric_less_than_equals: Option<HashMap<String, FlexiString>>,
    #[serde(rename = "NumericGreaterThan", skip_serializing_if = "Option::is_none")]
    pub numeric_greater_than: Option<HashMap<String, FlexiString>>,
    #[serde(rename = "NumericGreaterThanEquals", skip_serializing_if = "Option::is_none")]
    pub numeric_greater_than_equals: Option<HashMap<String, FlexiString>>,
    #[serde(rename = "DateEquals", skip_serializing_if = "Option::is_none")]
    pub date_equals: Option<HashMap<String, FlexiString>>,
    #[serde(rename = "DateNotEquals", skip_serializing_if = "Option::is_none")]
    pub date_not_equals: Option<HashMap<String, FlexiString>>,
    #[serde(rename = "DateLessThan", skip_serializing_if = "Option::is_none")]
    pub date_less_than: Option<HashMap<String, FlexiString>>,
    #[serde(rename = "DateLessThanEquals", skip_serializing_if = "Option::is_none")]
    pub date_less_than_equals: Option<HashMap<String, FlexiString>>,
    #[serde(rename = "DateGreaterThan", skip_serializing_if = "Option::is_none")]
    pub date_greater_than: Option<HashMap<String, FlexiString>>,
    #[serde(rename = "DateGreaterThanEquals", skip_serializing_if = "Option::is_none")]
    pub date_greater_than_equals: Option<HashMap<String, FlexiString>>,
    #[serde(rename = "Bool", skip_serializing_if = "Option::is_none")]
    pub bool: Option<HashMap<String, FlexiString>>,
    #[serde(rename = "BinaryEquals", skip_serializing_if = "Option::is_none")]
    pub binary_equals: Option<HashMap<String, FlexiString>>,
    #[serde(rename = "IpAddress", skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<HashMap<String, FlexiString>>,
    #[serde(rename = "NotIpAddress", skip_serializing_if = "Option::is_none")]
    pub not_ip_address: Option<HashMap<String, FlexiString>>,
    #[serde(rename = "ArnEquals", skip_serializing_if = "Option::is_none")]
    pub arn_equals: Option<HashMap<String, FlexiString>>,
    #[serde(rename = "ArnLike", skip_serializing_if = "Option::is_none")]
    pub arn_like: Option<HashMap<String, FlexiString>>,
    #[serde(rename = "ArnNotEquals", skip_serializing_if = "Option::is_none")]
    pub arn_not_equals: Option<HashMap<String, FlexiString>>,
    #[serde(rename = "ArnNotLike", skip_serializing_if = "Option::is_none")]
    pub arn_not_like: Option<HashMap<String, FlexiString>>,
    #[serde(rename = "Null", skip_serializing_if = "Option::is_none")]
    pub null: Option<HashMap<String, FlexiString>>,
}

#[derive(Debug, Validate, Serialize, Deserialize)]
pub struct Statement {
    #[serde(rename = "Sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    #[validate(custom = "validate_principal")]
    #[serde(rename = "Principal", skip_serializing_if = "Option::is_none")]
    pub principal: Option<PrincipalData>,
    #[serde(rename = "NotPrincipal", skip_serializing_if = "Option::is_none")]
    pub not_principal: Option<PrincipalData>,
    #[serde(rename = "Condition", skip_serializing_if = "Option::is_none")]
    pub condition: Option<Condition>,
    #[validate(length(min = 1), custom = "validate_effect")]
    #[serde(rename = "Effect")]
    pub effect: String,
    #[serde(rename = "Action", skip_serializing_if = "Option::is_none")]
    pub action: Option<FlexiString>,
    #[serde(rename = "NotAction", skip_serializing_if = "Option::is_none")]
    pub not_action: Option<FlexiString>,
    #[validate(custom = "validate_resource")]
    #[serde(rename = "Resource", skip_serializing_if = "Option::is_none")]
    pub resource: Option<FlexiString>,
    #[serde(rename = "NotResource", skip_serializing_if = "Option::is_none")]
    pub not_resource: Option<FlexiString>,
}

#[derive(Debug, Validate, Serialize, Deserialize)]
pub struct LocalPolicyDocument {
    #[validate(length(min = 1), custom = "validate_version")]
    #[serde(rename = "Version")]
    pub version: String,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[validate]
    #[serde(rename = "Statement")]
    pub statement: Vec<Statement>,
    #[validate]
    #[serde(rename = "Conditions", skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
}

fn validate_effect(effect: &str) -> Result<(), ValidationError> {
    if effect == "Allow" || effect == "Deny" {
        Ok(())
    } else {
        Err(ValidationError::new("Unsupported Effect. Supported values: 'Allow', 'Deny'."))
    }
}

fn validate_version(version: &str) -> Result<(), ValidationError> {
    if version == "2008-10-17" || version == "2012-10-17" {
        return Ok(());
    }

    return Err(ValidationError::new("Unsupported Version. Supported values: '2008-10-17', '2012-10-17'."));
}

fn validate_resource(resource: &FlexiString) -> Result<(), ValidationError> {
    Result::Ok(())
}

fn validate_principal(principal: &PrincipalData) -> Result<(), ValidationError> {
    Result::Ok(())
}

#[cfg(test)]
mod tests {
    use validator::Validate;

    use crate::types::LocalPolicyDocument;

    #[test]
    fn test_policy_document_version_2008_10_17() {
        let policy_document_json = r#"
        {
            "Version": "2012-10-17",
            "Statement": [
                {
                    "Effect": "Allow",
                    "Action": [
                        "ec2:*"
                    ],
                    "Resource": "*"
                }
            ]
        }"#;
        let policy_document: LocalPolicyDocument = serde_json::from_str(policy_document_json).unwrap();
        let validation_result = policy_document.validate();
        assert!(validation_result.is_ok());
    }

    #[test]
    fn test_policy_document_version_2012_10_17() {
        let policy_document_json = r#"{
            "Version": "2012-10-17",
            "Statement": [
                {
                    "Effect": "Allow",
                    "Action": [
                        "s3:*",
                        "cloudwatch:*"
                    ],
                    "Resource": "*"
                }
            ]
        }"#;
        let policy_document: LocalPolicyDocument = serde_json::from_str(policy_document_json).unwrap();
        let validation_result = policy_document.validate();
        assert!(validation_result.is_ok());
    }

    #[test]
    fn test_policy_document_invalid_version() {
        let policy_document_json = r#"{
            "Version": "2023-01-01",
            "Statement": [
                {
                    "Effect": "Allow",
                    "Action": [
                        "cloudwatch:*"
                    ],
                    "Resource": "*"
                }
            ]
        }"#;
        let policy_document: LocalPolicyDocument = serde_json::from_str(policy_document_json).unwrap();
        let validation_result = policy_document.validate();
        assert!(validation_result.is_err());
    }

    #[test]
    fn test_policy_document_effect_allow() {
        let policy_document_json = r#"{
            "Version": "2012-10-17",
            "Statement": [
                {
                    "Effect": "Allow",
                    "Action": [
                        "s3:*"
                    ],
                    "Resource": "*"
                }
            ]
        }"#;
        let policy_document: LocalPolicyDocument = serde_json::from_str(policy_document_json).unwrap();
        let validation_result = policy_document.validate();
        assert!(validation_result.is_ok());
    }

    #[test]
    fn test_policy_document_effect_deny() {
        let policy_document_json = r#"{
            "Version": "2012-10-17",
            "Statement": [
                {
                    "Effect": "Deny",
                    "Action": [
                        "s3:*"
                    ],
                    "Resource": "*"
                }
            ]
        }"#;
        let policy_document: LocalPolicyDocument = serde_json::from_str(policy_document_json).unwrap();
        let validation_result = policy_document.validate();
        assert!(validation_result.is_ok());
    }

    #[test]
    fn test_policy_document_invalid_effect() {
        let policy_document_json = r#"{
            "Version": "2012-10-17",
            "Statement": [
                {
                    "Effect": "Apply",
                    "Action": [
                        "s3:*"
                    ],
                    "Resource": "*"
                }
            ]
        }"#;
        let policy_document: LocalPolicyDocument = serde_json::from_str(policy_document_json).unwrap();
        let validation_result = policy_document.validate();
        assert!(validation_result.is_err());
    }

    #[test]
    fn test_policy_document_condition() {
        let policy_document_json = r#"
        {
            "Version": "2012-10-17",
            "Statement": [
              {
                "Effect": "Allow",
                "Action": [
                  "s3:ListAllMyBuckets",
                  "s3:GetBucketLocation"
                ],
                "Resource": "arn:aws:s3:::*"
              },
              {
                "Effect": "Allow",
                "Action": "s3:ListBucket",
                "Resource": "arn:aws:s3:::BUCKET-NAME",
                "Condition": {"StringLike": {"s3:prefix": [
                  "",
                  "home/",
                  "home/${aws:username}/"
                ]}}
              },
              {
                "Effect": "Allow",
                "Action": "s3:*",
                "Resource": [
                  "arn:aws:s3:::BUCKET-NAME/home/${aws:username}",
                  "arn:aws:s3:::BUCKET-NAME/home/${aws:username}/*"
                ]
              }
            ]
          }
        "#;
        let policy_document: LocalPolicyDocument = serde_json::from_str(policy_document_json).unwrap();
        let validation_result = policy_document.validate();
        assert!(validation_result.is_ok());
    }
}
