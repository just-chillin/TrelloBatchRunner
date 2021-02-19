use serde::{Deserialize, Serialize};
use std::env;


struct Auth {
    key: String,
    token: String,
}
impl Auth {
    fn new(service_name: &'static str) -> Self {
        let get_var = |key: &'static str| -> String {
            env::var(format!("{}_{}", service_name, key))
                .unwrap_or_else(|_| panic!("{} env var not found!", key))
        };
        Self {
            key: get_var("KEY"),
            token: get_var("TOKEN"),
        }
    }
}

struct TrelloAPIService {
    auth: Auth,
}
impl TrelloAPIService {
    fn new() -> Self {
        Self {
            auth: Auth::new("Trello"),
        }
    }
    async fn get_cards(&self, board_id: &str) -> Result<Vec<Card>, reqwest::Error> {
        let url = format!(
            r"https://api.trello.com/1/boards/{}/cards?key={}&token={}",
            board_id, self.auth.key, self.auth.token
        );
        let req = reqwest::get(&url).await?;
        req.json::<Vec<Card>>().await
    }
}

#[derive(Serialize, Deserialize)]
struct Card {
    id: String,
    name: String,
}

fn get_cards_url(board_id: &str, auth: (String, String)) {
    let url = format!(
        r"https://api.trello.com/1/boards/{}/cards?key={}&token={}",
        board_id, auth.0, auth.1
    );
}

struct IGDBApiService {
    auth: Auth,
}
impl IGDBApiService {
    fn new() -> Self {
        Self {
            auth: Auth::new("IGDB"),
        }
    }
}

#[tokio::main]
async fn main() {
    let trello_svc = TrelloAPIService::new();
    let cards = trello_svc.get_cards("5ec5703c5000d38ad73c160e").await;
}
