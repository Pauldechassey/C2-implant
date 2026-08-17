TRIGGER WARNING

This repository contains C2 (Command & Control) implant code for security research and authorized penetration testing purposes only.

# C2-Implant

Remote code execution agent written in Rust for authorized security testing.

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

### With Environment Variables

```bash
BASE_URL=http://10.10.10.10:8000 cargo build --target x86_64-pc-windows-gnu --release
```

## Configuration

Pass as environment variables during build:
```
BASE_URL=http://your-c2-server:port
JITTER_MIN=3
JITTER_MAX=7
```

Defaults: localhost:8000, jitter 3-7 seconds

## Security Notice

This code demonstrates security concepts. Use responsibly and only in authorized contexts.
