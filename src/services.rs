use std::collections::HashMap;

use anyhow::{bail, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
enum ServerType {
    Redis,
    Memcached,
    Server,
    LocalMemory,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Storage {
    host: String,
    port: usize,
    password: String,
    mode: ServerType, //storage type
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Host {
    host: String,
    port: usize,
    password: String,
    mode: ServerType,
    get_user_route: String,
    register_user_route: String,
}

impl Host {
    pub async fn get_user(&self) -> HashMap<String, String> {
        todo!()
    }

    pub async fn register_user(&self) -> HashMap<String, String> {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JWT {
    key: String,
    specify: bool,
    access_age: usize,
    refresh_age: usize,
}
