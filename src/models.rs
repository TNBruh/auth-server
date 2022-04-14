use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Token {
    refresh_token: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Login {
    identifier: String,
    password: String,
}
