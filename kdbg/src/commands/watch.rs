use anyhow::Result;
use colored::*;
use serde_json::Value;
use std::io::{self, Write};
use std::process::Command;
use std::thread;
use std::time::Duration;
use crate::utils::calculate_age;

pub fn watch_pods(namespace: Option<String>, interval: u64) -> Result<()> {
    println!(
        "{} Watching pods (refresh every {}s, press Ctrl+C to stop)...",
        "[INFO]".cyan(),
        interval
    );
    println!();

    loop {
        print!("\x1B[2J\x1B[1;1H");
        io::stdout().flush()?;

        println!("{}", "=".repeat(100).bright_black());
        println!(
            "{} {} {}",
            "kdbg watch".cyan().bold(),
            "-".bright_black(),
            chrono::Local::now()
                .format("%Y-%m-%d %H:%M:%S")
                .to_string()
                .bright_black()
        );
        println!("{}", "=".repeat(100).bright_black());
        println!();

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

        if output.status.success() {
            let json: Value = serde_json::from_slice(&output.stdout)?;
            let empty_vec = vec![];
            let pods = json["items"].as_array().unwrap_or(&empty_vec);

            let mut running = 0;
            let mut pending = 0;
            let mut failed = 0;
            let mut other = 0;

            for pod in pods {
                match pod["status"]["phase"].as_str().unwrap_or("Unknown") {
                    "Running" => running += 1,
                    "Pending" => pending += 1,
                    "Failed" => failed += 1,
                    _ => other += 1,
                }
            }

            println!(
                "Total: {} | {}: {} | {}: {} | {}: {} | {}: {}",
                pods.len().to_string().bold(),
                "Running".green(),
                running,
                "Pending".yellow(),
                pending,
                "Failed".red(),
                failed,
                "Other".bright_black(),
                other
            );
            println!();

            println!(
                "{:<50} {:<20} {:<15} {:<10} {}",
                "NAME".bold(),
                "NAMESPACE".bold(),
                "STATUS".bold(),
                "RESTARTS".bold(),
                "AGE".bold()
            );
            println!("{}", "-".repeat(100).bright_black());

            for pod in pods {
                let name = pod["metadata"]["name"].as_str().unwrap_or("unknown");
                let ns = pod["metadata"]["namespace"].as_str().unwrap_or("default");
                let status = pod["status"]["phase"].as_str().unwrap_or("Unknown");
                let restarts = pod["status"]["containerStatuses"]
                    .as_array()
                    .and_then(|arr| arr.first())
                    .and_then(|c| c["restartCount"].as_u64())
                    .unwrap_or(0);
                let created = pod["metadata"]["creationTimestamp"].as_str().unwrap_or("");
                let age = calculate_age(created);

                let status_colored = match status {
                    "Running" => status.green(),
                    "Pending" => status.yellow(),
                    "Failed" => status.red(),
                    _ => status.bright_black(),
                };

                println!(
                    "{:<50} {:<20} {:<15} {:<10} {}",
                    name, ns, status_colored, restarts, age
                );
            }
        } else {
            println!("{} Failed to get pods", "[ERROR]".red());
        }

        thread::sleep(Duration::from_secs(interval));
    }
}
