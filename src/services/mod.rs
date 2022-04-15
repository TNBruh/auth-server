pub mod structs;
pub mod traits;

use anyhow::{anyhow, bail};
use bcrypt::{hash, verify, DEFAULT_COST};
use std::collections::HashMap;

#[async_trait]
impl traits::RequestCreate for structs::Host {
    async fn create(
        &self,
        data: rocket::Either<std::string::String, HashMap<String, String>>,
    ) -> anyhow::Result<either::Either<String, HashMap<String, String>>> {
        //requests for user data to server
        match self.mode {
            structs::ServerType::Server => {
                let client = reqwest::Client::new();
                let login_hashmap = data.right().ok_or(anyhow!("not right"))?;
                let guest_identifier = login_hashmap
                    .get(String::from("identifier"))
                    .ok_or(anyhow!("no identifier"))?;
                let guest_password = login_hashmap
                    .get("password")
                    .ok_or(anyhow!("no password"))?;
                let route = self
                    .get_user_route
                    .clone()
                    .replace("<user_id>", &guest_identifier);
                let url = format!("{}:{}/{}", self.host, self.port, self.get_user_route);
                let resp_data = client
                    .get(url)
                    .header("Authorization", self.password.clone())
                    .send()
                    .await?
                    .json::<HashMap<String, String>>()
                    .await?;

                //check bcrypt hash
                //let hash_check = verify(login_hashmap.get("password"), resp_data.get("password"));
            }
            _ => {
                bail!("Unsupported mode");
            }
        }

        todo!()
    }
}
