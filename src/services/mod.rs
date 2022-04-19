pub mod structs;
mod tests;
pub mod traits;

use anyhow::{anyhow, bail, ensure};
use bcrypt::{hash, verify, DEFAULT_COST};
use either::Either;
use log::{debug, info, trace, warn};
use rocket::serde::{json::serde_json, Serialize};
use std::collections::HashMap;

use crate::models::Login;

use self::traits::RequestRead;

#[async_trait]
impl traits::RequestRead for structs::Host {
    async fn read(
        &self,
        data: Either<&'static str, &(impl Serialize + std::marker::Sync)>,
    ) -> anyhow::Result<Either<String, HashMap<String, String>>> {
        //requests for user data to server
        let result = match self.mode {
            structs::ServerType::Server => {
                //init client
                let client = reqwest::Client::new();

                //parse Either input
                let serializeable_login = data.right().ok_or(anyhow!("not right"))?;
                trace!("serialized data input");

                //extracts identifier(username or email) and password
                let login_data = serde_json::to_value(serializeable_login)?;
                debug!("serialized login data into Value: {}", login_data);
                // let guest_password = login_hashmap
                //     .get("password")
                //     .ok_or(anyhow!("no password"))?;

                let guest_identifier = login_data
                    .get("identifier")
                    .ok_or(anyhow!("cannot deserialize"))?;
                debug!("guest identifier: {}", guest_identifier);

                let route = self
                    .get_user_route
                    .clone()
                    .replace("<user_id>", &guest_identifier.to_string());
                let url = format!("{}:{}/{}", self.host, self.port, route);
                debug!("route: {}", url);

                //requests user login data from server
                let res = client
                    .get(url)
                    .header("Authorization", &self.password)
                    .send()
                    .await?
                    .json::<HashMap<String, String>>()
                    .await?;
                trace!("successfully received response");
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
        data: &Login, //
    ) -> anyhow::Result<Either<String, HashMap<String, String>>> {
        let result = match self.mode {
            structs::ServerType::Server => {
                trace!("extracting");
                let guest_data = data.extract()?;
                trace!("extracted guest data");
                let guest_password = guest_data
                    .get("password")
                    .ok_or(anyhow!("missing password"))?;
                debug!("guest password: {}", guest_password);
                //get data from server
                let mut user_data = self
                    .read(Either::Right(data))
                    .await?
                    .right()
                    .ok_or(anyhow!("not right"))?;
                trace!("finished reading data from host");

                //read password field
                let hash = user_data
                    .get("password")
                    .ok_or(anyhow!("did not receive password"))?;
                debug!("extracted hash: {}", hash);

                //check bcrypt hash
                let hash_check = verify(guest_password, hash)?;
                ensure!(hash_check, "hash_check failed");
                trace!("successful hash check");

                //filter out password
                user_data.remove("password");
                trace!("removed password field");

                anyhow::Ok(Either::Right(user_data))
            }
            _ => {
                bail!("Unsupported mode");
            }
        };

        result
    }
}
