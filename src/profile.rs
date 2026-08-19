use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::codec::{get_codec, Codec};
use crate::garbage::GarbageGenerator;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Fragment {
    pub index: usize,
    pub location: String,
    pub field: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Garbage {
    pub location: String,
    pub field: String,
    pub garbage_type: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CommandId {
    pub location: String,
    pub field: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProfileConfig {
    pub headers: Option<HashMap<String, String>>,
    pub serializer: String,
    pub chunk_size: usize,
    pub fragments: Vec<Fragment>,
    pub garbages: Option<Vec<Garbage>>,
    pub command_id: Option<CommandId>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ClientConfig {
    #[serde(flatten)]
    pub profile: ProfileConfig,
    pub api_url: Option<String>,
    pub jitter_min: Option<u64>,
    pub jitter_max: Option<u64>,
    pub next_command_route: Option<String>,
    pub command_response_route: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerConfig {
    #[serde(flatten)]
    pub profile: ProfileConfig,
}

pub struct Profile {
    pub config: ProfileConfig,
    codec: Box<dyn Codec>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DistributedPayload {
    pub body: HashMap<String, String>,
    pub headers: HashMap<String, String>,
    pub cookies: HashMap<String, String>,
    pub query: HashMap<String, String>,
}

pub struct RequestData {
    pub body: HashMap<String, String>,
    pub headers: HashMap<String, String>,
    pub cookies: HashMap<String, String>,
}

impl Profile {
    pub fn new(config: ProfileConfig) -> Self {
        let codec = get_codec(&config.serializer);
        Profile { config, codec }
    }

    pub fn encode(&self, text: &str, command_id: Uuid) -> DistributedPayload {
        let encoded = self.codec.encode(text);
        let fragmented = self.fragment(&encoded);
        self.distribute(&fragmented, command_id)
    }

    pub fn decode(&self, request: &RequestData) -> Result<String, String> {
        let extracted = self.extract(request);
        let defragmented = self.defragment(&extracted);
        self.codec.decode(&defragmented)
    }

    fn fragment(&self, text: &str) -> HashMap<String, String> {
        let mut out: HashMap<String, String> = HashMap::new();
        for f in &self.config.fragments {
            out.insert(f.field.clone(), String::new());
        }

        let fields: Vec<String> = self.config.fragments.iter().map(|f| f.field.clone()).collect();
        let chunk_size = self.config.chunk_size;

        for (i, chunk) in text.chars().collect::<Vec<_>>().chunks(chunk_size).enumerate() {
            let field = &fields[i % fields.len()];
            let chunk_str: String = chunk.iter().collect();
            out.entry(field.clone()).or_insert_with(String::new).push_str(&chunk_str);
        }

        out
    }

    fn defragment(&self, chunks: &HashMap<String, String>) -> String {
        let fields: Vec<String> = self.config.fragments.iter().map(|f| f.field.clone()).collect();
        let chunk_size = self.config.chunk_size;
        let mut result = String::new();
        let mut idx = 0;
        let empty = String::new();

        loop {
            let mut found = false;
            for field in &fields {
                let chunk_value = chunks.get(field).unwrap_or(&empty);
                let start = idx * chunk_size;
                let end = (idx + 1) * chunk_size;
                if start < chunk_value.len() {
                    result.push_str(&chunk_value[start..end.min(chunk_value.len())]);
                    found = true;
                }
            }
            if !found {
                break;
            }
            idx += 1;
        }

        result
    }

    fn distribute(&self, chunks: &HashMap<String, String>, command_id: Uuid) -> DistributedPayload {
        let mut payload = DistributedPayload {
            body: HashMap::new(),
            headers: HashMap::new(),
            cookies: HashMap::new(),
            query: HashMap::new(),
        };

        for frag in &self.config.fragments {
            if let Some(value) = chunks.get(&frag.field) {
                if !value.is_empty() {
                    let bucket = self.get_bucket_mut(&mut payload, &frag.location);
                    bucket.insert(frag.field.clone(), value.clone());
                }
            }
        }

        if let Some(cmd_id_cfg) = &self.config.command_id {
            let bucket = self.get_bucket_mut(&mut payload, &cmd_id_cfg.location);
            bucket.insert(cmd_id_cfg.field.clone(), command_id.to_string());
        }

        if let Some(garbages) = &self.config.garbages {
            for garbage in garbages {
                let garbage_value = GarbageGenerator::generate(&garbage.garbage_type);
                let bucket = self.get_bucket_mut(&mut payload, &garbage.location);
                bucket.insert(garbage.field.clone(), garbage_value);
            }
        }

        payload
    }

    fn extract(&self, request: &RequestData) -> HashMap<String, String> {
        let mut chunks = HashMap::new();
        for frag in &self.config.fragments {
            let value = match frag.location.as_str() {
                "body" => request.body.get(&frag.field),
                "header" => request.headers.get(&frag.field),
                "cookie" => request.cookies.get(&frag.field),
                _ => None,
            };
            if let Some(v) = value {
                chunks.insert(frag.field.clone(), v.clone());
            }
        }
        chunks
    }

    fn get_bucket_mut<'a>(&self, payload: &'a mut DistributedPayload, location: &str) -> &'a mut HashMap<String, String> {
        match location {
            "body" => &mut payload.body,
            "header" => &mut payload.headers,
            "cookie" => &mut payload.cookies,
            _ => &mut payload.body,
        }
    }
}
#[derive(Debug, Deserialize)]
pub struct ProfileFileRoot {
    pub client: ClientConfig,
    pub server: ServerConfig,
}

pub struct Profiles {
    pub client: Profile,
    pub server: Profile,
    pub api_url: Option<String>,
    pub jitter_min: Option<u64>,
    pub jitter_max: Option<u64>,
    pub next_command_route: Option<String>,
    pub command_response_route: Option<String>,
}

impl Profiles {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let content = include_str!("../profile.json");
        let root: ProfileFileRoot = serde_json::from_str(content)?;
        Ok(Profiles {
            client: Profile::new(root.client.profile),
            server: Profile::new(root.server.profile),
            api_url: root.client.api_url,
            jitter_min: root.client.jitter_min,
            jitter_max: root.client.jitter_max,
            next_command_route: root.client.next_command_route,
            command_response_route: root.client.command_response_route,
        })
    }
}
