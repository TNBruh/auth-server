use anyhow::Result;
use either::Either;

use std::collections::HashMap;

#[rocket::async_trait]
pub trait RequestCreate {
    async fn create(
        &self,
        data: Either<&'static str, &HashMap<String, String>>,
    ) -> Result<Either<String, HashMap<String, String>>>;
}

#[rocket::async_trait]
pub trait RequestRead {
    async fn read(
        &self,
        data: Either<&'static str, &HashMap<String, String>>,
    ) -> Result<Either<String, HashMap<String, String>>>;
}

#[rocket::async_trait]
pub trait RequestUpdate {
    async fn update(
        &self,
        data: Either<&'static str, &HashMap<String, String>>,
    ) -> Result<Either<String, HashMap<String, String>>>;
}

#[rocket::async_trait]
pub trait RequestDelete {
    async fn delete(
        &self,
        data: Either<&'static str, &HashMap<String, String>>,
    ) -> Result<Either<String, HashMap<String, String>>>;
}
