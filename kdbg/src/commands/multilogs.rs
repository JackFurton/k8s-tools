use crate::kubectl::get_pods_json;
use anyhow::Result;
use colored::*;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::thread;

pub fn multi_logs(
    pod_pattern: &str,
    namespace: Option<String>,
    follow: bool,
    tail: u32,
) -> Result<()> {
    // Find all matching pods
    let json = get_pods_json(namespace.clone())?;
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

    println!("{} Found {} matching pods:", "[INFO]".cyan(), matches.len());

    let colors = [
        Color::Green,
        Color::Yellow,
        Color::Blue,
        Color::Magenta,
        Color::Cyan,
        Color::BrightGreen,
        Color::BrightYellow,
        Color::BrightBlue,
        Color::BrightMagenta,
        Color::BrightCyan,
    ];

    let mut pod_list = Vec::new();
    for (i, pod) in matches.iter().enumerate() {
        let name = pod["metadata"]["name"]
            .as_str()
            .unwrap_or("unknown")
            .to_string();
        let ns = pod["metadata"]["namespace"]
            .as_str()
            .unwrap_or("default")
            .to_string();
        let color = colors[i % colors.len()];

        println!("  {} {}", "●".color(color), name.color(color));
        pod_list.push((name, ns, color));
    }

    println!("{}", "-".repeat(100));

    if !follow {
        // Non-follow mode: just get logs sequentially
        for (name, ns, color) in &pod_list {
            let tail_str = tail.to_string();
            let output = Command::new("kubectl")
                .args(["logs", name, "-n", ns, "--tail", &tail_str])
                .output()?;

            if output.status.success() {
                let logs = String::from_utf8_lossy(&output.stdout);
                for line in logs.lines() {
                    println!("{} {}", format!("[{}]", name).color(*color), line);
                }
            }
        }
        return Ok(());
    }

    // Follow mode: spawn thread per pod
    let (tx, rx) = mpsc::channel();

    for (name, ns, color) in pod_list {
        let tx = tx.clone();
        thread::spawn(move || {
            let tail_str = tail.to_string();
            let mut child = Command::new("kubectl")
                .args(["logs", &name, "-n", &ns, "--tail", &tail_str, "-f"])
                .stdout(Stdio::piped())
                .stderr(Stdio::null())
                .spawn()
                .ok()?;

            if let Some(stdout) = child.stdout.take() {
                let reader = BufReader::new(stdout);
                for line in reader.lines() {
                    if let Ok(line) = line {
                        let _ = tx.send((name.clone(), line, color));
                    }
                }
            }

            let _ = child.wait();
            Some(())
        });
    }

    drop(tx);

    // Print logs as they come in
    for (pod_name, line, color) in rx {
        println!("{} {}", format!("[{}]", pod_name).color(color), line);
    }

    Ok(())
}
