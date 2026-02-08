# kdash - Kubernetes Dashboard

Real-time TUI dashboard for Kubernetes clusters.

## Features

- Live pod status monitoring
- Auto-refresh every 5 seconds
- Color-coded pod states (Running/Pending/Failed)
- Interactive pod selection (↑↓ arrow keys)
- Real-time log streaming (press `l`)
- Shows namespace, restarts, and age
- Minimal resource usage

## Installation

```bash
cargo build --release
sudo cp target/release/kdash /usr/local/bin/
```

## Usage

```bash
kdash
```

**Controls:**
- `q` - Quit
- `r` - Manual refresh
- `l` - Toggle logs panel
- `↑` - Select previous pod
- `↓` - Select next pod

## Screenshot

```
┌─ kdash - Kubernetes Dashboard ─────────────────────────┐
│                                                         │
├─ Pods (14) - ↑↓ to select ─────────────────────────────┤
│ > coredns-7f496c8d7d-7cbfs  kube-system   Running  ... │
│   metrics-server-7b9c9c4b9  kube-system   Running  ... │
├─ Logs: kube-system/coredns-7f496c8d7d-7cbfs ───────────┤
│ [INFO] CoreDNS-1.13.1                                  │
│ [INFO] linux/arm64, go1.25.2                           │
├─────────────────────────────────────────────────────────┤
│ q:quit | r:refresh | l:logs | ↑↓:select | Update: 2s  │
└─────────────────────────────────────────────────────────┘
```

## Requirements

- kubectl installed and configured
- Rust 1.70+ (for building)

## Why kdash?

Lightweight alternative to k9s for quick cluster monitoring. Perfect for:
- Quick cluster health checks
- Monitoring pod restarts
- Watching deployments roll out
- Streaming logs from multiple pods
- Terminal-only environments

## Companion Tools

- **kdbg** - Kubernetes pod debugger with fuzzy matching

## License

MIT
