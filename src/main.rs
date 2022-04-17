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
    let settings = Config::builder()
        .add_source(config::File::with_name("Settings"))
        .build()?;

    // Print out our settings (as a HashMap)
    println!(
        "{:?}",
        settings
            .try_deserialize::<HashMap<String, String>>()
            .unwrap()
    );

    let server = rocket::build()
        .mount("/", routes::get_routes())
        .ignite()
        .await?;
    let result = server.launch().await;

    Ok(())
}
