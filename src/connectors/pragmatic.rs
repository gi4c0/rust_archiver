use std::net::Ipv4Addr;

use anyhow::Context;
use serde::{Deserialize, Serialize};
use serde_repr::Deserialize_repr;

use crate::{
    archiver::bets::loader::Bet,
    enums::Language,
    helpers::crypto,
    types::{ProviderBetID, ProviderGameVendorID, Url, UserID},
};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PragmaticConfig {
    pub casino_name: String,
    pub secret_key: String,
    #[serde(rename = "providerID")]
    pub provider_id: String,
    pub api_url: Url,
    pub username: String,
    pub secure_login: String,
    pub ip_list: Vec<Ipv4Addr>,
    pub game_server_domain: Url,
}

#[derive(Deserialize, Debug)]
pub struct Connector {
    config: PragmaticConfig,
}

impl Connector {
    pub fn new(config: PragmaticConfig) -> Self {
        Self { config }
    }

    pub async fn get_bet_round_history(&self, bet: &Bet) -> anyhow::Result<Url> {
        let mut payload = BetRoundHistoryPayload {
            game_id: bet.provider_game_vendor_id.clone(),
            language: Language::English,
            player_id: bet.user_id.clone(),
            round_id: bet.provider_bet_id.clone(),
            secure_login: self.config.secure_login.clone(),
            hash: None,
        };

        payload.hash = Some(crypto::md5(format!(
            "{}{}",
            serde_urlencoded::to_string(&payload).unwrap(),
            self.config.secret_key
        )));

        let response: BetRoundHistoryResponse = reqwest::Client::new()
            .post(format!("{}/OpenHistoryExtended/", self.config.api_url))
            .form(&payload)
            .send()
            .await
            .with_context(|| format!("Failed to fetch a bet info for bet: '{}", &bet.id))?
            .json()
            .await
            .context("Failed to parse response from pragmatic bet history")?;

        match response.error {
            ErrorCode::Success => Ok(response.url),
            _ => anyhow::bail!(
                "Pragmatic bet history response returned error: {}",
                response.description
            ),
        }
    }
}

#[derive(Serialize)]
#[serde(rename = "camelCase")]
struct BetRoundHistoryPayload {
    game_id: ProviderGameVendorID,
    language: Language,
    player_id: UserID,
    round_id: ProviderBetID,
    secure_login: String,
    hash: Option<String>,
}

#[derive(Deserialize)]
struct BetRoundHistoryResponse {
    description: String,
    error: ErrorCode,
    url: Url,
}

#[repr(u32)]
#[derive(Deserialize_repr)]
enum ErrorCode {
    Success = 0,
    InsufficientBalance = 1,
    PlayerNotFound = 2,
    BetIsNotAllowed = 3,
    TokenExpired = 4,
    InvalidHash = 5,
    PlayerFrozen = 6,
    BadRequestParams = 7,
    GameNotFound = 8,
    BetLimitReached = 9,
    InternalServerError = 100,
    // Operator logic does not require a retry of the request.
    // InternalServerError = 120, //
    EndRoundError = 130,
    RealityCheckWarning = 210,
    BetLimitsChanged = 310,
    #[serde(other)]
    Other,
}
