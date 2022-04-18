use anyhow::{bail, ensure, Result};
use rocket::serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, FromForm)]
#[serde(crate = "rocket::serde")]
pub struct Token {
    pub refresh_token: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, FromForm)]
#[serde(crate = "rocket::serde")]
pub struct Login {
    pub identifier: Option<String>,
    pub password: Option<String>,
}

impl Login {
    pub fn extract(&self) -> Result<HashMap<String, String>> {
        ensure!(
            self.identifier.is_some() && self.password.is_some(),
            "missing field",
        );

        let mut result = HashMap::new();
        let identifier = self.identifier.as_ref().unwrap();
        let password = self.password.as_ref().unwrap();
        result.insert(String::from("identifier"), String::from(identifier));
        result.insert(String::from("password"), String::from(password));

        anyhow::Ok(result)
    }
}
