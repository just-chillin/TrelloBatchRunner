mod trello;
mod igdb;
mod auth;

use std::env;


#[tokio::main]
async fn main() {
    let trello_svc = trello::APIService::new();
    let igdb_svc = igdb::ApiService::new();
    let cards = trello_svc.get_cards("5ec5703c5000d38ad73c160e").await.unwrap();
    for card in cards {

    }
}
