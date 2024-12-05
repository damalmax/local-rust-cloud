use serde::de::{DeserializeSeed, SeqAccess};

use crate::de::error::DeError;
use crate::de::node::Node;
use crate::de::node_deserializer::NodeDeserializer;

pub(crate) struct NodeSeq<'de, I>(I)
where
    I: Iterator<Item = Node<'de>>;

impl<'de, I> NodeSeq<'de, I>
where
    I: Iterator<Item = Node<'de>>,
{
    pub fn new(iter: I) -> Self {
        NodeSeq(iter)
    }
}

impl<'de, I: Iterator<Item = Node<'de>>> SeqAccess<'de> for NodeSeq<'de, I> {
    type Error = DeError;
    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        if let Some(v) = self.0.next() {
            seed.deserialize(NodeDeserializer::new(v)).map(Some)
        } else {
            Ok(None)
        }
    }
}
