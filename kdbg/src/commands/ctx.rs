use anyhow::Result;
use colored::*;
use std::process::Command;

pub fn switch_context(context: Option<String>) -> Result<()> {
    if let Some(ctx) = context {
        println!("{} Switching to context: {}", "[INFO]".cyan(), ctx.bold());

        let status = Command::new("kubectl")
            .args(["config", "use-context", &ctx])
            .status()?;

        if !status.success() {
            anyhow::bail!("Failed to switch context");
        }

        println!("{} Context switched successfully", "[SUCCESS]".green());
    } else {
        println!("{} Available contexts:", "[INFO]".cyan());
        println!("{}", "-".repeat(100));

        let output = Command::new("kubectl")
            .args(["config", "get-contexts", "-o", "name"])
            .output()?;

        if !output.status.success() {
            anyhow::bail!("Failed to get contexts");
        }

        let contexts = String::from_utf8_lossy(&output.stdout);

        let current_output = Command::new("kubectl")
            .args(["config", "current-context"])
            .output()?;

        let current = String::from_utf8_lossy(&current_output.stdout)
            .trim()
            .to_string();

        for ctx in contexts.lines() {
            if ctx == current {
                println!("  {} {}", "●".green(), ctx.green().bold());
            } else {
                println!("  ○ {}", ctx);
            }
        }

        println!();
        println!("{} Use 'kdbg ctx <name>' to switch", "[TIP]".yellow());
    }

    Ok(())
}
