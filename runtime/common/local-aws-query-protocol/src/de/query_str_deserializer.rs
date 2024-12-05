use std::borrow::Cow;

use serde::de::{IntoDeserializer, Visitor};
use serde::forward_to_deserialize_any;

use crate::de::error::DeError;
use crate::de::node::Node;
use crate::de::node_deserializer::NodeDeserializer;

pub(crate) struct QueryStrDeserializer<'de>(Cow<'de, str>);

impl<'de> QueryStrDeserializer<'de> {
    pub fn new(s: Cow<'de, str>) -> Self {
        QueryStrDeserializer(s)
    }
}

macro_rules! forward_parsable_to_deserialize_any {
    ($($ty:ident => $meth:ident,)*) => {
        $(
            fn $meth<V>(self, visitor: V) -> Result<V::Value, DeError> where V: serde::de::Visitor<'de> {
                match self.0.parse::<$ty>() {
                    Ok(val) => val.into_deserializer().$meth(visitor),
                    Err(e) => Err(DeError::Custom(e.to_string()))
                }
            }
        )*
    }
}

impl<'de> serde::Deserializer<'de> for QueryStrDeserializer<'de> {
    type Error = DeError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.0.into_deserializer().deserialize_any(visitor)
    }

    fn deserialize_enum<V>(
        self, _: &'static str, _: &'static [&'static str], visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_enum(NodeDeserializer::new(Node::Flat(self.0)))
    }

    forward_to_deserialize_any! {
        byte_buf
        bytes
        char
        identifier
        ignored_any
        newtype_struct
        map
        option
        seq
        str
        string
        struct
        tuple
        tuple_struct
        unit
        unit_struct
    }

    forward_parsable_to_deserialize_any! {
        bool => deserialize_bool,
        u8 => deserialize_u8,
        u16 => deserialize_u16,
        u32 => deserialize_u32,
        u64 => deserialize_u64,
        i8 => deserialize_i8,
        i16 => deserialize_i16,
        i32 => deserialize_i32,
        i64 => deserialize_i64,
        f32 => deserialize_f32,
        f64 => deserialize_f64,
    }
}
