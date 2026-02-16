use anyhow::Result;
use colored::*;
use std::process::Command;

pub fn show_top(namespace: Option<String>) -> Result<()> {
    let mut args = vec!["top", "pods"];

    let ns_str;
    if let Some(ns) = &namespace {
        ns_str = ns.clone();
        args.extend(&["-n", &ns_str]);
    } else {
        args.push("--all-namespaces");
    }

    println!("{}", "Pod Resource Usage:".cyan().bold());
    println!("{}", "-".repeat(100));

    let status = Command::new("kubectl").args(&args).status()?;

    if !status.success() {
        eprintln!(
            "{} Failed to get resource usage (metrics-server may not be installed)",
            "[WARN]".yellow()
        );
    }

    Ok(())
}
