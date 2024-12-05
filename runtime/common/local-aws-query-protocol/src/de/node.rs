use std::borrow::Cow;
use std::collections::BTreeMap;

use crate::de::aws_query::QueryKey;

#[derive(Debug, PartialEq, Clone, Ord, PartialOrd, Eq)]
pub enum Node<'de> {
    Nested(BTreeMap<QueryKey<'de>, Node<'de>>),
    IndexedSeq(BTreeMap<usize, Node<'de>>),
    MapArg(BTreeMap<usize, MapEntry<'de>>),
    Flat(Cow<'de, str>),
    Invalid(String),
    Uninitialized,
}

#[derive(Debug, PartialEq, Clone, Ord, PartialOrd, Eq)]
pub struct MapEntry<'de> {
    pub key: Node<'de>,
    pub value: Node<'de>,
}

impl<'de> Node<'de> {
    pub(crate) fn flat_from(text: &str) -> Self {
        Node::Flat(Cow::from(text.to_owned()))
    }
}
