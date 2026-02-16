use anyhow::Result;
use colored::*;
use serde_json::Value;
use std::process::Command;

/// Find a pod by pattern (fuzzy matching)
/// Returns (pod_name, namespace)
pub fn find_pod(pod_pattern: &str, namespace: Option<String>) -> Result<(String, String)> {
    let mut args = vec!["get", "pods"];

    let ns_str;
    if let Some(ns) = &namespace {
        ns_str = ns.clone();
        args.extend(&["-n", &ns_str]);
    } else {
        args.push("--all-namespaces");
    }

    args.extend(&["-o", "json"]);

    let output = Command::new("kubectl").args(&args).output()?;

    let json: Value = serde_json::from_slice(&output.stdout)?;
    let empty_vec = vec![];
    let pods = json["items"].as_array().unwrap_or(&empty_vec);

    let matches: Vec<_> = pods
        .iter()
        .filter(|pod| {
            let name = pod["metadata"]["name"].as_str().unwrap_or("");
            name.contains(pod_pattern)
        })
        .collect();

    if matches.is_empty() {
        anyhow::bail!("No pods found matching '{}'", pod_pattern);
    }

    if matches.len() > 1 {
        println!("{} Multiple pods found:", "[INFO]".yellow());
        for pod in &matches {
            let name = pod["metadata"]["name"].as_str().unwrap_or("unknown");
            let ns = pod["metadata"]["namespace"].as_str().unwrap_or("default");
            println!("  - {} (namespace: {})", name.cyan(), ns.bright_black());
        }
        anyhow::bail!("Please be more specific");
    }

    let pod = matches[0];
    let name = pod["metadata"]["name"]
        .as_str()
        .unwrap_or("unknown")
        .to_string();
    let ns = pod["metadata"]["namespace"]
        .as_str()
        .unwrap_or("default")
        .to_string();

    Ok((name, ns))
}

/// Get all pods as JSON
pub fn get_pods_json(namespace: Option<String>) -> Result<Value> {
    let mut args = vec!["get", "pods"];

    let ns_str;
    if let Some(ns) = &namespace {
        ns_str = ns.clone();
        args.extend(&["-n", &ns_str]);
    } else {
        args.push("--all-namespaces");
    }

    args.extend(&["-o", "json"]);

    let output = Command::new("kubectl").args(&args).output()?;
    let json: Value = serde_json::from_slice(&output.stdout)?;
    Ok(json)
}

/// Execute kubectl command and return output
pub fn kubectl_exec(args: &[&str]) -> Result<std::process::Output> {
    Ok(Command::new("kubectl").args(args).output()?)
}

/// Execute kubectl command interactively (inherits stdio)
pub fn kubectl_interactive(args: &[&str]) -> Result<()> {
    let status = Command::new("kubectl")
        .args(args)
        .status()?;

    if !status.success() {
        anyhow::bail!("kubectl command failed");
    }

    Ok(())
}
