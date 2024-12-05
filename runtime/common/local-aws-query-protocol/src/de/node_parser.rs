use std::collections::{btree_map, BTreeMap};

use log::trace;

use crate::de::aws_query::{AwsQueryDeserializer, QueryKey};
use crate::de::error::DeError;
use crate::de::node::{MapEntry, Node};
use crate::ext::StringExt;

pub(crate) struct NodeParser<'de>(form_urlencoded::Parse<'de>);

const KEY_DELIMITER: char = '.';
const SEQUENCE_MEMBER_KEY: &str = "member";
const MAP_ENTRY_KEY: &str = "key";
const MAP_ENTRY_VALUE: &str = "value";
const MAP_ENTRY: &str = "entry";

impl<'de> NodeParser<'de> {
    pub fn new(parse: form_urlencoded::Parse<'de>) -> NodeParser<'de> {
        NodeParser(parse)
    }

    pub fn as_deserializer(&mut self) -> Result<AwsQueryDeserializer<'de>, DeError> {
        let map = BTreeMap::default();
        let mut root = Node::Nested(map);

        // Parses all top level nodes into the `root` map.
        self.parse(&mut root)?;
        let iter = match root {
            Node::Nested(map) => map.into_iter(),
            _ => BTreeMap::default().into_iter(),
        };
        Ok(AwsQueryDeserializer {
            iter: iter.peekable(),
            value: None,
        })
    }

    fn parse_indexed_seq_first_time(&mut self, parts: &[&str], value: &str) -> Result<Node<'de>, DeError> {
        let mut map = BTreeMap::default();
        let (index, parts_tail) = if parts[0] == SEQUENCE_MEMBER_KEY {
            (parts[1].parse().unwrap(), &parts[2..])
        } else {
            (parts[0].parse().unwrap(), &parts[1..])
        };
        map.insert(index, self.parse_with_split_key(parts_tail, value)?);
        Ok(Node::IndexedSeq(map))
    }

    fn update_indexed_seq(
        &mut self, seq_map: &mut BTreeMap<usize, Node<'de>>, parts: &[&str], value: &str,
    ) -> Result<(), DeError> {
        let (index, parts_tail) = if parts[0] == SEQUENCE_MEMBER_KEY {
            (parts[1].parse::<usize>().unwrap(), &parts[2..])
        } else {
            (parts[0].parse::<usize>().unwrap(), &parts[1..])
        };
        return if let btree_map::Entry::Vacant(map_entry) = seq_map.entry(index) {
            map_entry.insert(self.parse_with_split_key(parts_tail, value)?);
            Ok(())
        } else {
            let node = seq_map.get_mut(&index).unwrap();
            self.parse_with_complex_key(node, parts_tail, value)
        };
    }

    fn parse_map_entry_first_time(&mut self, parts: &[&str], value: &str) -> Result<MapEntry<'de>, DeError> {
        trace!("key: {:?}, value: {:?}", parts, value);
        let (key, value) = if parts[0] == MAP_ENTRY_KEY {
            (self.parse_with_split_key(&parts[1..], value)?, Node::Uninitialized)
        } else {
            (Node::Uninitialized, self.parse_with_split_key(&parts[1..], value)?)
        };
        Ok(MapEntry { key, value })
    }

    fn merge_map_entry_item(&mut self, node: &mut Node<'de>, parts: &[&str], value: &str) -> Result<(), DeError> {
        match node {
            Node::Nested(ref mut inner_map) => {
                trace!("inner_map: {:?}", inner_map);
                self.parse_into_map(inner_map, parts[0], &parts[1..], value)
            }
            node => Err(DeError::InvalidSource(format!(
                "unexpected node type while merging map entry argument. node={:?}",
                node
            ))),
        }
    }

    fn update_map_entry(
        &mut self, node_map_entry: &mut MapEntry<'de>, parts: &[&str], value: &str,
    ) -> Result<(), DeError> {
        trace!("node_map_entry: {:?}", node_map_entry);
        if parts[2] == MAP_ENTRY_KEY {
            if let Node::Uninitialized = node_map_entry.key {
                node_map_entry.key = self.parse_with_split_key(&parts[3..], value)?;
                trace!("node_map_entry: {:?}", node_map_entry);
            } else {
                return self.merge_map_entry_item(&mut node_map_entry.key, &parts[3..], value);
            }
        } else if let Node::Uninitialized = node_map_entry.value {
            node_map_entry.value = self.parse_with_split_key(&parts[3..], value)?;
            trace!("{:?}", node_map_entry);
        } else {
            return self.merge_map_entry_item(&mut node_map_entry.value, &parts[3..], value);
        }
        Ok(())
    }

    fn parse_with_split_key(&mut self, key_parts: &[&str], value: &str) -> Result<Node<'de>, DeError> {
        return if key_parts.len() == 1 && !key_parts[0].is_numeric() {
            let mut map = BTreeMap::default();
            map.insert(QueryKey::from_text(key_parts[0]), Node::flat_from(value));
            Ok(Node::Nested(map))
        } else if is_map_entry_item(key_parts) {
            let index = key_parts[1].parse().unwrap();
            let mut map = BTreeMap::default();
            map.insert(index, self.parse_map_entry_first_time(&key_parts[2..], value)?);
            Ok(Node::MapArg(map))
        } else if is_indexed_seq_item(key_parts) {
            self.parse_indexed_seq_first_time(key_parts, value)
        } else {
            Ok(Node::flat_from(value))
        };
    }

    fn parse_into_map(
        &mut self, map: &mut BTreeMap<QueryKey<'de>, Node<'de>>, key_head: &str, key_tail: &[&str], value: &str,
    ) -> Result<(), DeError> {
        let query_key = QueryKey::from_text(key_head);
        if map.contains_key(&query_key) {
            let field_node = map.get_mut(&query_key).unwrap();
            match field_node {
                Node::Nested(inner_map) => {
                    let field = key_tail[0];
                    if inner_map.contains_key(&QueryKey::from_text(field)) {
                        return Err(DeError::Custom(
                            "failed to parse URL parameter. Key is already defined".to_owned(),
                        ));
                    }
                    if key_tail.len() > 1 {
                        self.parse_with_complex_key(field_node, key_tail, value)?;
                    } else {
                        inner_map.insert(QueryKey::from_text(field), Node::flat_from(value));
                    }
                }
                Node::MapArg(ref mut inner_map) => {
                    trace!("key: {:?}, value: {:?}, inner_map: {:?}", key_tail, value, inner_map);
                    if !is_map_entry_item(key_tail) {
                        return Err(DeError::Custom("types conflict. expected map".to_owned()));
                    }
                    let index: usize = key_tail[1].parse().unwrap();

                    return if let btree_map::Entry::Vacant(map_entry) = inner_map.entry(index) {
                        map_entry.insert(self.parse_map_entry_first_time(&key_tail[2..], value)?);
                        Ok(())
                    } else {
                        let map_entry = inner_map.get_mut(&index).unwrap();
                        self.update_map_entry(map_entry, key_tail, value)
                    };
                }
                Node::IndexedSeq(ref mut seq_map) => {
                    if !is_indexed_seq_item(key_tail) {
                        return Err(DeError::Custom("types conflict. expected indexed sequence of items".to_owned()));
                    }
                    return self.update_indexed_seq(seq_map, key_tail, value);
                }
                node => return Err(DeError::Internal(format!("failed to parse query string, {:?}", node))),
            }
        } else {
            let node = self.parse_with_split_key(key_tail, value)?;
            map.insert(QueryKey::from_text(key_head), node);
        }
        trace!("parse_into_map: {:?}", map);
        Ok(())
    }

    fn parse_with_complex_key(&mut self, node: &mut Node<'de>, key_parts: &[&str], value: &str) -> Result<(), DeError> {
        if is_map_entry_item(key_parts) {
            return match node {
                Node::Nested(map) => self.parse_into_map(map, key_parts[0], &key_parts[1..], value),
                node => {
                    Err(DeError::Custom(format!("failed to parse query string: expected map, but found {:?}", node)))
                }
            };
        } else if is_indexed_seq_item(key_parts) {
            return match node {
                Node::Nested(map) => self.parse_into_map(map, key_parts[0], &key_parts[1..], value),
                Node::IndexedSeq(inner_map) => self.update_indexed_seq(inner_map, key_parts, value),
                node => Err(DeError::Internal(format!("unsupported {:?}, expected IndexedSeq", node))),
            };
        }
        match node {
            Node::Nested(map) => self.parse_into_map(map, key_parts[0], &key_parts[1..], value),
            _ => Err(DeError::Custom(
                "failed to parse query string: unexpected parameter definition structure".to_owned(),
            )),
        }
    }

    fn parse(&mut self, node: &mut Node<'de>) -> Result<(), DeError> {
        while let Some((key, value)) = self.0.next() {
            if !key.chars().all(|ch| ch.is_alphanumeric() || ch == '_' || ch == '.') {
                return Err(DeError::RootNode(format!(
                    "query string key '{key}' contains illegal character(s). \
                    allowed values: '[a-zA-Z]', '[0-9]', '_', '.'"
                )));
            }
            if key.contains(KEY_DELIMITER) {
                let key_parts: Vec<&str> = key.split(KEY_DELIMITER).collect();
                self.parse_with_complex_key(node, &key_parts, &value)?;
            } else {
                match node {
                    Node::Nested(map) => {
                        map.insert(QueryKey::Text(key), Node::Flat(value));
                    }
                    _ => return Err(DeError::RootNode("unexpected root-level element".to_owned())),
                }
            }
        }
        Ok(())
    }
}

fn is_map_entry_item(key_parts: &[&str]) -> bool {
    if key_parts.len() < 2 {
        return false;
    }
    key_parts[0] == MAP_ENTRY
        && key_parts[1].is_numeric()
        && (key_parts[2] == MAP_ENTRY_KEY || key_parts[2] == MAP_ENTRY_VALUE)
}

fn is_indexed_seq_item(key_parts: &[&str]) -> bool {
    key_parts.len() == 1 && key_parts[0].is_numeric() || (key_parts.len() > 1 && key_parts[1].is_numeric())
}
