use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListShape {
    pub member: ListMember,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListMember {
    pub target: String,
}
