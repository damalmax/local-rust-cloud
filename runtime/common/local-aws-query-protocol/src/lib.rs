//! `AWS query protocol` meets Serde

#![forbid(unsafe_code)]

pub use de::{error::DeError, from_bytes, from_reader, from_str};

mod de;
mod ext;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use serde_derive::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    struct LimitStruct {
        min: i32,
        max: Option<i32>,
    }

    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    struct QueryStruct {
        #[serde(rename = "Action")]
        action: String,
        #[serde(rename = "Version")]
        version: String,
        #[serde(rename = "Limit")]
        limit: LimitStruct,
    }

    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    struct QueryMapsStruct {
        #[serde(rename = "Action")]
        action: String,
        #[serde(rename = "Version")]
        version: String,
        #[serde(rename = "MapArg")]
        map_arg: HashMap<String, String>,
        #[serde(rename = "reNamed")]
        renamed_map: HashMap<String, String>,
        #[serde(rename = "ComplexMapArg")]
        complex_map: Option<HashMap<String, GreetingStruct>>,
    }

    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    struct GreetingStruct {
        hi: String,
    }

    #[test]
    fn test_from_str_with_inner_struct() {
        let q = QueryStruct {
            action: "Query".to_owned(),
            version: "2023-11-26".to_owned(),
            limit: LimitStruct { min: 22, max: None },
        };
        assert_eq!(super::from_str::<QueryStruct>("Action=Query&Version=2023-11-26&Limit.min=22").unwrap(), q);
    }

    #[test]
    fn test_from_str_with_inner_struct_with_multiple_fields() {
        let q = QueryStruct {
            action: "Query".to_owned(),
            version: "2023-11-26".to_owned(),
            limit: LimitStruct { min: 22, max: Some(11) },
        };
        let result =
            super::from_str::<QueryStruct>("Action=Query&Version=2023-11-26&Limit.min=22&Limit.max=11").unwrap();
        assert_eq!(result, q);
    }

    #[test]
    fn test_from_str_query_maps() {
        let mut map_arg = HashMap::default();
        map_arg.insert("bar".to_owned(), "Bar".to_owned());
        map_arg.insert("foo".to_owned(), "Foo".to_owned());
        let mut renamed_map = HashMap::default();
        renamed_map.insert("foo".to_owned(), "Foo".to_owned());
        let mut complex_map = HashMap::default();
        complex_map.insert("bar".to_owned(), GreetingStruct { hi: "Bar".to_owned() });
        complex_map.insert("foo".to_owned(), GreetingStruct { hi: "Foo".to_owned() });
        let q = QueryMapsStruct {
            action: "QueryMaps".to_owned(),
            version: "2020-07-02".to_owned(),
            map_arg,
            renamed_map,
            complex_map: Some(complex_map),
        };

        let result = super::from_str::<QueryMapsStruct>(
            "Action=QueryMaps&Version=2020-07-02\
                    &MapArg.entry.1.key=bar&MapArg.entry.1.value=Bar&MapArg.entry.2.key=foo\
                    &MapArg.entry.2.value=Foo&reNamed.entry.1.key=foo&reNamed.entry.1.value=Foo\
                    &ComplexMapArg.entry.1.key=bar&ComplexMapArg.entry.1.value.hi=Bar\
                    &ComplexMapArg.entry.2.key=foo&ComplexMapArg.entry.2.value.hi=Foo",
        )
        .unwrap();

        assert_eq!(result, q);
    }

    #[test]
    fn test_from_str_query_maps_without_value_for_complex_map() {
        let mut map_arg = HashMap::default();
        map_arg.insert("bar".to_owned(), "Bar".to_owned());
        map_arg.insert("foo".to_owned(), "Foo".to_owned());
        let mut renamed_map = HashMap::default();
        renamed_map.insert("foo".to_owned(), "Foo".to_owned());
        let q = QueryMapsStruct {
            action: "QueryMaps".to_owned(),
            version: "2020-07-02".to_owned(),
            map_arg,
            renamed_map,
            complex_map: None,
        };

        let result = super::from_str::<QueryMapsStruct>(
            "Action=QueryMaps&Version=2020-07-02\
                    &MapArg.entry.1.key=bar&MapArg.entry.1.value=Bar&MapArg.entry.2.key=foo\
                    &MapArg.entry.2.value=Foo&reNamed.entry.1.value=Foo&reNamed.entry.1.key=foo",
        )
        .unwrap();

        assert_eq!(result, q);
    }

    #[derive(Debug, Deserialize, PartialEq, Eq, Ord, PartialOrd, Hash)]
    struct KeyStruct {
        field1: String,
        field2: String,
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct StructWithMapWithKeyStructInside {
        #[serde(rename = "InnerMap")]
        inner_map: HashMap<KeyStruct, String>,
    }

    #[test]
    fn test_from_map_with_complex_key() {
        let mut inner_map = HashMap::default();
        inner_map.insert(
            KeyStruct {
                field1: "field1-value".to_owned(),
                field2: "field2-value".to_owned(),
            },
            "value".to_owned(),
        );

        let q = StructWithMapWithKeyStructInside { inner_map };
        assert_eq!(
            super::from_str::<StructWithMapWithKeyStructInside>(
                "InnerMap.entry.1.key.field1=field1-value\
                    &InnerMap.entry.1.key.field2=field2-value\
                    &InnerMap.entry.1.value=value"
            )
            .unwrap(),
            q
        );
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct QueryListStruct {
        #[serde(rename = "Action")]
        action: String,
        #[serde(rename = "Version")]
        version: String,
        #[serde(rename = "ListArg")]
        list_arg: Vec<String>,
        #[serde(rename = "ComplexListArg")]
        complex_list_arg: Vec<GreetingStruct>,
        #[serde(rename = "FlattenedListArg")]
        flattened_list_arg: Option<Vec<String>>,
    }

    #[test]
    fn test_from_query_list() {
        let q = QueryListStruct {
            action: "QueryLists".to_owned(),
            version: "2020-07-02".to_owned(),
            list_arg: vec!["foo".to_owned(), "bar".to_owned(), "baz".to_owned()],
            complex_list_arg: vec![
                GreetingStruct { hi: "hello".to_owned() },
                GreetingStruct { hi: "hola".to_owned() },
            ],
            flattened_list_arg: Some(vec!["A".to_owned(), "B".to_owned()]),
        };
        assert_eq!(
            super::from_str::<QueryListStruct>(
                "Action=QueryLists\
                        &Version=2020-07-02\
                        &ListArg.member.2=bar\
                        &ListArg.member.1=foo\
                        &ListArg.member.3=baz\
                        &ComplexListArg.member.2.hi=hola\
                        &ComplexListArg.member.1.hi=hello\
                        &FlattenedListArg.2=B\
                        &FlattenedListArg.1=A"
            )
            .unwrap(),
            q
        );
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct FlattenedListWrapperStruct {
        #[serde(rename = "FlattenedListArg")]
        flattened_list_arg: Option<Vec<String>>,
    }

    #[test]
    fn test_from_flattened_query_list() {
        let q = FlattenedListWrapperStruct {
            flattened_list_arg: Some(vec!["A".to_owned(), "B".to_owned()]),
        };
        assert_eq!(
            super::from_str::<FlattenedListWrapperStruct>("&FlattenedListArg.1=A&FlattenedListArg.2=B").unwrap(),
            q
        );
    }

    #[test]
    fn test_from_query_list_no_flattened() {
        let q = QueryListStruct {
            action: "QueryLists".to_owned(),
            version: "2020-07-02".to_owned(),
            list_arg: vec!["foo".to_owned(), "bar".to_owned(), "baz".to_owned()],
            complex_list_arg: vec![
                GreetingStruct { hi: "hello".to_owned() },
                GreetingStruct { hi: "hola".to_owned() },
            ],
            flattened_list_arg: None,
        };
        assert_eq!(
            super::from_str::<QueryListStruct>(
                "Action=QueryLists\
                        &Version=2020-07-02\
                        &ListArg.member.1=foo\
                        &ListArg.member.2=bar\
                        &ListArg.member.3=baz\
                        &ComplexListArg.member.1.hi=hello\
                        &ComplexListArg.member.2.hi=hola"
            )
            .unwrap(),
            q
        );
    }

    #[derive(Debug, Deserialize, PartialEq)]
    enum EnumStruct {
        #[serde(rename = "FIRST")]
        First,
        #[serde(rename = "SECOND")]
        Second,
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct EnumWrapperStruct {
        #[serde(rename = "ListArg")]
        list_arg: Option<Vec<EnumStruct>>,
    }

    #[test]
    fn test_enum_from_str() {
        let q = EnumWrapperStruct {
            list_arg: Some(vec![
                EnumStruct::First,
                EnumStruct::Second,
                EnumStruct::First,
                EnumStruct::Second,
                EnumStruct::First,
            ]),
        };
        assert_eq!(
            super::from_str::<EnumWrapperStruct>(
                "ListArg.member.1=FIRST\
                        &ListArg.member.2=SECOND\
                        &ListArg.member.3=FIRST\
                        &ListArg.member.4=SECOND\
                        &ListArg.member.5=FIRST"
            )
            .unwrap(),
            q
        );
    }

    #[test]
    fn deserialize_enum_top_level() {
        #[derive(Deserialize, Debug, PartialEq)]
        enum EnumStruct {
            Value1 { field1: String },
            Value2 { field2: String },
        }

        let params = "Value1.field1=test";
        let rec_params: EnumStruct = super::from_str::<EnumStruct>(params).unwrap();
        assert_eq!(
            rec_params,
            EnumStruct::Value1 {
                field1: "test".to_owned()
            }
        );
    }

    #[test]
    fn test_enum_from_str_missing_member_error() {
        assert!(super::from_str::<EnumWrapperStruct>("ListArg.member.1=FIRST&ListArg.member.3=SECOND").is_err());
    }

    #[test]
    fn test_from_str_for_plain_struct() {
        let l = LimitStruct { min: 10, max: None };
        assert_eq!(super::from_str::<LimitStruct>("min=10").unwrap(), l);
    }

    #[test]
    fn test_decode_wrapped_string() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct WrappedString(String);

        #[derive(Debug, Deserialize, PartialEq)]
        struct WrappedNumber(i64);

        #[derive(Debug, Deserialize, PartialEq)]
        struct Entity {
            action: Option<WrappedString>,
            min: Option<WrappedNumber>,
        }

        let expected_result = Entity {
            action: Some(WrappedString(String::from("sometext"))),
            min: Some(WrappedNumber(10)),
        };

        let result = super::from_str::<Entity>("action=sometext&min=10").unwrap();
        assert_eq!(result, expected_result);
    }
}
