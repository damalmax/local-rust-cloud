const VALID_TOP_ELEMENTS: [&'static str; 4] =["Version", "Id", "Statement", "Conditions"];
const VALIDE_VERSIONS: [&'static str; 2] =["2008-10-17", "2012-10-17"];
const VALID_STATEMENT_ELEMENTS: [&'static str; 9] =[
    "Sid",
    "Action",
    "NotAction",
    "Resource",
    "NotResource",
    "Effect",
    "Principal",
    "NotPrincipal",
    "Condition",
];
const VALID_EFFECTS: [&'static str; 2] =["Allow", "Deny"];
const VALID_CONDITIONS: [&'static str; 27] =[
    "StringEquals",
    "StringNotEquals",
    "StringEqualsIgnoreCase",
    "StringNotEqualsIgnoreCase",
    "StringLike",
    "StringNotLike",
    "NumericEquals",
    "NumericNotEquals",
    "NumericLessThan",
    "NumericLessThanEquals",
    "NumericGreaterThan",
    "NumericGreaterThanEquals",
    "DateEquals",
    "DateNotEquals",
    "DateLessThan",
    "DateLessThanEquals",
    "DateGreaterThan",
    "DateGreaterThanEquals",
    "Bool",
    "BinaryEquals",
    "IpAddress",
    "NotIpAddress",
    "ArnEquals",
    "ArnLike",
    "ArnNotEquals",
    "ArnNotLike",
    "Null",
];
const VALID_CONDITION_PREFIXES: [&'static str; 2] =["ForAnyValue:", "ForAllValues:"];
const VALID_CONDITION_POSTFIXES: [&'static str; 1] =["IfExists"];
