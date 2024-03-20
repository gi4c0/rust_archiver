use std::net::Ipv4Addr;

use anyhow::{anyhow, bail, Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::{
    archiver::bets::loader::Bet,
    helpers::crypto,
    types::{Currency, ProviderBetID, Url, Username},
};

#[derive(Debug)]
pub struct Connector {
    config: DotConnectionsConfig,
}

impl Connector {
    pub fn new(config: DotConnectionsConfig) -> Self {
        Self { config }
    }

    pub async fn get_bet_history(&self, bet: &Bet) -> Result<Url> {
        let transaction: Value =
            serde_json::from_str(bet.transactions.get(0).ok_or_else(|| {
                anyhow!(
                    "Empty transactions list in '{}' dot_connections bet",
                    &bet.id
                )
            })?)
            .with_context(|| {
                format!(
                    "Failed to parse dot connections bet transaction: '{}'",
                    &bet.id
                )
            })?;

        let provider = transaction["provider"].as_str().ok_or_else(|| {
            anyhow!(
                "No 'provider' field in dot connections bet transactions: {}",
                &bet.id
            )
        })?;

        let payload = GetHistoryPayload {
            brand_id: self.config.brand_id.clone(),
            sign: self.generate_sign(bet.provider_bet_id.as_ref()),
            currency: bet.currency.clone(),
            round_id: bet.provider_bet_id.clone(),
            provider: provider.to_string(),
            brand_uid: bet.username.clone(),
        };

        let response: HistoryResponse<HistoryData> = Client::new()
            .post(format!("{}/dcs/getReplay", &self.config.api_url))
            .json(&payload)
            .send()
            .await
            .with_context(|| {
                format!(
                    "Failed to make request to dot_connections API for bet history: {}",
                    &bet.id
                )
            })?
            .json()
            .await
            .with_context(|| {
                format!(
                    "Failed to parse response from dot_connections API for bet history: {}",
                    &bet.id
                )
            })?;

        if response.code != DotConnectionsErrorCode::Ok {
            bail!(
                "dot_connections bet history API returned Error. Response code: {}, message: {} ",
                response.code as u64,
                response.msg
            );
        }

        Ok(response.data.record)
    }

    fn generate_sign(&self, part: &str) -> String {
        crypto::md5(format!(
            "{}{part}{}",
            self.config.brand_id, self.config.api_key
        ))
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct GetHistoryPayload {
    brand_id: String,
    sign: String,
    brand_uid: Username,
    currency: Currency,
    round_id: ProviderBetID,
    provider: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct HistoryResponse<T> {
    code: DotConnectionsErrorCode,
    msg: String,
    data: T,
}

#[derive(Debug, Serialize, Deserialize)]
struct HistoryData {
    record: Url,
}

#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, Eq)]
#[repr(u64)]
enum DotConnectionsErrorCode {
    Ok = 1000,
    SystemError = 1001,
    DuplicatedTransaction = 5043,
    BetNotFound = 5042,
    ValidationError = 5001,
    InsufficientBalance = 5003,
    PlayerNotFound = 5009,
    GameNotFound = 5012,
    InvalidProvider = 5015,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DotConnectionsConfig {
    pub api_url: Url,
    pub bet_data_url: Url,
    #[serde(rename = "brandID")]
    pub brand_id: String,
    pub api_key: String,
    pub ip_list: Vec<Ipv4Addr>,
}
