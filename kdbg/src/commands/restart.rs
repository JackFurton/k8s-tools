use crate::kubectl::find_pod;
use anyhow::Result;
use colored::*;
use std::process::Command;

pub fn restart_pod(pod_pattern: &str, namespace: Option<String>) -> Result<()> {
    let (pod_name, ns) = find_pod(pod_pattern, namespace)?;

    println!(
        "{} Restarting pod: {} (namespace: {})",
        "[INFO]".cyan(),
        pod_name.bold(),
        ns.bright_black()
    );
    println!(
        "{} This will delete the pod and let the controller recreate it",
        "[INFO]".yellow()
    );
    println!("{}", "-".repeat(100));

    let status = Command::new("kubectl")
        .args(["delete", "pod", &pod_name, "-n", &ns])
        .status()?;

    if !status.success() {
        anyhow::bail!("Failed to delete pod");
    }

    println!(
        "{} Pod deleted. Waiting for recreation...",
        "[SUCCESS]".green()
    );

    Ok(())
}
