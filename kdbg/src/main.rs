use anyhow::Result;
use clap::{Parser, Subcommand};

// Import all commands from library
use kdbg::commands::*;

#[derive(Parser)]
#[command(name = "kdbg")]
#[command(about = "Kubernetes Pod Debugger - Fast kubectl wrapper", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List all pods
    List {
        /// Namespace (default: all)
        #[arg(short, long)]
        namespace: Option<String>,

        /// Show more details
        #[arg(short, long)]
        verbose: bool,
    },

    /// Get pod logs
    Logs {
        /// Pod name (or partial match)
        pod: String,

        /// Namespace
        #[arg(short, long)]
        namespace: Option<String>,

        /// Follow logs
        #[arg(short, long)]
        follow: bool,

        /// Number of lines
        #[arg(long, default_value = "100")]
        tail: u32,
    },

    /// Execute command in pod
    Exec {
        /// Pod name (or partial match)
        pod: String,

        /// Namespace
        #[arg(short, long)]
        namespace: Option<String>,

        /// Command to run (default: /bin/sh)
        #[arg(short, long, default_value = "/bin/sh")]
        command: String,
    },

    /// Describe pod
    Describe {
        /// Pod name (or partial match)
        pod: String,

        /// Namespace
        #[arg(short, long)]
        namespace: Option<String>,
    },

    /// Show pod resource usage
    Top {
        /// Namespace
        #[arg(short, long)]
        namespace: Option<String>,
    },

    /// Port forward to pod
    Forward {
        /// Pod name (or partial match)
        pod: String,

        /// Local port
        local_port: u16,

        /// Pod port
        pod_port: u16,

        /// Namespace
        #[arg(short, long)]
        namespace: Option<String>,
    },

    /// Open interactive shell in pod
    Shell {
        /// Pod name (or partial match)
        pod: String,

        /// Namespace
        #[arg(short, long)]
        namespace: Option<String>,
    },

    /// Create debug pod and shell into it
    Debug {
        /// Container image (default: busybox)
        #[arg(short, long, default_value = "busybox")]
        image: String,

        /// Namespace
        #[arg(short, long, default_value = "default")]
        namespace: String,
    },

    /// Restart pod (delete and let it recreate)
    Restart {
        /// Pod name (or partial match)
        pod: String,

        /// Namespace
        #[arg(short, long)]
        namespace: Option<String>,
    },

    /// Show pod events
    Events {
        /// Pod name (or partial match)
        pod: String,

        /// Namespace
        #[arg(short, long)]
        namespace: Option<String>,
    },

    /// Watch pods in real-time
    Watch {
        /// Namespace (default: all)
        #[arg(short, long)]
        namespace: Option<String>,

        /// Refresh interval in seconds
        #[arg(short, long, default_value = "2")]
        interval: u64,
    },

    /// Switch kubectl context
    Ctx {
        /// Context name (omit to list contexts)
        context: Option<String>,
    },

    /// Run a plugin command
    Plugin {
        /// Plugin name
        name: String,

        /// Arguments to pass to plugin
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::List { namespace, verbose } => list_pods(namespace, verbose)?,
        Commands::Logs {
            pod,
            namespace,
            follow,
            tail,
        } => show_logs(&pod, namespace, follow, tail)?,
        Commands::Exec {
            pod,
            namespace,
            command,
        } => exec_pod(&pod, namespace, &command)?,
        Commands::Describe { pod, namespace } => describe_pod(&pod, namespace)?,
        Commands::Top { namespace } => show_top(namespace)?,
        Commands::Forward {
            pod,
            local_port,
            pod_port,
            namespace,
        } => port_forward(&pod, local_port, pod_port, namespace)?,
        Commands::Shell { pod, namespace } => shell_pod(&pod, namespace)?,
        Commands::Debug { image, namespace } => debug_pod(&image, &namespace)?,
        Commands::Restart { pod, namespace } => restart_pod(&pod, namespace)?,
        Commands::Events { pod, namespace } => show_events(&pod, namespace)?,
        Commands::Watch {
            namespace,
            interval,
        } => watch_pods(namespace, interval)?,
        Commands::Ctx { context } => switch_context(context)?,
        Commands::Plugin { name, args } => run_plugin(&name, &args)?,
    }

    Ok(())
}



