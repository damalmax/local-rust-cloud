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
    #[serde(rename = "AWS")]
    pub aws: Option<FlexiString>,
    #[serde(rename = "CanonicalUser")]
    pub canonical_user: Option<String>,
    #[serde(rename = "Federated")]
    pub federated: Option<String>,
    #[serde(rename = "Service")]
    pub service: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum PrincipalData {
    Str(String),
    Principal(Principal),
}


#[derive(Debug, Validate, Deserialize)]
pub struct Condition {
    #[serde(rename = "StringEquals")]
    pub string_equals: Option<HashMap<String, FlexiString>>,
    #[serde(rename = "StringNotEquals")]
    pub string_not_equals: Option<HashMap<String, FlexiString>>,
    #[serde(rename = "StringEqualsIgnoreCase")]
    pub string_equals_ignore_case: Option<HashMap<String, FlexiString>>,
    #[serde(rename = "StringNotEqualsIgnoreCase")]
    pub string_not_equals_ignore_case: Option<HashMap<String, FlexiString>>,
    #[serde(rename = "StringLike")]
    pub string_like: Option<HashMap<String, FlexiString>>,
    #[serde(rename = "StringNotLike")]
    pub string_not_like: Option<HashMap<String, FlexiString>>,
    #[serde(rename = "NumericEquals")]
    pub numeric_equals: Option<HashMap<String, FlexiString>>,
    #[serde(rename = "NumericNotEquals")]
    pub numeric_not_equals: Option<HashMap<String, FlexiString>>,
    #[serde(rename = "NumericLessThan")]
    pub numeric_less_than: Option<HashMap<String, FlexiString>>,
    #[serde(rename = "NumericLessThanEquals")]
    pub numeric_less_than_equals: Option<HashMap<String, FlexiString>>,
    #[serde(rename = "NumericGreaterThan")]
    pub numeric_greater_than: Option<HashMap<String, FlexiString>>,
    #[serde(rename = "NumericGreaterThanEquals")]
    pub numeric_greater_than_equals: Option<HashMap<String, FlexiString>>,
    #[serde(rename = "DateEquals")]
    pub date_equals: Option<HashMap<String, FlexiString>>,
    #[serde(rename = "DateNotEquals")]
    pub date_not_equals: Option<HashMap<String, FlexiString>>,
    #[serde(rename = "DateLessThan")]
    pub date_less_than: Option<HashMap<String, FlexiString>>,
    #[serde(rename = "DateLessThanEquals")]
    pub date_less_than_equals: Option<HashMap<String, FlexiString>>,
    #[serde(rename = "DateGreaterThan")]
    pub date_greater_than: Option<HashMap<String, FlexiString>>,
    #[serde(rename = "DateGreaterThanEquals")]
    pub date_greater_than_equals: Option<HashMap<String, FlexiString>>,
    #[serde(rename = "Bool")]
    pub bool: Option<HashMap<String, FlexiString>>,
    #[serde(rename = "BinaryEquals")]
    pub binary_equals: Option<HashMap<String, FlexiString>>,
    #[serde(rename = "IpAddress")]
    pub ip_address: Option<HashMap<String, FlexiString>>,
    #[serde(rename = "NotIpAddress")]
    pub not_ip_address: Option<HashMap<String, FlexiString>>,
    #[serde(rename = "ArnEquals")]
    pub arn_equals: Option<HashMap<String, FlexiString>>,
    #[serde(rename = "ArnLike")]
    pub arn_like: Option<HashMap<String, FlexiString>>,
    #[serde(rename = "ArnNotEquals")]
    pub arn_not_equals: Option<HashMap<String, FlexiString>>,
    #[serde(rename = "ArnNotLike")]
    pub arn_not_like: Option<HashMap<String, FlexiString>>,
    #[serde(rename = "Null")]
    pub null: Option<HashMap<String, FlexiString>>,
}

#[derive(Debug, Validate, Deserialize)]
pub struct Statement {
    #[serde(rename = "Sid")]
    pub sid: Option<String>,
    #[validate(custom = "validate_principal")]
    #[serde(rename = "Principal")]
    pub principal: Option<PrincipalData>,
    #[serde(rename = "NotPrincipal")]
    pub not_principal: Option<PrincipalData>,
    #[serde(rename = "Condition")]
    pub condition: Option<Condition>,
    #[validate(length(min = 1), custom = "validate_effect")]
    #[serde(rename = "Effect")]
    pub effect: String,
    #[serde(rename = "Action")]
    pub action: Option<FlexiString>,
    #[serde(rename = "NotAction")]
    pub not_action: Option<FlexiString>,
    #[validate(custom = "validate_resource")]
    #[serde(rename = "Resource")]
    pub resource: Option<FlexiString>,
    #[serde(rename = "NotResource")]
    pub not_resource: Option<FlexiString>,
}

#[derive(Debug, Validate, Deserialize)]
pub struct LocalPolicyDocument {
    #[validate(length(min = 1), custom = "validate_version")]
    #[serde(rename = "Version")]
    pub version: String,
    #[serde(rename = "Id")]
    pub id: Option<String>,
    #[validate]
    #[serde(rename = "Statement")]
    pub statement: Vec<Statement>,
    #[validate]
    #[serde(rename = "Conditions")]
    pub conditions: Option<Vec<Condition>>,
}

fn validate_effect(effect: &str) -> Result<(), ValidationError> {
    if effect == "Allow" || effect == "Deny" {
        return Result::Ok(());
    }

    return Err(ValidationError::new("Unsupported Effect. Supported values: 'Allow', 'Deny'."));
}

fn validate_version(version: &str) -> Result<(), ValidationError> {
    if version == "2008-10-17" || version == "2012-10-17" {
        return Result::Ok(());
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
