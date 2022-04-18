mod models;
mod routes;
mod services;

use anyhow::Result;
use config::Config;
use std::collections::HashMap;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref CONFIG: HashMap<String, String> = {
        let settings = Config::builder()
            .add_source(config::File::with_name("Settings"))
            .build()
            .unwrap();

        settings
            .try_deserialize::<HashMap<String, String>>()
            .unwrap()
    };
}

#[rocket::main]
async fn main() -> Result<()> {
    let host = inits::init_host()?;

    let server = rocket::build()
        .mount("/", routes::get_routes())
        .manage(host)
        .ignite()
        .await?;
    let result = server.launch().await?;

    Ok(result)
}

pub mod inits {
    use crate::services::structs::ServerType;
    use crate::services::structs::*;
    use crate::CONFIG;
    use anyhow::{anyhow, Ok, Result};

    pub fn init_host() -> Result<Host> {
        let host_type_str = CONFIG
            .get("HOST_TYPE")
            .ok_or(anyhow!("no HOST_TYPE provided"))?;

        let host_type = match host_type_str.as_str() {
            "SERVER" => ServerType::Server,
            "MEMCACHED" => ServerType::Memcached,
            "REDIS" => ServerType::Redis,
            _ => ServerType::LocalMemory,
        };

        let host_ip = CONFIG
            .get("HOST_IP")
            .ok_or(anyhow!("no HOST_IP provided"))?;

        let host_port = CONFIG
            .get("HOST_PORT")
            .ok_or(anyhow!("no HOST_PORT provided"))?
            .parse::<usize>()?;

        let get_user_route = CONFIG
            .get("GET_USER_ROUTE")
            .ok_or(anyhow!("no GET_USER_ROUTE provided"))?;

        let register_user_route = CONFIG
            .get("REGISTER_USER_ROUTE")
            .ok_or(anyhow!("no REGISTER_USER_ROUTE provided"))?;

        let host_password = CONFIG
            .get("HOST_PASSWORD")
            .ok_or(anyhow!("no HOST_PASSWORD provided"))?;

        Ok(Host {
            host: host_ip.clone(),
            port: host_port,
            password: host_password.clone(),
            mode: host_type,
            get_user_route: get_user_route.clone(),
            register_user_route: register_user_route.clone(),
        })
    }

    pub fn init_storage() -> Result<Storage> {
        todo!()
    }

    pub fn init_jwt() -> Result<JWT> {
        todo!()
    }
}
