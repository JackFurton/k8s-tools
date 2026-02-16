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
        #[arg(short, long)]
        namespace: Option<String>,
        #[arg(short, long)]
        verbose: bool,
    },

    /// Get pod logs
    Logs {
        pod: String,
        #[arg(short, long)]
        namespace: Option<String>,
        #[arg(short, long)]
        follow: bool,
        #[arg(long, default_value = "100")]
        tail: u32,
    },

    /// Get logs from multiple pods matching pattern
    MultiLogs {
        pod: String,
        #[arg(short, long)]
        namespace: Option<String>,
        #[arg(short, long)]
        follow: bool,
        #[arg(long, default_value = "100")]
        tail: u32,
    },

    /// Execute command in pod
    Exec {
        pod: String,
        #[arg(short, long)]
        namespace: Option<String>,
        #[arg(short, long, default_value = "/bin/sh")]
        command: String,
    },

    /// Describe pod
    Describe {
        pod: String,
        #[arg(short, long)]
        namespace: Option<String>,
    },

    /// Show pod resource usage
    Top {
        #[arg(short, long)]
        namespace: Option<String>,
    },

    /// Port forward to pod
    Forward {
        pod: String,
        local_port: u16,
        pod_port: u16,
        #[arg(short, long)]
        namespace: Option<String>,
    },

    /// Open interactive shell in pod
    Shell {
        pod: String,
        #[arg(short, long)]
        namespace: Option<String>,
    },

    /// Create debug pod and shell into it
    Debug {
        #[arg(short, long, default_value = "busybox")]
        image: String,
        #[arg(short, long, default_value = "default")]
        namespace: String,
    },

    /// Restart pod (delete and let it recreate)
    Restart {
        pod: String,
        #[arg(short, long)]
        namespace: Option<String>,
    },

    /// Show pod events
    Events {
        pod: String,
        #[arg(short, long)]
        namespace: Option<String>,
    },

    /// Watch pods in real-time
    Watch {
        #[arg(short, long)]
        namespace: Option<String>,
        #[arg(short, long, default_value = "2")]
        interval: u64,
    },

    /// Switch kubectl context
    Ctx { context: Option<String> },

    /// Run a plugin command
    Plugin {
        name: String,
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
        Commands::MultiLogs {
            pod,
            namespace,
            follow,
            tail,
        } => multi_logs(&pod, namespace, follow, tail)?,
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
