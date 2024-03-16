use std::net::Ipv4Addr;

use anyhow::{bail, Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::types::{ProviderBetID, Url};

pub struct Connector {
    config: ArcadiaConfig,
}

pub struct ArcadiaConfig {
    pub api_url: Url,
    pub authentication: String,
    pub ip_list: Vec<Ipv4Addr>,
}

impl Connector {
    pub fn new(config: ArcadiaConfig) -> Self {
        Self { config }
    }

    pub async fn get_bet_history(&self, bet_id: &ProviderBetID) -> Result<Url> {
        let payload = BetHistoryPayload {
            authentication: self.config.authentication.clone(),
            al_trans_id: bet_id.clone(),
        };

        let result: Response = Client::new()
            .post(format!("{}/GetGameResult", &self.config.api_url))
            .json(&payload)
            .send()
            .await
            .with_context(|| {
                format!(
                    "Failed to send request for bet history to Arcadia API for bet: '{}'",
                    bet_id
                )
            })?
            .json()
            .await
            .with_context(|| {
                format!(
                    "Failed to parse response from arcadia bet history for bet: {}",
                    bet_id
                )
            })?;

        if result.error_code != 0 {
            return Ok(result.data.url);
        }

        bail!(
            "Got error from Arcadia API for bet history '{}'",
            serde_json::to_string(&result).unwrap_or_else(|_| format!("Bet ID: {}", bet_id))
        );
    }
}

#[derive(Serialize)]
struct BetHistoryPayload {
    #[serde(rename = "ALTransID")]
    al_trans_id: ProviderBetID,
    #[serde(rename = "Authentication")]
    authentication: String,
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Response {
    error_code: i64,
    error_message: String,
    time_stamp: String,
    data: DataUrl,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "PascalCase")]
struct DataUrl {
    url: Url,
}
