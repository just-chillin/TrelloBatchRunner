use serde::{Deserialize, Serialize};
use std::env;
use crate::auth::Auth;

#[derive(Serialize, Deserialize)]
pub struct Card {
    id: String,
    name: String,
}

pub struct APIService {
    auth: Auth,
}
impl APIService {
    pub fn new() -> Self {
        Self {
            auth: Auth::new("TRELLO"),
        }
    }
    pub async fn get_cards(&self, board_id: &str) -> Result<Vec<Card>, reqwest::Error> {
        let url = format!(
            r"https://api.trello.com/1/boards/{}/cards?key={}&token={}",
            board_id, self.auth.client_id, self.auth.secret
        );
        let req = reqwest::get(&url).await?;
        req.json::<Vec<Card>>().await
    }
}
