#[derive(Debug)]
pub struct Odds {
    pub id: i32,
    pub match_id: i32,
    pub bookmaker_id: i32,
    pub home_win_start: Option<f64>,
    pub draw_start: Option<f64>,
    pub away_win_start: Option<f64>,
    pub home_win_end: Option<f64>,
    pub draw_end: Option<f64>,
    pub away_win_end: Option<f64>,
    pub note: String,
}
