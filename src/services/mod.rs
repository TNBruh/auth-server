pub mod structs;
mod tests;
pub mod traits;

use anyhow::{anyhow, bail, ensure};
use bcrypt::{hash, verify, DEFAULT_COST};
use either::Either;
use std::collections::HashMap;

use self::traits::RequestRead;

#[async_trait]
impl traits::RequestRead for structs::Host {
    async fn read(
        &self,
        data: Either<&'static str, &HashMap<String, String>>, //
    ) -> anyhow::Result<Either<String, HashMap<String, String>>> {
        //requests for user data to server
        let result = match self.mode {
            structs::ServerType::Server => {
                //init client
                let client = reqwest::Client::new();

                //parse Either input
                let login_hashmap = data.right().ok_or(anyhow!("not right"))?;

                //extracts identifier(username or email) and password
                let guest_identifier = login_hashmap
                    .get("identifier")
                    .ok_or(anyhow!("no identifier"))?;
                // let guest_password = login_hashmap
                //     .get("password")
                //     .ok_or(anyhow!("no password"))?;

                let route = self
                    .get_user_route
                    .clone()
                    .replace("<user_id>", guest_identifier);

                let url = format!("{}:{}/{}", self.host, self.port, route);

                //requests user login data from server
                let res = client
                    .get(url)
                    .header("Authorization", &self.password)
                    .send()
                    .await?
                    .json::<HashMap<String, String>>()
                    .await?;
                // let hash = res
                //     .get("password")
                //     .ok_or(anyhow!("did not receive password"))?;

                //check bcrypt hash
                // let hash_check = verify(guest_password, hash)?;
                // ensure!(hash_check, "hash_check failed");

                //filter out password
                // res.remove("password");

                //return data
                anyhow::Ok(Either::Right(res))
            }
            _ => {
                bail!("Unsupported mode");
            }
        };

        result
    }
}

impl structs::Host {
    pub async fn login(
        &self,
        data: &HashMap<String, String>, //
    ) -> anyhow::Result<Either<String, HashMap<String, String>>> {
        let result = match self.mode {
            structs::ServerType::Server => {
                let guest_password = data.get("password").ok_or(anyhow!("no password"))?;

                //get data from server
                let mut user_data = self
                    .read(Either::Right(data))
                    .await?
                    .right()
                    .ok_or(anyhow!("not right"))?;

                //read password field
                let hash = user_data
                    .get("password")
                    .ok_or(anyhow!("did not receive password"))?;

                //check bcrypt hash
                let hash_check = verify(guest_password, hash)?;
                ensure!(hash_check, "hash_check failed");

                //filter out password
                user_data.remove("password");

                anyhow::Ok(Either::Right(user_data))
            }
            _ => {
                bail!("Unsupported mode");
            }
        };

        result
    }
}
