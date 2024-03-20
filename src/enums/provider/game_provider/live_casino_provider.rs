use parse_display::Display;

#[derive(Display, Debug)]
pub enum LiveCasinoProvider {
    #[display("sexy")]
    Sexy,
    #[display("pragmatic")]
    Pragmatic,
    #[display("sa")]
    SA,
    #[display("ag")]
    AG,
    #[display("pretty")]
    Pretty,
    #[display("dream")]
    Dream,
    #[display("allbet")]
    AllBet,
    #[display("xtream")]
    Xtream,
    #[display("bet_games_live_casino")]
    BetGames,
    #[display("big_gaming_live_casino")]
    BigGaming,
    #[display("mg_live_casino")]
    MG,
    #[display("green_dragon")]
    GreenDragon,
    #[display("wm_live_casino")]
    WM,
}
