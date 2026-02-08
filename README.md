# k8s-tools

[![CI](https://github.com/JackFurton/k8s-tools/workflows/CI/badge.svg)](https://github.com/JackFurton/k8s-tools/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)

Rust-based Kubernetes tools for debugging and monitoring.

## Quick Start

```bash
git clone https://github.com/JackFurton/k8s-tools.git
cd k8s-tools
./install.sh
```

## Tools

### 🔧 kdbg - Kubernetes Pod Debugger
Fast kubectl wrapper with fuzzy matching and plugin system.

**13 Commands:**
- `list` - List all pods (with verbose mode)
- `logs` - Get pod logs (with follow and tail)
- `exec` - Execute commands in pods
- `shell` - Interactive shell (auto-detects bash/sh)
- `describe` - Describe pod details
- `top` - Show resource usage
- `forward` - Port forwarding
- `debug` - Create temporary debug pods
- `restart` - Restart pods (delete and recreate)
- `events` - Show pod events
- `watch` - Live-updating pod list
- `ctx` - Switch kubectl contexts
- `plugin` - Run custom plugins 🔌

**Examples:**
```bash
kdbg list                    # List all pods
kdbg shell nginx             # Fuzzy match - finds nginx-deployment-xxx
kdbg logs my-app -f          # Follow logs
kdbg debug --image ubuntu    # Create debug pod
kdbg watch                   # Live pod monitoring
kdbg ctx production          # Switch context
kdbg plugin pod-stats        # Run custom plugin
```

**Plugin System:**
Extend kdbg with custom commands! Drop shell scripts in `~/.kdbg/plugins/`:

```bash
# Create a plugin
cat > ~/.kdbg/plugins/backup.sh << 'EOF'
#!/bin/bash
kubectl get all -o yaml > backup-$(date +%Y%m%d).yaml
echo "Backup saved!"
EOF

chmod +x ~/.kdbg/plugins/backup.sh

# Run it
kdbg plugin backup
```

See [PLUGINS.md](kdbg/PLUGINS.md) for full plugin documentation.

### 📊 kdash - Kubernetes Dashboard
Real-time TUI dashboard for cluster monitoring.

**Features:**
- Live pod status with color-coding
- Interactive pod selection (↑↓ arrow keys)
- Real-time log streaming (press `l`)
- Resource metrics - CPU/memory (press `m`)
- Auto-refresh every 5 seconds
- Minimal resource usage

**Controls:**
- `q` - Quit
- `r` - Manual refresh
- `l` - Toggle logs panel
- `m` - Toggle metrics
- `↑↓` - Select pod

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
cargo build --release
sudo cp target/release/kdbg /usr/local/bin/
sudo cp target/release/kdash /usr/local/bin/
```

### Individual tools:
```bash
cargo build --release -p kdbg
cargo build --release -p kdash
```

## Requirements

- kubectl installed and configured
- Rust 1.70+ (for building)
- Access to a Kubernetes cluster

## Why these tools?

**kdbg makes kubectl faster:**
- No more typing full pod names (fuzzy matching)
- Shorter commands
- Better output formatting
- Extensible with plugins
- Quick debugging workflows

**kdash gives instant cluster visibility:**
- See all pods at a glance
- Monitor restarts and failures
- Stream logs in real-time
- Track resource usage
- Lightweight alternative to k9s

## Development

```bash
# Build all tools
cargo build --release

# Run CI checks locally
./ci-check.sh

# Format code
cargo fmt

# Run clippy
cargo clippy

# Run tests
cargo test
```

## Related Projects

- **netkit** - AWS/network analysis toolkit (separate repo)
- **mdcat** - Terminal markdown renderer (separate repo)

## License

MIT
