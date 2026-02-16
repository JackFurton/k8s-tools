use crate::kubectl::get_pods_json;
use crate::utils::calculate_age;
use anyhow::Result;
use colored::*;

pub fn list_pods(namespace: Option<String>, verbose: bool) -> Result<()> {
    let json = get_pods_json(namespace)?;

    let empty_vec = vec![];
    let pods = json["items"].as_array().unwrap_or(&empty_vec);

    println!("{}", "Pods:".cyan().bold());
    println!("{}", "-".repeat(100));

    if verbose {
        println!(
            "{:<40} {:<15} {:<10} {:<15} {:<20}",
            "NAME", "NAMESPACE", "STATUS", "RESTARTS", "AGE"
        );
        println!("{}", "-".repeat(100));
    } else {
        println!("{:<40} {:<15} {:<10}", "NAME", "NAMESPACE", "STATUS");
        println!("{}", "-".repeat(100));
    }

    for pod in pods {
        let name = pod["metadata"]["name"].as_str().unwrap_or("unknown");
        let ns = pod["metadata"]["namespace"].as_str().unwrap_or("default");
        let phase = pod["status"]["phase"].as_str().unwrap_or("Unknown");

        let status_colored = match phase {
            "Running" => phase.green(),
            "Pending" => phase.yellow(),
            "Failed" => phase.red(),
            "Succeeded" => phase.blue(),
            _ => phase.normal(),
        };

        if verbose {
            let restarts = pod["status"]["containerStatuses"]
                .as_array()
                .and_then(|cs| cs.first())
                .and_then(|c| c["restartCount"].as_u64())
                .unwrap_or(0);

            let age = pod["metadata"]["creationTimestamp"]
                .as_str()
                .map(calculate_age)
                .unwrap_or("unknown".to_string());

            println!(
                "{:<40} {:<15} {:<10} {:<15} {:<20}",
                name.cyan(),
                ns.bright_black(),
                status_colored,
                restarts,
                age
            );
        } else {
            println!(
                "{:<40} {:<15} {:<10}",
                name.cyan(),
                ns.bright_black(),
                status_colored
            );
        }
    }

    println!("\nTotal: {} pods", pods.len());

    Ok(())
}
