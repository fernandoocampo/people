use crate::errors::error::Error;
use crate::people::censor::Censorious;
use async_trait::async_trait;
use log::error;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct APIResponse {
    message: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BadWordsResponse {
    pub content: String,
    #[serde(rename = "bad_words_total")]
    pub bad_words_total: i64,
    #[serde(rename = "bad_words_list")]
    pub bad_words_list: Vec<BadWord>,
    #[serde(rename = "censored_content")]
    pub censored_content: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BadWord {
    pub original: String,
    pub word: String,
    pub deviations: i64,
    pub info: i64,
    pub start: i64,
    pub end: i64,
    pub replaced_len: i64,
}

#[derive(Debug, Clone)]
pub struct Censor {
    api_client: Client,
    api_key: String,
    api_url: String,
}

impl Censor {
    pub async fn new(client: Client, api_key: &str, api_url: &str) -> Self {
        Censor {
            api_client: client,
            api_key: api_key.to_string(),
            api_url: api_url.to_string(),
        }
    }

    pub async fn transform_error(&self, res: reqwest::Response) -> String {
        res.json::<APIResponse>().await.unwrap().message
    }
}

#[async_trait]
impl Censorious for Censor {
    async fn censor(&self, word: String) -> Result<String, Error> {
        let api_res = self
            .api_client
            .post(self.api_url.as_str())
            .header("apikey", self.api_key.as_str())
            .body(word.clone())
            .send()
            .await
            .map_err(|e| {
                error!("calling apilayer api: {}", e);
                Error::ValidateBadWordsError
            })?;

        if !api_res.status().is_success() && api_res.status().is_client_error() {
            let err = self.transform_error(api_res).await;
            error!("apilayer api response due to client error: {}", err);
            return Err(Error::ValidateBadWordsError);
        }

        if !api_res.status().is_success() && api_res.status().is_server_error() {
            let err = self.transform_error(api_res).await;
            error!("apilayer api response due to server error: {}", err);
            return Err(Error::ValidateBadWordsError);
        }

        let res = api_res.json::<BadWordsResponse>().await.map_err(|e| {
            error!("parsing apilayer api response: {}", e);
            Error::ValidateBadWordsError
        })?;

        Ok(res.censored_content)
    }
}
