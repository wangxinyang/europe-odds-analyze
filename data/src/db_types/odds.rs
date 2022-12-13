use derive_builder::Builder;
use sqlx::{types::BigDecimal, FromRow};
use std::str::FromStr;

#[derive(Debug, Clone, Builder, FromRow)]
pub struct Odds {
    #[builder(default)]
    pub id: i32,
    #[builder(default)]
    pub match_id: i32,
    #[builder(default)]
    pub bookmaker_id: i32,
    #[builder(default, setter(custom))]
    pub home_win_start: BigDecimal,
    #[builder(default, setter(custom))]
    pub draw_start: BigDecimal,
    #[builder(default, setter(custom))]
    pub away_win_start: BigDecimal,
    #[builder(default, setter(custom))]
    pub home_win_end: BigDecimal,
    #[builder(default, setter(custom))]
    pub draw_end: BigDecimal,
    #[builder(default, setter(custom))]
    pub away_win_end: BigDecimal,
    #[builder(default, setter(custom))]
    pub note: Option<String>,
}

impl OddsBuilder {
    pub fn home_win_start_setter(&mut self, value: &str) -> Self {
        self.home_win_start = Some(BigDecimal::from_str(value).unwrap());
        self.to_owned()
    }

    pub fn draw_start_setter(&mut self, value: &str) -> Self {
        self.draw_start = Some(BigDecimal::from_str(value).unwrap());
        self.to_owned()
    }

    pub fn away_win_start_setter(&mut self, value: &str) -> Self {
        self.away_win_start = Some(BigDecimal::from_str(value).unwrap());
        self.to_owned()
    }

    pub fn home_win_end_setter(&mut self, value: &str) -> Self {
        self.home_win_end = Some(BigDecimal::from_str(value).unwrap());
        self.to_owned()
    }

    pub fn draw_end_setter(&mut self, value: &str) -> Self {
        self.draw_end = Some(BigDecimal::from_str(value).unwrap());
        self.to_owned()
    }

    pub fn away_win_end_setter(&mut self, value: &str) -> Self {
        self.away_win_end = Some(BigDecimal::from_str(value).unwrap());
        self.to_owned()
    }
}
