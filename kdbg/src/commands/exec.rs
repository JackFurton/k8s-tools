use crate::kubectl::find_pod;
use anyhow::Result;
use colored::*;
use std::process::Command;

pub fn exec_pod(pod_pattern: &str, namespace: Option<String>, command: &str) -> Result<()> {
    let (pod_name, ns) = find_pod(pod_pattern, namespace)?;

    println!(
        "{} Executing in pod: {} (namespace: {})",
        "[INFO]".cyan(),
        pod_name.bold(),
        ns.bright_black()
    );
    println!("{} Command: {}", "[INFO]".cyan(), command.yellow());
    println!("{}", "-".repeat(100));

    let status = Command::new("kubectl")
        .args(["exec", "-it", &pod_name, "-n", &ns, "--", command])
        .status()?;

    if !status.success() {
        anyhow::bail!("Failed to exec into pod");
    }

    Ok(())
}
