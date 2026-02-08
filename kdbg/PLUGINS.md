# kdbg Plugins

Extend kdbg with custom commands!

## Usage

```bash
kdbg plugin <name> [args...]
```

## Creating Plugins

1. Create a shell script in `~/.kdbg/plugins/`:
```bash
cat > ~/.kdbg/plugins/my-plugin.sh << 'EOF'
#!/bin/bash
echo "Hello from my plugin!"
echo "Args: $@"
EOF

chmod +x ~/.kdbg/plugins/my-plugin.sh
```

2. Run it:
```bash
kdbg plugin my-plugin arg1 arg2
```

## Example Plugins

### pod-stats
Shows pod count by namespace
```bash
kdbg plugin pod-stats
```

### top-memory
Shows top 5 memory-consuming pods
```bash
kdbg plugin top-memory
```

### restarts
Shows pods with recent restarts
```bash
kdbg plugin restarts
```

## Plugin Environment

Plugins run with:
- `KDBG_PLUGIN=1` environment variable
- Access to `kubectl` and other CLI tools
- All arguments passed through

## Ideas for Plugins

- **backup**: Backup namespace resources
- **health**: Custom health checks
- **deploy**: Custom deployment workflows
- **cleanup**: Remove old resources
- **report**: Generate custom reports
- **alert**: Send notifications
- **migrate**: Move resources between clusters

## Advanced Example

```bash
#!/bin/bash
# ~/.kdbg/plugins/backup.sh
# Backup all resources in a namespace

NAMESPACE=${1:-default}

echo "Backing up namespace: $NAMESPACE"

kubectl get all -n $NAMESPACE -o yaml > backup-$NAMESPACE-$(date +%Y%m%d).yaml

echo "Backup saved to: backup-$NAMESPACE-$(date +%Y%m%d).yaml"
```

Usage:
```bash
kdbg plugin backup production
```

## Tips

- Use `#!/bin/bash` for shell scripts
- Use `#!/usr/bin/env python3` for Python scripts
- Use `#!/usr/bin/env node` for Node.js scripts
- Make sure scripts are executable (`chmod +x`)
- Test with `kdbg plugin <name>` before sharing

## Sharing Plugins

Share your plugins by creating a gist or repo!

Example:
```bash
# Install someone's plugin
curl -o ~/.kdbg/plugins/awesome-plugin.sh https://example.com/plugin.sh
chmod +x ~/.kdbg/plugins/awesome-plugin.sh
```
