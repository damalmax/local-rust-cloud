use convert_case::{Case, Casing};

pub fn operation_name(name: impl Into<String>) -> String {
    let name: String = name.into();
    if name.contains('#') {
        let name = name.split_at(name.find('#').unwrap() + 1).1;
        name.to_case(Case::Snake)
    } else {
        name.to_case(Case::Snake)
    }
}

pub fn struct_name(name: impl Into<String>) -> String {
    let name: String = name.into();
    if name.contains('#') {
        let name = name.split_at(name.find('#').unwrap() + 1).1;
        name.to_case(Case::UpperCamel)
    } else {
        name.to_case(Case::UpperCamel)
    }
}

