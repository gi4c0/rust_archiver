mod live_casino_provider;
mod lottery;
mod online_casino_provider;
mod slot_provider;
mod sport;

use std::str::FromStr;

use anyhow::{anyhow, Result};
pub use live_casino_provider::*;
pub use lottery::*;
pub use online_casino_provider::*;
pub use slot_provider::*;
pub use sport::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameProvider {
    LiveCasino(LiveCasinoProvider),
    Slot(SlotProvider),
    Lottery(Lottery),
    Sport(Sportsbook),
    OnlineCasino(OnlineCasinoProvider),
}

impl FromStr for GameProvider {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self> {
        match value {
            "sexy" => Ok(GameProvider::LiveCasino(LiveCasinoProvider::Sexy)),
            "pragmatic" => Ok(GameProvider::LiveCasino(LiveCasinoProvider::Pragmatic)),
            "sa" => Ok(GameProvider::LiveCasino(LiveCasinoProvider::SA)),
            "ag" => Ok(GameProvider::LiveCasino(LiveCasinoProvider::AG)),
            "pretty" => Ok(GameProvider::LiveCasino(LiveCasinoProvider::Pretty)),
            "dream" => Ok(GameProvider::LiveCasino(LiveCasinoProvider::Dream)),
            "allbet" => Ok(GameProvider::LiveCasino(LiveCasinoProvider::AllBet)),
            "xtream" => Ok(GameProvider::LiveCasino(LiveCasinoProvider::Xtream)),
            "bet_games_live_casino" => Ok(GameProvider::LiveCasino(LiveCasinoProvider::BetGames)),
            "big_gaming_live_casino" => Ok(GameProvider::LiveCasino(LiveCasinoProvider::BigGaming)),
            "mg_live_casino" => Ok(GameProvider::LiveCasino(LiveCasinoProvider::MG)),
            "green_dragon" => Ok(GameProvider::LiveCasino(LiveCasinoProvider::GreenDragon)),
            "wm_live_casino" => Ok(GameProvider::LiveCasino(LiveCasinoProvider::WM)),
            "evoplay_online_casino" => {
                Ok(GameProvider::OnlineCasino(OnlineCasinoProvider::Evoplay))
            }
            "bet_games_online_casino" => {
                Ok(GameProvider::OnlineCasino(OnlineCasinoProvider::BetGames))
            }
            "pg_online_casino" => Ok(GameProvider::OnlineCasino(OnlineCasinoProvider::PG)),
            "ambpoker_online_casino" => {
                Ok(GameProvider::OnlineCasino(OnlineCasinoProvider::AmbPoker))
            }
            "slot_xo_online_casino" => Ok(GameProvider::OnlineCasino(OnlineCasinoProvider::SlotXO)),
            "dragoon_online_casino" => {
                Ok(GameProvider::OnlineCasino(OnlineCasinoProvider::Dragoon))
            }
            "jili_online_casino" => Ok(GameProvider::OnlineCasino(OnlineCasinoProvider::Jili)),
            "mg_online_casino" => Ok(GameProvider::OnlineCasino(OnlineCasinoProvider::MG)),
            "gaming_soft_online_casino" => {
                Ok(GameProvider::OnlineCasino(OnlineCasinoProvider::GamingSoft))
            }
            "ambslot_online_casino" => {
                Ok(GameProvider::OnlineCasino(OnlineCasinoProvider::AmbSlot))
            }
            "habanero_online_casino" => {
                Ok(GameProvider::OnlineCasino(OnlineCasinoProvider::Habanero))
            }
            "astro_tech_online_casino" => {
                Ok(GameProvider::OnlineCasino(OnlineCasinoProvider::AstroTech))
            }
            "iconic_gaming_online_casino" => Ok(GameProvider::OnlineCasino(
                OnlineCasinoProvider::IconicGaming,
            )),
            "funta_online_casino" => Ok(GameProvider::OnlineCasino(OnlineCasinoProvider::FunTa)),
            "all_way_spin_online_casino" => {
                Ok(GameProvider::OnlineCasino(OnlineCasinoProvider::AllWaySpin))
            }
            "ka_gaming_online_casino" => {
                Ok(GameProvider::OnlineCasino(OnlineCasinoProvider::KaGaming))
            }
            "kingmaker" => Ok(GameProvider::OnlineCasino(OnlineCasinoProvider::Kingmaker)),
            "netent_online_casino" => Ok(GameProvider::OnlineCasino(OnlineCasinoProvider::Netent)),
            "red_tiger_online_casino" => {
                Ok(GameProvider::OnlineCasino(OnlineCasinoProvider::RedTiger))
            }
            "arcadia" => Ok(GameProvider::OnlineCasino(OnlineCasinoProvider::Arcadia)),
            "ninja_online_casino" => Ok(GameProvider::OnlineCasino(OnlineCasinoProvider::Ninja)),
            "kiss_online_casino" => Ok(GameProvider::OnlineCasino(OnlineCasinoProvider::Kiss)),
            "slot_xo_slot" => Ok(GameProvider::Slot(SlotProvider::SlotXO)),
            "pg_slot" => Ok(GameProvider::Slot(SlotProvider::PG)),
            "evoplay_slot" => Ok(GameProvider::Slot(SlotProvider::Evoplay)),
            "simpleplay" => Ok(GameProvider::Slot(SlotProvider::SimplePlay)),
            "spade" => Ok(GameProvider::Slot(SlotProvider::Spade)),
            "ambpoker_slot" => Ok(GameProvider::Slot(SlotProvider::AmbPoker)),
            "ambslot_slot" => Ok(GameProvider::Slot(SlotProvider::AmbSlot)),
            "pragmatic_slot" => Ok(GameProvider::Slot(SlotProvider::Pragmatic)),
            "dragoon_slot" => Ok(GameProvider::Slot(SlotProvider::Dragoon)),
            "jili_slot" => Ok(GameProvider::Slot(SlotProvider::Jili)),
            "mannaplay" => Ok(GameProvider::Slot(SlotProvider::MannaPlay)),
            "royal_slot_gaming" => Ok(GameProvider::Slot(SlotProvider::RoyalSlotGaming)),
            "mg_slot" => Ok(GameProvider::Slot(SlotProvider::MG)),
            "upg_slot" => Ok(GameProvider::Slot(SlotProvider::UPG)),
            "gaming_soft_slot" => Ok(GameProvider::Slot(SlotProvider::GamingSoft)),
            "ygg_slot" => Ok(GameProvider::Slot(SlotProvider::YGG)),
            "slotmill" => Ok(GameProvider::Slot(SlotProvider::Slotmill)),
            "habanero_slot" => Ok(GameProvider::Slot(SlotProvider::Habanero)),
            "astro_tech_slot" => Ok(GameProvider::Slot(SlotProvider::AstroTech)),
            "wazdan" => Ok(GameProvider::Slot(SlotProvider::Wazdan)),
            "all_way_spin_slot" => Ok(GameProvider::Slot(SlotProvider::AllWaySpin)),
            "booongo" => Ok(GameProvider::Slot(SlotProvider::Booongo)),
            "funta_slot" => Ok(GameProvider::Slot(SlotProvider::FunTa)),
            "iconic_gaming_slot" => Ok(GameProvider::Slot(SlotProvider::IconicGaming)),
            "gamatron" => Ok(GameProvider::Slot(SlotProvider::Gamatron)),
            "ka_gaming_slot" => Ok(GameProvider::Slot(SlotProvider::KaGaming)),
            "play_son" => Ok(GameProvider::Slot(SlotProvider::PlaySon)),
            "wm_slot" => Ok(GameProvider::Slot(SlotProvider::WM)),
            "ameba" => Ok(GameProvider::Slot(SlotProvider::Ameba)),
            "i8" => Ok(GameProvider::Slot(SlotProvider::I8)),
            "creative_gaming" => Ok(GameProvider::Slot(SlotProvider::CreativeGaming)),
            "red_tiger_slot" => Ok(GameProvider::Slot(SlotProvider::RedTiger)),
            "netent_slot" => Ok(GameProvider::Slot(SlotProvider::Netent)),
            "relax" => Ok(GameProvider::Slot(SlotProvider::Relax)),
            "ninja_slot" => Ok(GameProvider::Slot(SlotProvider::Ninja)),
            "kiss_slot" => Ok(GameProvider::Slot(SlotProvider::Kiss)),
            "spinix_slot" => Ok(GameProvider::Slot(SlotProvider::Spinix)),
            "dragon_gaming" => Ok(GameProvider::Slot(SlotProvider::DragonGaming)),
            "hacksaw" => Ok(GameProvider::Slot(SlotProvider::Hacksaw)),
            "thailotto" => Ok(GameProvider::Lottery(Lottery::Thai)),
            "laoslotto" => Ok(GameProvider::Lottery(Lottery::Lao)),
            "hanoylotto" => Ok(GameProvider::Lottery(Lottery::Hanoi)),
            "hanoylottovip" => Ok(GameProvider::Lottery(Lottery::HanoiVip)),
            "baaclotto" => Ok(GameProvider::Lottery(Lottery::Baac)),
            "gsblotto" => Ok(GameProvider::Lottery(Lottery::Gsb)),
            "pingponglotto" => Ok(GameProvider::Lottery(Lottery::PingPong)),
            "laoslotto_set" => Ok(GameProvider::Lottery(Lottery::LaoSet)),
            "yeekeelotto" => Ok(GameProvider::Lottery(Lottery::Yeekee)),
            "malaylotto" => Ok(GameProvider::Lottery(Lottery::Malaysian)),
            "hanoylotto_set" => Ok(GameProvider::Lottery(Lottery::HanoiSet)),
            "hanoylottovip_set" => Ok(GameProvider::Lottery(Lottery::HanoiVipSet)),
            "hanoylottospecial_set" => Ok(GameProvider::Lottery(Lottery::HanoiSpecialSet)),
            "malaylotto_set" => Ok(GameProvider::Lottery(Lottery::MalaySet)),
            "hanoylottospecial" => Ok(GameProvider::Lottery(Lottery::HanoiSpecial)),
            "stockkorea" => Ok(GameProvider::Lottery(Lottery::StockKorea)),
            "stockchina" => Ok(GameProvider::Lottery(Lottery::StockChina)),
            "stockdowjones" => Ok(GameProvider::Lottery(Lottery::StockDowJones)),
            "stockdowjonesvip" => Ok(GameProvider::Lottery(Lottery::StockDowJonesVip)),
            "stocktaiwan" => Ok(GameProvider::Lottery(Lottery::StockTaiwan)),
            "stockengland" => Ok(GameProvider::Lottery(Lottery::StockBritish)),
            "stockindia" => Ok(GameProvider::Lottery(Lottery::StockIndia)),
            "stockhangseng" => Ok(GameProvider::Lottery(Lottery::StockHangSeng)),
            "stockegypt" => Ok(GameProvider::Lottery(Lottery::StockEgyptian)),
            "stocknikkei" => Ok(GameProvider::Lottery(Lottery::StockNikkei)),
            "stocksingapore" => Ok(GameProvider::Lottery(Lottery::StockSingapore)),
            "stockthai" => Ok(GameProvider::Lottery(Lottery::StockThai)),
            "stockgerman" => Ok(GameProvider::Lottery(Lottery::StockGerman)),
            "stockrussia" => Ok(GameProvider::Lottery(Lottery::StockRussian)),
            "stock" => Ok(GameProvider::Lottery(Lottery::StockCollect)),
            "zodiaclotto" => Ok(GameProvider::Lottery(Lottery::Zodiac)),
            "pingponglotto2" => Ok(GameProvider::Lottery(Lottery::PingPong2)),
            "pingponglotto3" => Ok(GameProvider::Lottery(Lottery::PingPong3)),
            "pingponglotto6" => Ok(GameProvider::Lottery(Lottery::PingPong6)),
            "hanoylottostar_set" => Ok(GameProvider::Lottery(Lottery::HanoiStarSet)),
            "hanoylottostar" => Ok(GameProvider::Lottery(Lottery::HanoiStar)),
            "hanoylottotv" => Ok(GameProvider::Lottery(Lottery::HanoiTv)),
            "hanoylottotv_set" => Ok(GameProvider::Lottery(Lottery::HanoiTvSet)),
            "laoslottoextra" => Ok(GameProvider::Lottery(Lottery::LaoExtra)),
            "laoslottoextra_set" => Ok(GameProvider::Lottery(Lottery::LaoExtraSet)),
            "laoslottohd" => Ok(GameProvider::Lottery(Lottery::LaoHd)),
            "laoslottohd_set" => Ok(GameProvider::Lottery(Lottery::LaoHdSet)),
            "laoslottotv" => Ok(GameProvider::Lottery(Lottery::LaoTv)),
            "laoslottotv_set" => Ok(GameProvider::Lottery(Lottery::LaoTvSet)),
            "single_live" => Ok(GameProvider::Sport(Sportsbook::SingleLive)),
            "single_non_live" => Ok(GameProvider::Sport(Sportsbook::SingleNonLive)),
            "combo" => Ok(GameProvider::Sport(Sportsbook::Combo)),
            "parlay" => Ok(GameProvider::Sport(Sportsbook::Parlay)),
            _ => Err(anyhow!("Unexpected provider name: '{value}'")),
        }
    }
}

impl AsRef<str> for GameProvider {
    fn as_ref(&self) -> &str {
        match self {
            GameProvider::Slot(p) => p.as_ref(),
            GameProvider::OnlineCasino(p) => p.as_ref(),
            GameProvider::LiveCasino(p) => p.as_ref(),
            GameProvider::Sport(p) => p.as_ref(),
            GameProvider::Lottery(p) => p.as_ref(),
        }
    }
}
