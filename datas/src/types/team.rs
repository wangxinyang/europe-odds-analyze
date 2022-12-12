use chrono::NaiveDateTime;

#[derive(Debug)]
pub struct Team {
    pub id: i32,
    pub league_id: i32,
    pub name: String,
    pub note: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
