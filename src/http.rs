use reqwest::Client;
use crate::models::{Command, CommandResponse};

pub async fn get_command(client: &Client, base_url: &str) -> Option<Command> {
    let resp = client.get(format!("{base_url}/commands/next")).send().await;

    match resp {
        Ok(r) if r.status().is_success() => r.json::<Command>().await.ok(),
        _ => None,
    }
}

pub async fn send_response(client: &Client, base_url: &str, id: i32, response: &CommandResponse) {
    let _ = client
        .put(format!("{base_url}/commands/{id}"))
        .json(response)
        .send()
        .await;
}