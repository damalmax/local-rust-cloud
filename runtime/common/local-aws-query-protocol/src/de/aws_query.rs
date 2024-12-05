use std::borrow::Cow;
use std::collections::btree_map::IntoIter;
use std::collections::BTreeMap;
use std::iter::Peekable;

use serde::de::{DeserializeSeed, EnumAccess, MapAccess, VariantAccess, Visitor};
use serde::forward_to_deserialize_any;

use crate::de::error::DeError;
use crate::de::node::Node;
use crate::de::node_deserializer::NodeDeserializer;
use crate::de::node_parser::NodeParser;
use crate::de::query_str_deserializer::QueryStrDeserializer;

/// A deserializer for the AWS Query format.
pub(crate) struct AwsQueryDeserializer<'de> {
    pub iter: Peekable<IntoIter<QueryKey<'de>, Node<'de>>>,
    pub value: Option<Node<'de>>,
}

#[derive(Debug, PartialEq, Clone, Ord, PartialOrd, Eq)]
pub(crate) enum QueryKey<'de> {
    Text(Cow<'de, str>),
    QueryNode(Node<'de>),
}

impl<'de> QueryKey<'de> {
    pub(crate) fn from_text(text: &str) -> Self {
        QueryKey::Text(Cow::from(text.to_owned()))
    }
}

impl<'de> AwsQueryDeserializer<'de> {
    pub fn new(parse: form_urlencoded::Parse<'de>) -> Result<Self, DeError> {
        NodeParser::new(parse).as_deserializer()
    }
}

impl<'de> From<BTreeMap<QueryKey<'de>, Node<'de>>> for AwsQueryDeserializer<'de> {
    fn from(map: BTreeMap<QueryKey<'de>, Node<'de>>) -> Self {
        AwsQueryDeserializer {
            iter: map.into_iter().peekable(),
            value: None,
        }
    }
}

impl<'de> serde::Deserializer<'de> for AwsQueryDeserializer<'de> {
    type Error = DeError;

    fn deserialize_any<V>(mut self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        return if self.iter.peek().is_none() {
            visitor.visit_unit()
        } else {
            self.deserialize_map(visitor)
        };
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_seq<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(DeError::RootNode("sequence".to_owned()))
    }

    fn deserialize_tuple<V>(self, _len: usize, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(DeError::RootNode("tuple".to_owned()))
    }

    fn deserialize_tuple_struct<V>(self, _name: &'static str, _len: usize, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(DeError::RootNode("tuple struct".to_owned()))
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_map(self)
    }

    forward_to_deserialize_any! {
        byte_buf
        bytes
        bool
        char
        f32
        f64
        i8
        i16
        i32
        i64
        identifier
        ignored_any
        option
        str
        string
        u8
        u16
        u32
        u64
        unit
        unit_struct
    }

    fn deserialize_struct<V>(
        self, _name: &'static str, _fields: &'static [&'static str], visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self, _name: &'static str, _variants: &'static [&'static str], visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_enum(self)
    }
}

impl<'de> EnumAccess<'de> for AwsQueryDeserializer<'de> {
    type Error = DeError;
    type Variant = Self;

    fn variant_seed<V>(mut self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        if let Some((key, value)) = self.iter.next() {
            self.value = Some(value);
            match key {
                QueryKey::Text(text) => Ok((seed.deserialize(QueryStrDeserializer::new(text))?, self)),
                QueryKey::QueryNode(node) => Ok((seed.deserialize(NodeDeserializer::new(node))?, self)),
            }
        } else {
            Err(DeError::Custom("No more values".to_owned()))
        }
    }
}

impl<'de> VariantAccess<'de> for AwsQueryDeserializer<'de> {
    type Error = DeError;
    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        if let Some(value) = self.value {
            seed.deserialize(NodeDeserializer::new(value))
        } else {
            Err(DeError::Custom("no value to deserialize".to_owned()))
        }
    }
    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let Some(value) = self.value {
            serde::de::Deserializer::deserialize_seq(NodeDeserializer::new(value), visitor)
        } else {
            Err(DeError::Custom("no value to deserialize".to_owned()))
        }
    }
    fn struct_variant<V>(self, _fields: &'static [&'static str], visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if let Some(value) = self.value {
            serde::de::Deserializer::deserialize_map(NodeDeserializer::new(value), visitor)
        } else {
            Err(DeError::Custom("no value to deserialize".to_owned()))
        }
    }
}

impl<'de> MapAccess<'de> for AwsQueryDeserializer<'de> {
    type Error = DeError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        let entry = self.iter.next();
        if let Some((key, value)) = entry {
            match key {
                QueryKey::Text(text) => {
                    self.value = Some(value);
                    seed.deserialize(QueryStrDeserializer::new(text)).map(Some)
                }
                QueryKey::QueryNode(node) => {
                    self.value = Some(value);
                    seed.deserialize(NodeDeserializer::new(node)).map(Some)
                }
            }
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        if let Some(v) = self.value.take() {
            seed.deserialize(NodeDeserializer::new(v))
        } else {
            Err(DeError::Custom("Somehow the map was empty after a non-empty key was returned".to_owned()))
        }
    }
}
