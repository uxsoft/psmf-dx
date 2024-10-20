use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct SearchResponse {
    pub items: Vec<Team>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Team {
    pub name: String,
    pub description: String,
    pub url: String,
    pub uid: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Field {
    pub key: String,
    pub name: String,
    pub address: String,
    pub notes: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Match {
    pub round: Option<u16>,
    pub date: String,
    pub court: String,
    pub home_team: String,
    pub guest_team: String,
    pub score: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TeamDetails {
    pub next_match: Option<Match>,
    pub results: Vec<Match>,
    pub matches: Vec<Match>,
}
