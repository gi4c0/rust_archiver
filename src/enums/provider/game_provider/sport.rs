use parse_display::Display;

#[derive(Debug, Display)]
#[display(style = "snake_case")]
pub enum Sportsbook {
    SingleLive,
    SingleNonLive,
    Combo,
    Parlay,
}
