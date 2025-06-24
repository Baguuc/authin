use crate::prelude::*;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Permission {
    pub name: String,
}
