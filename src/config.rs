use crate::prelude::*;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Config {
    pub port: u16,
    pub database: DatabaseConfig,
    pub jwt: JWTConfig,
    pub permissions: Vec<String>,
    pub groups: Vec<crate::models::group::Group>,
    pub users: Vec<UserConfig>
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct DatabaseConfig {
    pub user: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub database: String
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct JWTConfig {
    pub encryption_key: String
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct UserConfig {
    pub login: String,
    pub initial_pwd: String,
    pub groups: Vec<String>
}

impl Config {
    pub fn read(path: String) -> Result<Self> {
        use std::fs::read_to_string;
        use serde_json::from_str;

        let content = read_to_string(path)?;
        let parsed = from_str::<Self>(&content)?;

        return Ok(parsed);
    }
}
