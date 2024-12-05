use std::borrow::Cow;
use std::collections::BTreeMap;

use log::trace;
use serde::de::{DeserializeSeed, EnumAccess, VariantAccess, Visitor};
use serde::forward_to_deserialize_any;

use crate::de::aws_query::{AwsQueryDeserializer, QueryKey};
use crate::de::error::DeError;
use crate::de::node::Node;
use crate::de::node_seq::NodeSeq;
use crate::de::query_str_deserializer::QueryStrDeserializer;

pub(crate) struct NodeDeserializer<'de>(Node<'de>);

impl<'de> NodeDeserializer<'de> {
    fn into_deserializer(self) -> Result<AwsQueryDeserializer<'de>, DeError> {
        match self.0 {
            Node::Nested(map) => Ok(AwsQueryDeserializer::from(map)),
            Node::IndexedSeq(map) => Ok(AwsQueryDeserializer::from(BTreeMap::from_iter(
                map.into_iter()
                    .map(|(key, value)| (QueryKey::from_text(&key.to_string()), value)),
            ))),
            Node::MapArg(map) => Ok(AwsQueryDeserializer::from(BTreeMap::from_iter(
                map.into_values()
                    .map(|entry| (QueryKey::QueryNode(entry.key), entry.value)),
            ))),
            Node::Invalid(e) => Err(DeError::Custom(e)),
            node => Err(DeError::Internal(format!("could not convert {:?} to AwsQueryDeserializer<'de>", node))),
        }
    }

    pub fn new(node: Node<'de>) -> Self {
        NodeDeserializer(node)
    }
}

fn validate<T>(map: &BTreeMap<usize, T>, msg: &str) -> Result<(), DeError> {
    let map_len = map.len();
    for index in 1..(map_len + 1) {
        if !map.contains_key(&index) {
            return Err(DeError::InvalidSource(msg.to_owned()));
        }
    }
    Ok(())
}

macro_rules! deserialize_primitive {
    ($ty:ident, $method:ident, $visit_method:ident) => {
        fn $method<V>(self, visitor: V) -> Result<V::Value, DeError>
        where
            V: serde::de::Visitor<'de>,
        {
            match self.0 {
                Node::Nested(_) => Err(DeError::Custom(format!("Expected: {:?}, got a Map", stringify!($ty)))),
                Node::IndexedSeq(_) => {
                    Err(DeError::Custom(format!("Expected: {:?}, got an IndexedSeq", stringify!($ty))))
                }
                Node::Flat(x) => QueryStrDeserializer::new(x).$method(visitor),
                Node::MapArg(_arg) => Err(DeError::Custom(format!("Expected: {:?}, got a MapArg", stringify!($ty)))),
                Node::Invalid(e) => Err(DeError::Custom(e)),
                Node::Uninitialized => Err(DeError::Custom("attempted to deserialize uninitialized value".to_owned())),
            }
        }
    };
}

impl<'de> serde::Deserializer<'de> for NodeDeserializer<'de> {
    type Error = DeError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.0 {
            Node::Nested(_) => self.into_deserializer()?.deserialize_map(visitor),
            Node::IndexedSeq(map) => {
                trace!("IndexedSeq: {:?}", map);
                validate(&map, "indexed sequence doesn't have all expected keys")?;
                visitor.visit_seq(NodeSeq::new(map.into_values()))
            }
            Node::Flat(x) => {
                trace!("Flat: {x}");
                match x {
                    Cow::Owned(s) => visitor.visit_string(s),
                    Cow::Borrowed(s) => visitor.visit_borrowed_str(s),
                }
            }
            Node::Invalid(e) => Err(DeError::Custom(e)),
            Node::Uninitialized => Err(DeError::Custom("attempted to deserialize uninitialized value".to_owned())),
            Node::MapArg(ref map) => {
                trace!("MapArg: {:?}", map);
                validate(map, "map definition doesn't have all expected entries")?;
                self.into_deserializer()?.deserialize_map(visitor)
            }
        }
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.0 {
            Node::Flat(ref x) if x == "" => visitor.visit_none(),
            _ => visitor.visit_some(self),
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.0 {
            Node::Flat(ref x) if x == "" => visitor.visit_unit(),
            _ => Err(DeError::Custom("expected unit: deserialize_unit".to_owned())),
        }
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.0 {
            Node::Nested(_) => self.into_deserializer()?.deserialize_map(visitor),
            Node::IndexedSeq(map) => visitor.visit_seq(NodeSeq::new(map.into_values())),
            Node::Flat(_) => {
                // For a newtype_struct, attempt to deserialize a flat value as a
                // single element sequence.
                visitor.visit_seq(NodeSeq::new(vec![self.0].into_iter()))
            }
            Node::Invalid(e) => Err(DeError::Custom(e)),
            Node::Uninitialized => Err(DeError::Custom("attempted to deserialize an uninitialized value".to_owned())),
            Node::MapArg(map) => {
                Err(DeError::Internal(format!("unexpected newtype deserializer called. MapArg: {:?}", map)))
            }
        }
    }

    /// given the hint that this is a map, will first
    /// attempt to deserialize ordered sequences into a map
    /// otherwise, follows the any code path
    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.0 {
            Node::IndexedSeq(_) => self.into_deserializer()?.deserialize_map(visitor),
            _ => self.deserialize_any(visitor),
        }
    }

    fn deserialize_enum<V>(
        self, name: &'static str, variants: &'static [&'static str], visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.0 {
            Node::Nested(map) => AwsQueryDeserializer::from(map).deserialize_enum(name, variants, visitor),
            Node::Flat(_) => visitor.visit_enum(self),
            x => Err(DeError::Custom(format!("{:?} does not appear to be an enum", x))),
        }
    }

    deserialize_primitive!(bool, deserialize_bool, visit_bool);
    deserialize_primitive!(i8, deserialize_i8, visit_i8);
    deserialize_primitive!(i16, deserialize_i16, visit_i16);
    deserialize_primitive!(i32, deserialize_i32, visit_i32);
    deserialize_primitive!(i64, deserialize_i64, visit_i64);
    deserialize_primitive!(u8, deserialize_u8, visit_u8);
    deserialize_primitive!(u16, deserialize_u16, visit_u16);
    deserialize_primitive!(u32, deserialize_u32, visit_u32);
    deserialize_primitive!(u64, deserialize_u64, visit_u64);
    deserialize_primitive!(f32, deserialize_f32, visit_f32);
    deserialize_primitive!(f64, deserialize_f64, visit_f64);

    forward_to_deserialize_any! {
        char
        str
        string
        bytes
        byte_buf
        unit_struct
        tuple_struct
        struct
        identifier
        tuple
        ignored_any
        seq
    }
}

impl<'de> EnumAccess<'de> for NodeDeserializer<'de> {
    type Error = DeError;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        match self.0 {
            Node::Flat(value) => Ok((
                seed.deserialize(QueryStrDeserializer::new(value))?,
                NodeDeserializer(Node::Invalid("this value can only deserialize to a UnitVariant".to_string())),
            )),
            _ => Err(DeError::Custom("this value can only deserialize to a UnitVariant".to_owned())),
        }
    }
}

impl<'de> VariantAccess<'de> for NodeDeserializer<'de> {
    type Error = DeError;
    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        seed.deserialize(self)
    }
    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        serde::de::Deserializer::deserialize_seq(self, visitor)
    }
    fn struct_variant<V>(self, _fields: &'static [&'static str], visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        serde::de::Deserializer::deserialize_map(self, visitor)
    }
}
