use std::net::Ipv4Addr;

use anyhow::{bail, Context, Result};
use reqwest::{header::HeaderMap, Client};
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::{
    archiver::bets::loader::Bet,
    enums::{provider::ProviderGameKind, Language},
    helpers::crypto,
    types::{
        Currency, ProviderBetID, ProviderGameVendorID, ProviderGameVendorLabel, Url, Username,
    },
};

#[derive(Deserialize, Serialize, Debug)]
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

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoyalSlotGamingGameConfig {
    pub game_label: ProviderGameVendorLabel,
    #[serde(rename = "gameID")]
    pub game_id: ProviderGameVendorID,
    #[serde(rename = "type")]
    pub game_type: ProviderGameKind,
}

#[derive(Debug)]
pub struct Connector {
    config: RoyalSlotGamingConfig,
    games_by_vendor_id: FxHashMap<ProviderGameVendorID, RoyalSlotGamingGameConfig>,
}

impl Connector {
    pub fn new(
        config: RoyalSlotGamingConfig,
        games_by_vendor_id: FxHashMap<ProviderGameVendorID, RoyalSlotGamingGameConfig>,
    ) -> Self {
        Self {
            config,
            games_by_vendor_id,
        }
    }

    pub async fn get_game_round_history(&self, bet: &Bet, lang: Option<Language>) -> Result<Url> {
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
            .headers(self.generate_headers(&hash)?)
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

    fn generate_headers(&self, des: &str) -> Result<HeaderMap> {
        let mut headers = HeaderMap::new();
        let unix_timestamp = OffsetDateTime::now_utc().unix_timestamp();

        headers.insert(
            "X-API-ClientID",
            self.config
                .client_id
                .parse()
                .context("[RSG] Failed to convert client_id to header value")?,
        );

        headers.insert(
            "X-API-Timestamp",
            unix_timestamp
                .to_string()
                .parse()
                .context("[RSG] Failed to convert unix_timestamp to header value")?,
        );

        let signature = crypto::md5(format!(
            "{}{}{}{}",
            &self.config.client_id, &self.config.client_secret, unix_timestamp, des
        ));

        headers.insert(
            "X-API-Signature",
            signature
                .parse()
                .context("[RSG] Failed to convert signature to header value")?,
        );

        Ok(headers)
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
    _error_code: i64,
    _error_message: String,
    _timestamp: u64,
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
