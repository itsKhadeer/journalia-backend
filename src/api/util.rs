use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct HistoryboardQuery {
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Deserialize, Serialize)]
pub struct HistoryboardResponse {
    pub games: Vec<HistoryboardEntry>,
    pub last_page: i64,
}

#[derive(Deserialize, Serialize)]
pub struct HistoryboardEntry {
    pub opponent_user_name: String,
    pub is_attack: bool,
    pub damage_percent: i32,
    pub artifacts_taken: i32,
    pub trophies_taken: i32,
    pub match_id: i32,
    pub replay_availability: bool,
    pub avatar_id: i32,
}
