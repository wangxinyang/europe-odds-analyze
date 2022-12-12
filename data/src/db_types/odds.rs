use derive_builder::Builder;

#[derive(Debug, Clone, Builder)]
pub struct Odds {
    #[builder(default)]
    pub id: i32,
    #[builder(default)]
    pub match_id: i32,
    #[builder(default)]
    pub bookmaker_id: i32,
    #[builder(default)]
    pub home_win_start: f64,
    #[builder(default)]
    pub draw_start: f64,
    #[builder(default)]
    pub away_win_start: f64,
    #[builder(default)]
    pub home_win_end: f64,
    #[builder(default)]
    pub draw_end: f64,
    #[builder(default)]
    pub away_win_end: f64,
    #[builder(setter(into, strip_option))]
    pub note: Option<String>,
}
