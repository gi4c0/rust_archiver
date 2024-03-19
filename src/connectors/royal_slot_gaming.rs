use std::{collections::HashMap, net::Ipv4Addr};

use anyhow::{bail, Context};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::{
    archiver::bets::loader::Bet,
    enums::{provider::ProviderGameKind, Language},
    helpers::crypto,
    types::{Currency, ProviderBetID, ProviderGameLabel, ProviderGameVendorID, Url, Username},
};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RoyalSlotGamingConfig {
    pub web_id: String,
    pub system_code: String,
    #[serde(rename = "clientID")]
    pub client_id: String,
    pub client_secret: String,
    pub api_url: Url,
    pub des_key: String,
    #[serde(rename = "desIV")]
    pub des_iv: String,
    pub ip_list: Vec<Ipv4Addr>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RoyalSlotGamingGameConfig {
    pub game_label: ProviderGameLabel,
    #[serde(rename = "gameID")]
    pub game_id: ProviderGameVendorID,
    // TODO: SELECT type AS "game_type"
    pub game_type: ProviderGameKind,
}

#[derive(Debug)]
pub struct Connector {
    config: RoyalSlotGamingConfig,
    games_by_vendor_id: HashMap<ProviderGameVendorID, RoyalSlotGamingGameConfig>,
}

impl Connector {
    pub fn new(
        config: RoyalSlotGamingConfig,
        games_by_vendor_id: HashMap<ProviderGameVendorID, RoyalSlotGamingGameConfig>,
    ) -> Self {
        Self {
            config,
            games_by_vendor_id,
        }
    }

    pub async fn get_game_round_history(
        &self,
        bet: &Bet,
        lang: Option<Language>,
    ) -> anyhow::Result<Url> {
        let game = self.games_by_vendor_id.get(&bet.provider_game_vendor_id);

        let game_type: u8 = game.map_or(1, |g| {
            if g.game_type == ProviderGameKind::Fishing {
                2
            } else {
                1
            }
        });

        let number_game_id: u64 = bet.provider_bet_id.clone().0.parse().with_context(|| {
            format!(
                "Expected royal slot bet to have a number for game id. Provider bet ID: {}",
                &bet.provider_bet_id
            )
        })?;

        let payload = RoundHistoryPayload {
            user_id: bet.username.clone(),
            language: get_provider_language(lang.unwrap_or(Language::English)),
            game_id: number_game_id,
            currency: bet.currency.clone(),
            game_type,
            sequen_number: bet.provider_bet_id.clone(),
            web_id: self.config.web_id.clone(),
            system_code: self.config.system_code.clone(),
        };

        let json_payload = serde_json::ser::to_string(&payload)
            .context("Failed to serialize RoyalSlotGaming RoundHistoryPayload")?;

        let hash =
            crypto::des_cbc_encrypt(&json_payload, &self.config.des_key, &self.config.des_iv)
                .context("Failed to DES-CBC encrypt payload for RoyalSlotGaming")?;

        let encrypted_response: String = Client::new()
            .post(format!(
                "{}/Player/GetGameMinDetailURLTokenBySeq",
                &self.config.api_url
            ))
            .body(format!("Msg={hash}"))
            .send()
            .await
            .with_context(|| {
                format!(
                    "Failed to fetch bet details RoyalSlotGaming: {}",
                    &bet.provider_bet_id
                )
            })?
            .text()
            .await
            .with_context(|| {
                format!(
                    "Failed to extract response from bet details RoyalSlotGaming: {}",
                    &bet.provider_bet_id
                )
            })?;

        let decrypted: String = crypto::des_cbc_decrypt(
            &encrypted_response,
            &self.config.des_key,
            &self.config.des_iv,
        )
        .with_context(|| {
            format!(
                "Failed to decrypt RoyalSlotGaming bet details response: {}",
                &bet.provider_bet_id
            )
        })?;

        let parsed: ApiResponse<UrlResponse> =
            serde_json::from_str(&decrypted).with_context(|| {
                format!(
                    "Failed to parse RoyalSlotGaming bet details json: {}",
                    &bet.provider_bet_id
                )
            })?;

        match parsed.data {
            Some(data) => Ok(data.url),
            _ => bail!("RoyalSlotGaming bet details API Error: '{}'", &decrypted),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
struct RoundHistoryPayload {
    user_id: Username,
    game_id: u64,
    game_type: u8,
    language: &'static str,
    sequen_number: ProviderBetID,
    currency: Currency,
    system_code: String,
    web_id: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct ApiResponse<T> {
    error_code: i64,
    error_message: String,
    timestamp: u64,
    data: Option<T>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
struct UrlResponse {
    url: Url,
}

fn get_provider_language(lang: Language) -> &'static str {
    match lang {
        Language::Chinese => "zh-CN",
        Language::English => "en-US",
        Language::Indonesian => "id-ID",
        Language::Malay => "en-US",
        Language::Thai => "th-TH",
        Language::Vietnamese => "vi-VN",
        Language::Laotian => "en-US",
        Language::Tagalog => "en-US",
        Language::Hindi => "en-US",
        Language::Korean => "en-US",
        Language::Japanese => "en-US",
    }
}
