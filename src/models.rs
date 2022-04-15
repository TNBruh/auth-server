use anyhow::{bail, Result};
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, FromForm)]
#[serde(crate = "rocket::serde")]
pub struct Token {
    refresh_token: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, FromForm)]
#[serde(crate = "rocket::serde")]
pub struct Login {
    identifier: Option<String>,
    password: Option<String>,
}
