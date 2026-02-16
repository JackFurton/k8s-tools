use anyhow::Result;
use colored::*;
use std::process::Command;
use crate::kubectl::find_pod;

pub fn describe_pod(pod_pattern: &str, namespace: Option<String>) -> Result<()> {
    let (pod_name, ns) = find_pod(pod_pattern, namespace)?;

    println!(
        "{} Describing pod: {} (namespace: {})",
        "[INFO]".cyan(),
        pod_name.bold(),
        ns.bright_black()
    );
    println!("{}", "-".repeat(100));

    let status = Command::new("kubectl")
        .args(["describe", "pod", &pod_name, "-n", &ns])
        .status()?;

    if !status.success() {
        anyhow::bail!("Failed to describe pod");
    }

    Ok(())
}
