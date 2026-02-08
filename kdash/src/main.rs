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
    widgets::{Block, Borders, List, ListItem, Paragraph},
};
use serde_json::Value;
use std::io;
use std::process::Command;
use std::time::{Duration, Instant};

struct App {
    pods: Vec<PodInfo>,
    last_update: Instant,
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
        }
    }

    fn update(&mut self) -> Result<()> {
        self.pods = get_pods()?;
        self.last_update = Instant::now();
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
            let chunks = Layout::default()
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
            f.render_widget(header, chunks[0]);

            // Pod list
            let items: Vec<ListItem> = app
                .pods
                .iter()
                .map(|pod| {
                    let status_color = match pod.status.as_str() {
                        "Running" => Color::Green,
                        "Pending" => Color::Yellow,
                        "Failed" => Color::Red,
                        _ => Color::Gray,
                    };

                    let line = format!(
                        "{:<40} {:<15} {:<10} R:{} Age:{}",
                        pod.name, pod.namespace, pod.status, pod.restarts, pod.age
                    );

                    ListItem::new(Line::from(Span::styled(
                        line,
                        Style::default().fg(status_color),
                    )))
                })
                .collect();

            let pods_list = List::new(items).block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!("Pods ({})", app.pods.len())),
            );
            f.render_widget(pods_list, chunks[1]);

            // Footer
            let elapsed = app.last_update.elapsed().as_secs();
            let footer = Paragraph::new(format!(
                "Press 'q' to quit | 'r' to refresh | Last update: {}s ago",
                elapsed
            ))
            .block(Block::default().borders(Borders::ALL));
            f.render_widget(footer, chunks[2]);
        })?;

        // Handle input
        if event::poll(Duration::from_millis(100))?
            && let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('r') => app.update()?,
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
