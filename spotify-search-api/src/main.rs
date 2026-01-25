mod models;
mod utils;

use reqwest;
use reqwest::header::{ACCEPT, CONTENT_TYPE, USER_AGENT};
use crate::models::common::*;
use crate::utils::helper::*;
use std::env;


#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let search_query = &args[1];
    let url = format!("https://musicbrainz.org/ws/2/recording?query={}&fmt=json", search_query);
    let client = reqwest::Client::new();
    let response = client
    .get(url)
    .header(CONTENT_TYPE, "application/json")
    .header(ACCEPT, "application/json")
    .header(USER_AGENT, "MusicBrainzApi/0.1.0")
    .send()
    .await
    .unwrap();

    let _ = print_tracks(&response.json::<APIResponse>().await.unwrap()).await;

}
