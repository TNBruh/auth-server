pub mod structs;
pub mod traits;

use anyhow::{anyhow, bail, ensure};
use bcrypt::{hash, verify, DEFAULT_COST};
use either::Either;
use std::collections::HashMap;

#[async_trait]
impl traits::RequestRead for structs::Host {
    async fn read(
        &self,
        data: Either<&'static str, HashMap<&'static str, &'static str>>, //
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
                let guest_password = login_hashmap
                    .get("password")
                    .ok_or(anyhow!("no password"))?;

                let url = format!("{}:{}/{}", self.host, self.port, guest_identifier);

                //requests user login data from server
                let mut res = client
                    .get(url)
                    .header("Authorization", &self.password)
                    .send()
                    .await?
                    .json::<HashMap<String, String>>()
                    .await?;
                let hash = res
                    .get("password")
                    .ok_or(anyhow!("did not receive password"))?;

                //check bcrypt hash
                let hash_check = verify(guest_password, hash)?;
                ensure!(hash_check, "hash_check failed");

                //filter out password
                res.remove("password");

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
