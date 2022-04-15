use anyhow::Result;
use either::Either;

use async_trait::async_trait;
use std::collections::HashMap;

#[async_trait]
pub trait RequestCreate {
    async fn create(
        &self,
        data: Either<String, Vec<String>>,
    ) -> Result<Either<String, HashMap<String, String>>>;
}

#[async_trait]
pub trait RequestRead {
    async fn read(
        &self,
        data: Either<String, Vec<String>>,
    ) -> Result<Either<String, HashMap<String, String>>>;
}

#[async_trait]
pub trait RequestUpdate {
    async fn update(
        &self,
        data: Either<String, Vec<String>>,
    ) -> Result<Either<String, HashMap<String, String>>>;
}

#[async_trait]
pub trait RequestDelete {
    async fn delete(
        &self,
        data: Either<String, Vec<String>>,
    ) -> Result<Either<String, HashMap<String, String>>>;
}
