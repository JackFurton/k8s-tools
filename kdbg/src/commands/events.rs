use crate::kubectl::find_pod;
use anyhow::Result;
use colored::*;
use std::process::Command;

pub fn show_events(pod_pattern: &str, namespace: Option<String>) -> Result<()> {
    let (pod_name, ns) = find_pod(pod_pattern, namespace)?;

    println!(
        "{} Events for pod: {} (namespace: {})",
        "[INFO]".cyan(),
        pod_name.bold(),
        ns.bright_black()
    );
    println!("{}", "-".repeat(100));

    let status = Command::new("kubectl")
        .args([
            "get",
            "events",
            "-n",
            &ns,
            "--field-selector",
            &format!("involvedObject.name={}", pod_name),
            "--sort-by",
            ".lastTimestamp",
        ])
        .status()?;

    if !status.success() {
        anyhow::bail!("Failed to get events");
    }

    Ok(())
}
