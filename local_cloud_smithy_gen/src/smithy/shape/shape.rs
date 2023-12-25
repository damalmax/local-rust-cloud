use serde::{Deserialize, Serialize};

use crate::smithy::shape::service::ServiceShape;

use super::{
    blob::BlobShape, boolean::BooleanShape, double::DoubleShape, enum_shape::EnumShape, float::FloatShape, integer::IntegerShape,
    list::ListShape, long::LongShape, map::MapShape, operation::OperationShape, string::StringShape, structure::StructureShape,
    union_shape::UnionShape,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum Shape {
    #[serde(rename = "service")]
    Service(ServiceShape),
    #[serde(rename = "structure")]
    Structure(StructureShape),
    #[serde(rename = "enum")]
    Enum(EnumShape),
    #[serde(rename = "list")]
    List(ListShape),
    #[serde(rename = "string")]
    String(StringShape),
    #[serde(rename = "operation")]
    Operation(OperationShape),
    #[serde(rename = "blob")]
    Blob(BlobShape),
    #[serde(rename = "map")]
    Map(MapShape),
    #[serde(rename = "integer")]
    Integer(IntegerShape),
    #[serde(rename = "double")]
    Double(DoubleShape),
    #[serde(rename = "boolean")]
    Boolean(BooleanShape),
    #[serde(rename = "timestamp")]
    Timestamp,
    #[serde(rename = "long")]
    Long(LongShape),
    #[serde(rename = "float")]
    Float(FloatShape),
    #[serde(rename = "union")]
    Union(UnionShape),
}
