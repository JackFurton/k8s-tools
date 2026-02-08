use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Sparkline},
};
use serde_json::Value;
use std::collections::VecDeque;
use std::io;
use std::process::Command;
use std::time::{Duration, Instant};

struct App {
    pods: Vec<PodInfo>,
    last_update: Instant,
    selected_index: usize,
    logs: Vec<String>,
    show_logs: bool,
    show_metrics: bool,
    metrics_history: VecDeque<MetricsSnapshot>,
}

struct MetricsSnapshot {
    timestamp: Instant,
    pod_metrics: Vec<PodMetrics>,
}

struct PodMetrics {
    name: String,
    namespace: String,
    cpu: u64,    // millicores
    memory: u64, // bytes
}

struct PodInfo {
    name: String,
    namespace: String,
    status: String,
    restarts: u64,
    age: String,
}

impl App {
    fn new() -> Self {
        Self {
            pods: Vec::new(),
            last_update: Instant::now(),
            selected_index: 0,
            logs: Vec::new(),
            show_logs: false,
            show_metrics: false,
            metrics_history: VecDeque::with_capacity(60), // Keep 60 data points
        }
    }

    fn update(&mut self) -> Result<()> {
        self.pods = get_pods()?;
        self.last_update = Instant::now();

        // Update metrics if showing
        if self.show_metrics {
            self.update_metrics()?;
        }

        Ok(())
    }

    fn update_metrics(&mut self) -> Result<()> {
        let metrics = get_pod_metrics()?;

        self.metrics_history.push_back(MetricsSnapshot {
            timestamp: Instant::now(),
            pod_metrics: metrics,
        });

        // Keep only last 60 snapshots (5 minutes at 5s intervals)
        if self.metrics_history.len() > 60 {
            self.metrics_history.pop_front();
        }

        Ok(())
    }

    fn toggle_metrics(&mut self) -> Result<()> {
        self.show_metrics = !self.show_metrics;
        if self.show_metrics {
            self.update_metrics()?;
        }
        Ok(())
    }

    fn select_next(&mut self) {
        if !self.pods.is_empty() {
            self.selected_index = (self.selected_index + 1) % self.pods.len();
        }
    }

    fn select_prev(&mut self) {
        if !self.pods.is_empty() {
            if self.selected_index > 0 {
                self.selected_index -= 1;
            } else {
                self.selected_index = self.pods.len() - 1;
            }
        }
    }

    fn toggle_logs(&mut self) -> Result<()> {
        self.show_logs = !self.show_logs;
        if self.show_logs && !self.pods.is_empty() {
            self.fetch_logs()?;
        }
        Ok(())
    }

    fn fetch_logs(&mut self) -> Result<()> {
        if let Some(pod) = self.pods.get(self.selected_index) {
            let output = Command::new("kubectl")
                .args(["logs", &pod.name, "-n", &pod.namespace, "--tail=50"])
                .output()?;

            if output.status.success() {
                self.logs = String::from_utf8_lossy(&output.stdout)
                    .lines()
                    .map(|s| s.to_string())
                    .collect();
            } else {
                self.logs = vec!["Failed to fetch logs".to_string()];
            }
        }
        Ok(())
    }
}

fn get_pods() -> Result<Vec<PodInfo>> {
    let output = Command::new("kubectl")
        .args(["get", "pods", "--all-namespaces", "-o", "json"])
        .output()?;

    if !output.status.success() {
        return Ok(Vec::new());
    }

    let json: Value = serde_json::from_slice(&output.stdout)?;
    let mut pods = Vec::new();

    if let Some(items) = json["items"].as_array() {
        for item in items {
            let name = item["metadata"]["name"]
                .as_str()
                .unwrap_or("unknown")
                .to_string();
            let namespace = item["metadata"]["namespace"]
                .as_str()
                .unwrap_or("default")
                .to_string();
            let status = item["status"]["phase"]
                .as_str()
                .unwrap_or("Unknown")
                .to_string();

            let restarts = item["status"]["containerStatuses"]
                .as_array()
                .and_then(|arr| arr.first())
                .and_then(|c| c["restartCount"].as_u64())
                .unwrap_or(0);

            let created = item["metadata"]["creationTimestamp"].as_str().unwrap_or("");
            let age = calculate_age(created);

            pods.push(PodInfo {
                name,
                namespace,
                status,
                restarts,
                age,
            });
        }
    }

    Ok(pods)
}

fn get_pod_metrics() -> Result<Vec<PodMetrics>> {
    let output = Command::new("kubectl")
        .args(["top", "pods", "--all-namespaces", "--no-headers"])
        .output()?;

    if !output.status.success() {
        return Ok(Vec::new());
    }

    let mut metrics = Vec::new();
    let output_str = String::from_utf8_lossy(&output.stdout);

    for line in output_str.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 4 {
            let namespace = parts[0].to_string();
            let name = parts[1].to_string();

            // Parse CPU (e.g., "1m" -> 1 millicores)
            let cpu = parts[2].trim_end_matches('m').parse::<u64>().unwrap_or(0);

            // Parse memory (e.g., "128Mi" -> bytes)
            let memory_str = parts[3];
            let memory = if memory_str.ends_with("Mi") {
                memory_str
                    .trim_end_matches("Mi")
                    .parse::<u64>()
                    .unwrap_or(0)
                    * 1024
                    * 1024
            } else if memory_str.ends_with("Gi") {
                memory_str
                    .trim_end_matches("Gi")
                    .parse::<u64>()
                    .unwrap_or(0)
                    * 1024
                    * 1024
                    * 1024
            } else {
                0
            };

            metrics.push(PodMetrics {
                name,
                namespace,
                cpu,
                memory,
            });
        }
    }

    Ok(metrics)
}

fn calculate_age(timestamp: &str) -> String {
    use std::time::{SystemTime, UNIX_EPOCH};

    let created = chrono::DateTime::parse_from_rfc3339(timestamp)
        .map(|dt| dt.timestamp())
        .unwrap_or(0);

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let diff = now - created;

    if diff < 60 {
        format!("{}s", diff)
    } else if diff < 3600 {
        format!("{}m", diff / 60)
    } else if diff < 86400 {
        format!("{}h", diff / 3600)
    } else {
        format!("{}d", diff / 86400)
    }
}

fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    app.update()?;

    loop {
        terminal.draw(|f| {
            let main_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Min(0),
                    Constraint::Length(3),
                ])
                .split(f.area());

            // Header
            let header = Paragraph::new(vec![Line::from(vec![
                Span::styled(
                    "kdash",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(" - Kubernetes Dashboard"),
            ])])
            .block(Block::default().borders(Borders::ALL));
            f.render_widget(header, main_chunks[0]);

            // Split middle section if logs are shown
            if app.show_logs {
                let middle_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                    .split(main_chunks[1]);

                // Pod list
                let items: Vec<ListItem> = app
                    .pods
                    .iter()
                    .enumerate()
                    .map(|(i, pod)| {
                        let status_color = match pod.status.as_str() {
                            "Running" => Color::Green,
                            "Pending" => Color::Yellow,
                            "Failed" => Color::Red,
                            _ => Color::Gray,
                        };

                        let line = format!(
                            "{} {:<38} {:<15} {:<10} R:{} Age:{}",
                            if i == app.selected_index { ">" } else { " " },
                            pod.name,
                            pod.namespace,
                            pod.status,
                            pod.restarts,
                            pod.age
                        );

                        let mut style = Style::default().fg(status_color);
                        if i == app.selected_index {
                            style = style.add_modifier(Modifier::BOLD);
                        }

                        ListItem::new(Line::from(Span::styled(line, style)))
                    })
                    .collect();

                let pods_list = List::new(items).block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(format!("Pods ({}) - ↑↓ to select", app.pods.len())),
                );
                f.render_widget(pods_list, middle_chunks[0]);

                // Logs panel
                let log_lines: Vec<Line> =
                    app.logs.iter().map(|l| Line::from(l.as_str())).collect();

                let selected_pod = app
                    .pods
                    .get(app.selected_index)
                    .map(|p| format!("{}/{}", p.namespace, p.name))
                    .unwrap_or_else(|| "None".to_string());

                let logs_widget = Paragraph::new(log_lines).block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(format!("Logs: {}", selected_pod)),
                );
                f.render_widget(logs_widget, middle_chunks[1]);
            } else {
                // Just pod list
                let items: Vec<ListItem> = app
                    .pods
                    .iter()
                    .enumerate()
                    .map(|(i, pod)| {
                        let status_color = match pod.status.as_str() {
                            "Running" => Color::Green,
                            "Pending" => Color::Yellow,
                            "Failed" => Color::Red,
                            _ => Color::Gray,
                        };

                        let line = format!(
                            "{} {:<38} {:<15} {:<10} R:{} Age:{}",
                            if i == app.selected_index { ">" } else { " " },
                            pod.name,
                            pod.namespace,
                            pod.status,
                            pod.restarts,
                            pod.age
                        );

                        let mut style = Style::default().fg(status_color);
                        if i == app.selected_index {
                            style = style.add_modifier(Modifier::BOLD);
                        }

                        ListItem::new(Line::from(Span::styled(line, style)))
                    })
                    .collect();

                let pods_list = List::new(items).block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(format!("Pods ({}) - ↑↓ to select", app.pods.len())),
                );
                f.render_widget(pods_list, main_chunks[1]);
            }

            // Footer
            let elapsed = app.last_update.elapsed().as_secs();
            let mut footer_text = format!(
                "q:quit | r:refresh | l:logs | m:metrics | ↑↓:select | Last update: {}s ago",
                elapsed
            );

            if app.show_metrics && !app.metrics_history.is_empty() {
                let latest = app.metrics_history.back().unwrap();
                let total_cpu: u64 = latest.pod_metrics.iter().map(|m| m.cpu).sum();
                let total_mem: u64 = latest.pod_metrics.iter().map(|m| m.memory).sum();
                let mem_mb = total_mem / 1024 / 1024;
                footer_text = format!("{} | CPU: {}m | MEM: {}Mi", footer_text, total_cpu, mem_mb);
            }

            let footer = Paragraph::new(footer_text).block(Block::default().borders(Borders::ALL));
            f.render_widget(footer, main_chunks[2]);
        })?;

        // Handle input
        if event::poll(Duration::from_millis(100))?
            && let Event::Key(key) = event::read()?
        {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Char('r') => app.update()?,
                KeyCode::Char('l') => app.toggle_logs()?,
                KeyCode::Char('m') => app.toggle_metrics()?,
                KeyCode::Up => app.select_prev(),
                KeyCode::Down => app.select_next(),
                _ => {}
            }
        }

        // Auto-refresh every 5 seconds
        if app.last_update.elapsed() > Duration::from_secs(5) {
            app.update()?;
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
