use rand::Rng;
use serde_json::Value;

pub struct GarbageGenerator;

impl GarbageGenerator {
    pub fn json_data() -> String {
        let mut rng = rand::thread_rng();
        let keys = ["id", "type", "timestamp", "user", "data", "status", "event"];
        let num_pairs = rng.gen_range(2..=5);

        let mut obj = serde_json::Map::new();
        for _ in 0..num_pairs {
            let key = keys[rng.gen_range(0..keys.len())];
            let value: String = (0..8)
                .map(|_| (b'a' + rng.gen_range(0..26)) as char)
                .collect();
            obj.insert(key.to_string(), Value::String(value));
        }

        Value::Object(obj).to_string()
    }

    pub fn stream_header() -> String {
        "stream_response".to_string()
    }

    pub fn timestamp() -> String {
        format!("{}", chrono::Local::now().timestamp())
    }

    pub fn random_string() -> String {
        let mut rng = rand::thread_rng();
        (0..124)
            .map(|_| (b'a' + rng.gen_range(0..26)) as char)
            .collect()
    }

    pub fn generate(garbage_type: &str) -> String {
        match garbage_type {
            "json_data" => Self::json_data(),
            "stream_header" => Self::stream_header(),
            "timestamp" => Self::timestamp(),
            _ => Self::random_string(),
        }
    }
}
