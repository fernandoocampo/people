use crate::errors::error::Error;
use async_trait::async_trait;

#[async_trait]
pub trait Censorious {
    async fn censor(&self, word: String) -> Result<String, Error>;
    async fn censor_with_backoff(&self, word: String) -> Result<String, Error>;
}
