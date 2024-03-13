use std::net::Ipv4Addr;

use anyhow::Context;
use serde::{Deserialize, Serialize};

use crate::types::{ProviderBetID, Url, Username};

impl AeConnector {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub async fn get_transaction_history_result(
        &self,
        username: &Username,
        provider_bet_id: ProviderBetID,
    ) -> anyhow::Result<Url> {
        let payload = GetHistoryResultPayload {
            platform: "SEXYBCRT".to_string(),
            cert: self.config.cert.clone(),
            user_id: username.clone(),
            agent_id: self.config.agent_id.clone(),
            platform_tx_id: provider_bet_id.clone(),
        };

        let result: AeTransactionHistoryResponse = reqwest::Client::new()
            .post(&format!(
                "{}/getTransactionHistoryResult",
                &self.config.host
            ))
            .form(&payload)
            .send()
            .await
            .with_context(|| {
                format!(
                    "Failed to fetch sexy bet details for bet '{}'",
                    &provider_bet_id
                )
            })?
            .json()
            .await?;

        if result.status == AeResponseStatus::Success {
            if let Some(url) = result.url {
                return Ok(url);
            }
        }

        anyhow::bail!(
            "Error returned from ae bet details API: {}",
            result.desc.unwrap_or("empty".to_string())
        );
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub host: Url,
    pub cert: String,
    #[serde(rename = "agentID")]
    pub agent_id: String,
    pub secret_key: String,
    pub ip_list: Vec<Ipv4Addr>,
}

pub struct AeConnector {
    config: Config,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct GetHistoryResultPayload {
    cert: String,
    agent_id: String,
    user_id: Username,
    platform_tx_id: ProviderBetID,
    platform: String,
}

#[derive(PartialEq, Eq, Deserialize)]
enum AeResponseStatus {
    #[serde(rename = "0000")]
    Success,

    #[serde(rename = "1018")]
    LowBalance,

    #[serde(rename = "1013")]
    AccountLocked,

    #[serde(rename = "1014")]
    AccountSuspended,

    #[serde(rename = "1012")]
    AccountNotFound,

    #[serde(rename = "9999")]
    Fail,

    #[serde(untagged)] // Basically says "Put anything else here if it's a string"
    Other(String),
}

#[derive(Deserialize)]
struct AeTransactionHistoryResponse {
    status: AeResponseStatus,
    desc: Option<String>,
    url: Option<Url>,
}
