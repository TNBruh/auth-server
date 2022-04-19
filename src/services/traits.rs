use anyhow::Result;
use either::Either;
use rocket::serde::{json::serde_json, Serialize};

use std::collections::HashMap;

#[rocket::async_trait]
pub trait RequestCreate {
    async fn create(
        &self,
        data: Either<&'static str, &impl Serialize>,
    ) -> anyhow::Result<Either<String, HashMap<String, String>>>;
}

#[rocket::async_trait]
pub trait RequestRead {
    async fn read(
        &self,
        data: Either<&'static str, &(impl Serialize + std::marker::Sync)>,
    ) -> anyhow::Result<Either<String, HashMap<String, String>>>;
}

#[rocket::async_trait]
pub trait RequestUpdate {
    async fn update(
        &self,
        data: Either<&'static str, &impl Serialize>,
    ) -> anyhow::Result<Either<String, HashMap<String, String>>>;
}

#[rocket::async_trait]
pub trait RequestDelete {
    async fn delete(
        &self,
        data: Either<&'static str, &impl Serialize>,
    ) -> anyhow::Result<Either<String, HashMap<String, String>>>;
}
