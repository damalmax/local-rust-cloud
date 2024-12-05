use std::io::Read;

use serde::de::{DeserializeOwned, Error};
use serde::Deserialize;

use crate::de::aws_query::AwsQueryDeserializer;
use crate::de::error::DeError;

mod aws_query;
pub mod error;
mod node;
mod node_deserializer;
mod node_parser;
mod node_seq;
mod query_str_deserializer;

/// The type represents all possible errors than can occur when serializing or deserializing
/// a query string written in accordance with AWS Query protocol.
/// Deserializes a querystring from a `&[u8]`.
///
/// ```
///use serde_derive::{Deserialize, Serialize};
///
///
/// #[derive(Debug, Deserialize, PartialEq, Serialize)]
/// struct Query {
///     #[serde(rename = "Action")]
///     action: String,
///     #[serde(rename = "Version")]
///     version: String,
///     #[serde(rename = "Limit")]
///     limit: Limit,
/// }
///
/// #[derive(Debug, Deserialize, PartialEq, Serialize)]
/// struct Limit {
///     min: i32
/// }
///
/// # fn main(){
/// let q =  Query {
///     action: "Query".to_owned(),
///     version: "2023-11-26".to_owned(),
///     limit: Limit {
///         min: 22,
///     },
/// };
///
/// assert_eq!(
///     local_aws_query_protocol::from_bytes::<Query>(
///         "Action=Query&Version=2023-11-26&Limit.min=22".as_bytes()
///     ).unwrap(), q);
/// # }
/// ```
pub fn from_bytes<'a, T>(input: &'a [u8]) -> Result<T, DeError>
where
    T: Deserialize<'a>,
{
    T::deserialize(AwsQueryDeserializer::new(form_urlencoded::parse(input))?)
}

/// Deserializes a querystring from a `&[u8]`.
///
/// ```
///use serde_derive::{Deserialize, Serialize};
///
///
/// #[derive(Debug, Deserialize, PartialEq, Serialize)]
/// struct Query {
///     #[serde(rename = "Action")]
///     action: String,
///     #[serde(rename = "Version")]
///     version: String,
///     #[serde(rename = "Limit")]
///     limit: Limit,
/// }
///
/// #[derive(Debug, Deserialize, PartialEq, Serialize)]
/// struct Limit {
///     min: i32
/// }
///
/// # fn main(){
/// let q =  Query {
///     action: "Query".to_owned(),
///     version: "2023-11-26".to_owned(),
///     limit: Limit {
///         min: 22,
///     },
/// };
///
/// assert_eq!(
///     local_aws_query_protocol::from_str::<Query>(
///         "Action=Query&Version=2023-11-26&Limit.min=22"
///     ).unwrap(), q);
/// # }
/// ```
pub fn from_str<'a, T>(input: &'a str) -> Result<T, DeError>
where
    T: Deserialize<'a>,
{
    from_bytes(input.as_bytes())
}

pub fn from_reader<T, R>(mut reader: R) -> Result<T, DeError>
where
    T: DeserializeOwned,
    R: Read,
{
    let mut buf = vec![];
    reader
        .read_to_end(&mut buf)
        .map_err(|e| Error::custom(format_args!("could not read input: {}", e)))?;
    from_bytes(&buf)
}
