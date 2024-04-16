use std::net::Ipv4Addr;

use anyhow::{bail, Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::types::{ProviderBetID, Url, Username};

#[derive(Debug)]
pub struct Connector {
    config: KingMakerConfig,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KingMakerConfig {
    pub api_url: Url,
    pub lobby_url: Url,
    pub game_provider_code: String,
    #[serde(rename = "clientID")]
    pub client_id: String,
    pub client_secret: String,
    pub ip_list: Vec<Ipv4Addr>,
}

impl Connector {
    pub fn new(config: KingMakerConfig) -> Self {
        Self { config }
    }

    pub async fn get_round_history(
        &self,
        username: &Username,
        round_id: &ProviderBetID,
    ) -> Result<SuccessHistoryResponse> {
        let result: Response<SuccessHistoryResponse> = Client::new()
            .get(format!(
                "{}/history/providers/{}/rounds/{round_id}/users/{username}",
                self.config.api_url, self.config.game_provider_code
            ))
            .send()
            .await
            .with_context(|| {
                format!(
                    "Failed to fetch king maker round history for bet {}",
                    round_id
                )
            })?
            .json()
            .await
            .context("Failed to parse json from KingMaker")?;

        match result {
            Response::Fail { err, err_desc } => {
                bail!(
                    "Kingmaker get_round_history API error: {}. {}",
                    err,
                    err_desc
                )
            }
            Response::Success(data) => Ok(data),
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum Response<T> {
    Fail {
        err: String,
        #[serde(rename = "errdesc")]
        err_desc: String,
    },
    Success(T),
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SuccessHistoryResponse {
    pub urls: Vec<Url>,
}
