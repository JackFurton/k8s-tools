use anyhow::Result;
use colored::*;
use std::process::{Command, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn debug_pod(image: &str, namespace: &str) -> Result<()> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let pod_name = format!("debug-{}", timestamp);

    println!(
        "{} Creating debug pod: {} (image: {}, namespace: {})",
        "[INFO]".cyan(),
        pod_name.bold(),
        image.yellow(),
        namespace.bright_black()
    );
    println!(
        "{} Pod will be deleted when you exit the shell",
        "[INFO]".yellow()
    );
    println!("{}", "-".repeat(100));

    let output = Command::new("kubectl")
        .args([
            "run",
            &pod_name,
            "--image",
            image,
            "-n",
            namespace,
            "--restart=Never",
            "--rm",
            "-it",
            "--",
            "/bin/sh",
        ])
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;

    if !output.success() {
        anyhow::bail!("Failed to create debug pod");
    }

    Ok(())
}
