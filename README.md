# k8s-tools

[![CI](https://github.com/JackFurton/k8s-tools/workflows/CI/badge.svg)](https://github.com/JackFurton/k8s-tools/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)

Rust-based Kubernetes tools for debugging and monitoring.

## Tools

### kdbg - Kubernetes Pod Debugger
Fast kubectl wrapper with fuzzy matching and better UX.

**12 Commands:**
- `list` - List all pods
- `logs` - Get pod logs
- `exec` - Execute commands
- `describe` - Describe pods
- `top` - Resource usage
- `forward` - Port forwarding
- `shell` - Interactive shell
- `debug` - Create debug pods
- `restart` - Restart pods
- `events` - Show pod events
- `watch` - Live pod monitoring
- `ctx` - Switch contexts

**Example:**
```bash
kdbg list
kdbg shell nginx    # Fuzzy match - finds nginx-deployment-7d4f8c9b5-xk2lp
kdbg logs my-app -f
kdbg debug --image ubuntu
kdbg watch          # Live-updating pod list
kdbg ctx            # List/switch contexts
```

### kdash - Kubernetes Dashboard
Real-time TUI dashboard for cluster monitoring.

**Features:**
- Live pod status
- Auto-refresh every 5s
- Color-coded states
- Minimal resource usage

**Example:**
```bash
kdash    # Opens TUI dashboard
```

## Installation

### Quick install (recommended):
```bash
git clone https://github.com/JackFurton/k8s-tools.git
cd k8s-tools
./install.sh
```

### From source:
```bash
git clone https://github.com/JackFurton/k8s-tools.git
cd k8s-tools
cargo build --release

# Install both tools
sudo cp target/release/kdbg /usr/local/bin/
sudo cp target/release/kdash /usr/local/bin/
```

### Individual tools:
```bash
# Just kdbg
cargo build --release -p kdbg

# Just kdash
cargo build --release -p kdash
```

## Requirements

- kubectl installed and configured
- Rust 1.70+ (for building)
- Access to a Kubernetes cluster

## Why these tools?

**kdbg** makes kubectl faster:
- No more typing full pod names
- Shorter commands
- Better output formatting
- Quick debugging workflows

**kdash** gives you instant cluster visibility:
- See all pods at a glance
- Monitor restarts and failures
- Lightweight alternative to k9s

## Workspace Structure

```
k8s-tools/
├── Cargo.toml       # Workspace config
├── kdbg/            # Pod debugger
│   ├── src/
│   └── README.md
├── kdash/           # TUI dashboard
│   ├── src/
│   └── README.md
└── README.md        # This file
```

## Development

```bash
# Build all tools
cargo build --release

# Build specific tool
cargo build --release -p kdbg

# Run tests
cargo test

# Format code
cargo fmt
```

## Related Projects

- **netkit** - AWS/network analysis toolkit (separate repo)
- **mdcat** - Terminal markdown renderer (separate repo)

## License

MIT
