use rand::Rng;

pub trait Codec {
    fn encode(&self, text: &str) -> String;
    fn decode(&self, encoded: &str) -> Result<String, String>;
}

pub struct HexCodec;

impl Codec for HexCodec {
    fn encode(&self, text: &str) -> String {
        let mut rng = rand::thread_rng();
        let junk = "ABCDEFGHIJKLMNOPQRSTUVWXYZghijklmnopqrstuvwxyz+/=";
        let hex = hex::encode(text);

        let mut out = String::new();
        for c in hex.chars() {
            out.push(c);
            if rng.gen::<f32>() < 0.5 {
                let idx = rng.gen_range(0..junk.len());
                out.push(junk.chars().nth(idx).unwrap());
            }
        }
        out.push_str("==");
        out
    }

    fn decode(&self, encoded: &str) -> Result<String, String> {
        let clean: String = encoded.chars().filter(|c| "0123456789abcdef".contains(*c)).collect();
        hex::decode(&clean)
            .ok()
            .and_then(|b| String::from_utf8(b).ok())
            .ok_or_else(|| "Failed to decode hex".to_string())
    }
}

pub struct Base64Codec;

impl Base64Codec {
    fn noise() -> String {
        let mut rng = rand::thread_rng();
        let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let alphabet_chars: Vec<char> = alphabet.chars().collect();
        let n = rng.gen_range(4..33);

        let mut result = String::new();
        for _ in 0..n {
            let idx = rng.gen_range(0..alphabet_chars.len());
            result.push(alphabet_chars[idx]);
        }
        result
    }
}

impl Codec for Base64Codec {
    fn encode(&self, text: &str) -> String {
        let real = base64_encode(text);
        format!("{}.{}.{}", Self::noise(), real, Self::noise())
    }

    fn decode(&self, encoded: &str) -> Result<String, String> {
        let parts: Vec<&str> = encoded.split('.').collect();
        if parts.len() != 3 {
            return Err("Invalid base64 format".to_string());
        }
        base64_decode(parts[1])
    }
}

fn base64_encode(text: &str) -> String {
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let bytes = text.as_bytes();
    let mut result = String::new();

    for chunk in bytes.chunks(3) {
        let b1 = chunk[0];
        let b2 = chunk.get(1).copied().unwrap_or(0);
        let b3 = chunk.get(2).copied().unwrap_or(0);

        let n = ((b1 as u32) << 16) | ((b2 as u32) << 8) | (b3 as u32);

        result.push(ALPHABET[((n >> 18) & 0x3f) as usize] as char);
        result.push(ALPHABET[((n >> 12) & 0x3f) as usize] as char);

        if chunk.len() > 1 {
            result.push(ALPHABET[((n >> 6) & 0x3f) as usize] as char);
        } else {
            result.push('=');
        }

        if chunk.len() > 2 {
            result.push(ALPHABET[(n & 0x3f) as usize] as char);
        } else {
            result.push('=');
        }
    }

    result
}

fn base64_decode(encoded: &str) -> Result<String, String> {
    const ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    let mut bytes = Vec::new();
    let chars: Vec<char> = encoded.chars().filter(|c| !c.is_whitespace()).collect();

    for chunk in chars.chunks(4) {
        if chunk.len() < 2 {
            continue;
        }

        let c1 = ALPHABET.find(chunk[0]).ok_or("Invalid character")?;
        let c2 = ALPHABET.find(chunk[1]).ok_or("Invalid character")?;
        let c3 = if chunk.len() > 2 && chunk[2] != '=' { ALPHABET.find(chunk[2]).ok_or("Invalid character")? } else { 0 };
        let c4 = if chunk.len() > 3 && chunk[3] != '=' { ALPHABET.find(chunk[3]).ok_or("Invalid character")? } else { 0 };

        let n = ((c1 << 18) | (c2 << 12) | (c3 << 6) | c4) as u32;

        bytes.push(((n >> 16) & 0xff) as u8);
        if chunk.len() > 2 && chunk[2] != '=' {
            bytes.push(((n >> 8) & 0xff) as u8);
        }
        if chunk.len() > 3 && chunk[3] != '=' {
            bytes.push((n & 0xff) as u8);
        }
    }

    String::from_utf8(bytes).map_err(|e| e.to_string())
}

pub fn get_codec(serializer: &str) -> Box<dyn Codec> {
    match serializer {
        "base64" => Box::new(Base64Codec),
        _ => Box::new(HexCodec),
    }
}
