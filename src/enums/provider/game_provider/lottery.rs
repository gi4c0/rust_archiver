use strum_macros::{AsRefStr, EnumString};

#[derive(Debug, AsRefStr, EnumString)]
pub enum Lottery {
    #[strum(serialize = "thailotto")]
    Thai,
    #[strum(serialize = "laoslotto")]
    Lao,
    #[strum(serialize = "hanoylotto")]
    Hanoi,
    #[strum(serialize = "hanoylottovip")]
    HanoiVip,
    #[strum(serialize = "baaclotto")]
    Baac,
    #[strum(serialize = "gsblotto")]
    Gsb,
    #[strum(serialize = "pingponglotto")]
    PingPong,
    #[strum(serialize = "laoslotto_set")]
    LaoSet,
    #[strum(serialize = "yeekeelotto")]
    Yeekee,
    #[strum(serialize = "malaylotto")]
    Malaysian,
    #[strum(serialize = "hanoylotto_set")]
    HanoiSet,
    #[strum(serialize = "hanoylottovip_set")]
    HanoiVipSet,
    #[strum(serialize = "hanoylottospecial_set")]
    HanoiSpecialSet,
    #[strum(serialize = "malaylotto_set")]
    MalaySet,
    #[strum(serialize = "hanoylottospecial")]
    HanoiSpecial,
    #[strum(serialize = "stockkorea")]
    StockKorea,
    #[strum(serialize = "stockchina")]
    StockChina,
    #[strum(serialize = "stockdowjones")]
    StockDowJones,
    #[strum(serialize = "stockdowjonesvip")]
    StockDowJonesVip,
    #[strum(serialize = "stocktaiwan")]
    StockTaiwan,
    #[strum(serialize = "stockengland")]
    StockBritish,
    #[strum(serialize = "stockindia")]
    StockIndia,
    #[strum(serialize = "stockhangseng")]
    StockHangSeng,
    #[strum(serialize = "stockegypt")]
    StockEgyptian,
    #[strum(serialize = "stocknikkei")]
    StockNikkei,
    #[strum(serialize = "stocksingapore")]
    StockSingapore,
    #[strum(serialize = "stockthai")]
    StockThai,
    #[strum(serialize = "stockgerman")]
    StockGerman,
    #[strum(serialize = "stockrussia")]
    StockRussian,
    #[strum(serialize = "stock")]
    StockCollect,
    #[strum(serialize = "zodiaclotto")]
    Zodiac,
    #[strum(serialize = "pingponglotto2")]
    PingPong2,
    #[strum(serialize = "pingponglotto3")]
    PingPong3,
    #[strum(serialize = "pingponglotto6")]
    PingPong6,
    #[strum(serialize = "hanoylottostar_set")]
    HanoiStarSet,
    #[strum(serialize = "hanoylottostar")]
    HanoiStar,
    #[strum(serialize = "hanoylottotv")]
    HanoiTv,
    #[strum(serialize = "hanoylottotv_set")]
    HanoiTvSet,
    #[strum(serialize = "laoslottoextra")]
    LaoExtra,
    #[strum(serialize = "laoslottoextra_set")]
    LaoExtraSet,
    #[strum(serialize = "laoslottohd")]
    LaoHd,
    #[strum(serialize = "laoslottohd_set")]
    LaoHdSet,
    #[strum(serialize = "laoslottotv")]
    LaoTv,
    #[strum(serialize = "laoslottotv_set")]
    LaoTvSet,
}
