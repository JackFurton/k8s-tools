use crate::kubectl::find_pod;
use anyhow::Result;
use colored::*;
use std::process::Command;

pub fn show_logs(
    pod_pattern: &str,
    namespace: Option<String>,
    follow: bool,
    tail: u32,
) -> Result<()> {
    let (pod_name, ns) = find_pod(pod_pattern, namespace)?;

    println!(
        "{} Logs for pod: {} (namespace: {})",
        "[INFO]".cyan(),
        pod_name.bold(),
        ns.bright_black()
    );
    println!("{}", "-".repeat(100));

    let tail_str = tail.to_string();
    let mut args = vec!["logs", &pod_name, "-n", &ns, "--tail", &tail_str];

    if follow {
        args.push("-f");
    }

    let status = Command::new("kubectl").args(&args).status()?;

    if !status.success() {
        anyhow::bail!("Failed to get logs");
    }

    Ok(())
}
