use crate::errors::error::Error;
use async_trait::async_trait;

#[async_trait]
pub trait Censorious {
    async fn censor(&self, word: String) -> Result<String, Error>;
}
