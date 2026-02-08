# kdash - Kubernetes Dashboard

Real-time TUI dashboard for Kubernetes clusters.

## Features

- Live pod status monitoring
- Auto-refresh every 5 seconds
- Color-coded pod states (Running/Pending/Failed)
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

## Screenshot

```
┌─ kdash - Kubernetes Dashboard ─────────────────────────┐
│                                                         │
├─ Pods (14) ────────────────────────────────────────────┤
│ coredns-7f496c8d7d-7cbfs    kube-system   Running  ... │
│ metrics-server-7b9c9c4b9c   kube-system   Running  ... │
│ test-shell                  default       Running  ... │
├─────────────────────────────────────────────────────────┤
│ Press 'q' to quit | 'r' to refresh | Last update: 2s   │
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
- Terminal-only environments

## Companion Tools

- **kdbg** - Kubernetes pod debugger with fuzzy matching

## License

MIT
