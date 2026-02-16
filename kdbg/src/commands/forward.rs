use crate::kubectl::find_pod;
use anyhow::Result;
use colored::*;
use std::process::Command;

pub fn port_forward(
    pod_pattern: &str,
    local_port: u16,
    pod_port: u16,
    namespace: Option<String>,
) -> Result<()> {
    let (pod_name, ns) = find_pod(pod_pattern, namespace)?;

    println!(
        "{} Port forwarding: localhost:{} -> {}:{} (namespace: {})",
        "[INFO]".cyan(),
        local_port,
        pod_name.bold(),
        pod_port,
        ns.bright_black()
    );
    println!("{} Press Ctrl+C to stop", "[INFO]".yellow());
    println!("{}", "-".repeat(100));

    let status = Command::new("kubectl")
        .args([
            "port-forward",
            &pod_name,
            &format!("{}:{}", local_port, pod_port),
            "-n",
            &ns,
        ])
        .status()?;

    if !status.success() {
        anyhow::bail!("Port forwarding failed");
    }

    Ok(())
}
