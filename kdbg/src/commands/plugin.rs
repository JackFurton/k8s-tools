use anyhow::Result;
use colored::*;
use std::fs;
use std::process::Command;
use crate::utils::get_plugin_dir;

pub fn run_plugin(name: &str, args: &[String]) -> Result<()> {
    let plugin_dir = get_plugin_dir();
    let plugin_path = plugin_dir.join(format!("{}.sh", name));

    if !plugin_path.exists() {
        println!("{} Plugin '{}' not found", "[ERROR]".red(), name);
        println!();
        println!("Available plugins:");

        if plugin_dir.exists() {
            if let Ok(entries) = fs::read_dir(&plugin_dir) {
                let mut found_any = false;
                for entry in entries.flatten() {
                    if let Some(filename) = entry.file_name().to_str() {
                        if filename.ends_with(".sh") {
                            let plugin_name = filename.trim_end_matches(".sh");
                            println!("  - {}", plugin_name.cyan());
                            found_any = true;
                        }
                    }
                }
                if !found_any {
                    println!("  (none)");
                }
            }
        } else {
            println!("  (none - create plugins in ~/.kdbg/plugins/)");
        }

        println!();
        println!("{} Create a plugin:", "[TIP]".yellow());
        println!("  echo '#!/bin/bash' > ~/.kdbg/plugins/{}.sh", name);
        println!(
            "  echo 'echo \"Hello from {}!\"' >> ~/.kdbg/plugins/{}.sh",
            name, name
        );
        println!("  chmod +x ~/.kdbg/plugins/{}.sh", name);

        anyhow::bail!("Plugin not found");
    }

    println!("{} Running plugin: {}", "[INFO]".cyan(), name.bold());
    println!("{}", "-".repeat(100));

    let status = Command::new(&plugin_path)
        .args(args)
        .env("KDBG_PLUGIN", "1")
        .status()?;

    if !status.success() {
        anyhow::bail!("Plugin exited with error");
    }

    Ok(())
}
