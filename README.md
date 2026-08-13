TRIGGER WARNING

This repository contains C2 (Command & Control) implant code for security research and authorized penetration testing purposes only.

# C2-Implant

Remote code execution agent written in Rust for authorized security testing.

## Overview

This implant is designed to:
- Execute commands remotely via the C2 server
- Bypass proxy configurations (this branch)
- Handle HTTP communication with the team server
- Maintain connection reliability with configurable timeouts

## Branches

- **implant-simple**: Basic implant with direct HTTP communication
- **bypass-proxy**: Enhanced implant with proxy bypass capabilities (you are here)

## Building

```bash
cargo build --release
```

## Configuration

Edit `.env` file:
```
SERVER_URL=http://your-c2-server:port
POLL_INTERVAL=5000
PROXY_BYPASS=true
```

## Components

- `src/main.rs` - Entry point and main loop
- `src/executor.rs` - Command execution engine
- `src/http.rs` - C2 communication protocol with proxy bypass
- `src/models.rs` - Data structures

## Security Notice

This code demonstrates security concepts. Use responsibly and only in authorized contexts.
