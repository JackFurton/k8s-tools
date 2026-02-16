use crate::kubectl::find_pod;
use anyhow::Result;
use colored::*;
use std::process::{Command, Stdio};

pub fn shell_pod(pod_pattern: &str, namespace: Option<String>) -> Result<()> {
    let (pod_name, ns) = find_pod(pod_pattern, namespace)?;

    println!(
        "{} Opening shell in pod: {} (namespace: {})",
        "[INFO]".cyan(),
        pod_name.bold(),
        ns.bright_black()
    );
    println!("{}", "-".repeat(100));

    let shells = ["/bin/bash", "/bin/sh"];

    for (i, shell) in shells.iter().enumerate() {
        let mut cmd = Command::new("kubectl");
        cmd.args(["exec", "-it", &pod_name, "-n", &ns, "--", shell]);

        cmd.stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::null());

        let status = cmd.status()?;

        if status.success() {
            return Ok(());
        }

        if i == shells.len() - 1 {
            let mut cmd = Command::new("kubectl");
            cmd.args(["exec", "-it", &pod_name, "-n", &ns, "--", shell]);
            cmd.stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit());

            let status = cmd.status()?;
            if status.success() {
                return Ok(());
            }
        }
    }

    anyhow::bail!("Failed to open shell (tried bash and sh)")
}
