#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServerType {
    Redis,
    Memcached,
    Server,
    LocalMemory,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Storage {
    pub host: String,
    pub port: usize,
    pub password: String,
    pub mode: ServerType, //storage type
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Host {
    pub host: String,
    pub port: usize,
    pub password: String,
    pub mode: ServerType,
    pub get_user_route: String,
    pub register_user_route: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JWT {
    pub key: String,
    pub specify: bool,
    pub access_age: usize,
    pub refresh_age: usize,
}
