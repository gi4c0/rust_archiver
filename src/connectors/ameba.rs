use std::net::Ipv4Addr;

use anyhow::{bail, Context};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::types::{ProviderBetID, Url, Username};

#[derive(Debug)]
pub struct Connector {
    config: AmebaConfig,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AmebaConfig {
    pub secret_key: String,
    pub api_url: Url,
    #[serde(rename = "siteID")]
    pub site_id: i64,
    pub ip_list: Vec<Ipv4Addr>,
}

impl Connector {
    pub fn new(config: AmebaConfig) -> Self {
        Self { config }
    }

    pub async fn get_round_history(
        &self,
        username: &Username,
        bet_id: &ProviderBetID,
    ) -> anyhow::Result<Url> {
        let payload = GetRoundHistoryPayload {
            round_id: bet_id.clone(),
            action: "get_game_history_url",
            site_id: self.config.site_id,
            account_name: username.clone(),
        };

        let result: GetRoundHistoryResponse = Client::new()
            .post(format!("{}/dms/api", &self.config.api_url))
            .form(&payload)
            .send()
            .await
            .with_context(|| format!("Failed to fetch bet detail for '{}'", bet_id))?
            .json()
            .await
            .context("Failed to parse response from ameba bet detail")?;

        match result.error_code.as_str() {
            "OK" => Ok(result.game_history_url),
            _ => bail!("Ameba get bet detail returned error: {:?}", result),
        }
    }
}

#[derive(Serialize)]
struct GetRoundHistoryPayload {
    action: &'static str,
    site_id: i64,
    account_name: Username,
    round_id: ProviderBetID,
}

#[derive(Debug, Deserialize)]
struct GetRoundHistoryResponse {
    error_code: String,
    game_history_url: Url,
}
