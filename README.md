TRIGGER WARNING

This repository contains C2 (Command & Control) implant code for security research and authorized penetration testing purposes only.

# C2-Implant

Remote code execution agent written in Rust for authorized security testing.

## Overview

This implant is designed to:
- Execute commands remotely via the C2 server
- Bypass proxy configurations (on `bypass-proxy` branch)
- Handle HTTP communication with the team server
- Maintain connection reliability with configurable timeouts

## Branches

- **implant-simple**: Basic implant with direct HTTP communication
- **bypass-proxy**: Enhanced implant with proxy bypass capabilities

## Building

### Linux to Windows (MinGW)

```bash
rustup target add x86_64-pc-windows-gnu
cargo build --target x86_64-pc-windows-gnu --release
# Output: target/x86_64-pc-windows-gnu/release/agent.exe
```

### Linux to Linux

```bash
cargo build --release
# Output: target/release/agent
```

### With Environment Variables (no .env needed)

```bash
BASE_URL=https://10.10.10.10 POLL_INTERVAL=5000 cargo build --target x86_64-pc-windows-gnu --release
```

## Configuration

Create `.env` file in project root:
```
SERVER_URL=https://your-c2-server:port
POLL_INTERVAL=5000
```

Or pass as environment variables during build (see above).

## Components

- `src/main.rs` - Entry point and main loop
- `src/executor.rs` - Command execution engine
- `src/http.rs` - C2 communication protocol
- `src/models.rs` - Data structures

## Security Notice

This code demonstrates security concepts. Use responsibly and only in authorized contexts.
