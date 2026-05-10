use std::collections::HashMap;
use std::time::{Duration, Instant};
use anyhow::Result;
use chrono::{DateTime, Utc};
use crossterm::event::{self, Event, KeyCode};
use ratatui::{backend::CrosstermBackend, layout::{Constraint, Direction, Layout}, style::{Color, Modifier, Style}, text::{Line, Span}, widgets::{Block, Borders, List, ListItem, ListState, Paragraph}, Frame, Terminal};
use rusqlite::{params, Connection};
use sysinfo::{Pid, ProcessExt, System, SystemExt};

mod db;
mod monitor;

use db::HistoryDB;
use monitor::{PythonProcess, ProcessScanner};

struct App {
    processes: Vec<PythonProcess>,          // current live processes
    selected_index: usize,                 // which process is selected in left pane
    history_entries: Vec<db::HistoryEntry>, // history for the selected script path
    history_state: ListState,               // scroll state for history list
    db: HistoryDB,
    scanner: ProcessScanner,
    last_scan: Instant,
}

impl App {
    fn new() -> Result<Self> {
        let db = HistoryDB::new()?;
        let scanner = ProcessScanner::new();
        Ok(Self {
            processes: Vec::new(),
            selected_index: 0,
            history_entries: Vec::new(),
            history_state: ListState::default(),
            db,
            scanner,
            last_scan: Instant::now(),
        })
    }

    fn refresh(&mut self) -> Result<()> {
        // Scan for Python processes
        self.processes = self.scanner.scan();
        self.update_history_for_selected();
        Ok(())
    }

    fn update_history_for_selected(&mut self) {
        if let Some(proc) = self.processes.get(self.selected_index) {
            self.history_entries = self.db.get_history_by_path(&proc.script_path);
        } else {
            self.history_entries.clear();
        }
        // Reset history list state
        self.history_state.select(Some(0));
    }

    fn next_process(&mut self) {
        if !self.processes.is_empty() {
            self.selected_index = (self.selected_index + 1) % self.processes.len();
            self.update_history_for_selected();
        }
    }

    fn prev_process(&mut self) {
        if !self.processes.is_empty() {
            self.selected_index = if self.selected_index == 0 {
                self.processes.len() - 1
            } else {
                self.selected_index - 1
            };
            self.update_history_for_selected();
        }
    }

    fn scroll_history_down(&mut self) {
        let i = self.history_state.selected().unwrap_or(0);
        if !self.history_entries.is_empty() {
            let next = (i + 1) % self.history_entries.len();
            self.history_state.select(Some(next));
        }
    }

    fn scroll_history_up(&mut self) {
        let i = self.history_state.selected().unwrap_or(0);
        if !self.history_entries.is_empty() {
            let prev = if i == 0 { self.history_entries.len() - 1 } else { i - 1 };
            self.history_state.select(Some(prev));
        }
    }

    // Called periodically to record new processes and update end times for finished ones
    fn record_history(&mut self) -> Result<()> {
        let current_pids: Vec<Pid> = self.processes.iter().map(|p| p.pid).collect();
        self.db.update_finished_processes(&current_pids)?;
        // Insert new processes as started (if not already in DB as running)
        for proc in &self.processes {
            self.db.record_start(&proc.script_path, proc.pid.as_u32())?;
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    // Setup terminal
    let stdout = std::io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    ratatui::crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(std::io::stderr(), crossterm::terminal::EnterAlternateScreen)?;

    let mut app = App::new()?;
    let mut ticker = Instant::now();

    loop {
        terminal.draw(|f| ui::draw(f, &mut app))?;

        // Handle input
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Down => app.next_process(),
                    KeyCode::Up => app.prev_process(),
                    KeyCode::Right => app.scroll_history_down(),
                    KeyCode::Left => app.scroll_history_up(),
                    _ => {}
                }
            }
        }

        // Periodic refresh (every 2 seconds)
        if ticker.elapsed() >= Duration::from_secs(2) {
            app.refresh()?;
            app.record_history()?;
            ticker = Instant::now();
        }
    }

    // Cleanup
    ratatui::crossterm::terminal::disable_raw_mode()?;
    crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen)?;
    Ok(())
}

/// UI module (separate for clarity)
mod ui {
    use super::*;

    pub fn draw(f: &mut Frame<CrosstermBackend<std::io::Stdout>>, app: &mut App) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
            .split(f.size());

        // Left pane: current Python processes
        let process_items: Vec<ListItem> = app.processes.iter().map(|p| {
            let cpu_style = if p.cpu_percent > 50.0 { Color::Red } else { Color::Green };
            let line = Line::from(vec![
                Span::raw(format!("[{}] ", p.pid.as_u32())),
                Span::styled(format!("{:.0}% ", p.cpu_percent), Style::default().fg(cpu_style)),
                Span::raw(format!("{:.0}MB ", p.memory_mb)),
                Span::raw(&p.script_path),
            ]);
            ListItem::new(line)
        }).collect();

        let process_list = List::new(process_items)
            .block(Block::default().borders(Borders::ALL).title(" Live Python Processes (↑/↓) "))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD).bg(Color::DarkGray))
            .highlight_symbol("> ");
        let mut state = ListState::default();
        state.select(Some(app.selected_index));
        f.render_stateful_widget(process_list, chunks[0], &mut state);

        // Right pane: history for selected script
        let title = if let Some(proc) = app.processes.get(app.selected_index) {
            format!(" History for {}", proc.script_path)
        } else {
            " No process selected ".to_string()
        };
        let history_items: Vec<ListItem> = app.history_entries.iter().map(|h| {
            let duration = h.end_time.map(|et| {
                let diff = et.signed_duration_since(h.start_time);
                format!("{}s", diff.num_seconds())
            }).unwrap_or_else(|| "running".to_string());
            let mem_str = h.peak_memory_kb.map(|k| format!("{} MB", k / 1024)).unwrap_or_else(|| "? MB".to_string());
            let line = Line::from(vec![
                Span::raw(format!("{} ", h.start_time.format("%H:%M:%S"))),
                Span::raw(format!("({}) ", duration)),
                Span::raw(mem_str),
            ]);
            ListItem::new(line)
        }).collect();

        let history_list = List::new(history_items)
            .block(Block::default().borders(Borders::ALL).title(title))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD));
        f.render_stateful_widget(history_list, chunks[1], &mut app.history_state);
    }
}

/// Database handling
mod db {
    use super::*;
    use rusqlite::Connection;

    pub struct HistoryDB {
        conn: Connection,
    }

    #[derive(Debug, Clone)]
    pub struct HistoryEntry {
        pub start_time: DateTime<Utc>,
        pub end_time: Option<DateTime<Utc>>,
        pub peak_memory_kb: Option<u64>,
        pub cpu_time_user_sec: Option<f64>,
        pub cpu_time_system_sec: Option<f64>,
        pub exit_code: Option<i32>,
    }

    impl HistoryDB {
        pub fn new() -> Result<Self> {
            let conn = Connection::open("python_monitor.db")?;
            conn.execute(
                "CREATE TABLE IF NOT EXISTS python_runs (
                    id INTEGER PRIMARY KEY,
                    script_path TEXT NOT NULL,
                    start_time TEXT NOT NULL,
                    end_time TEXT,
                    pid INTEGER,
                    peak_memory_kb INTEGER,
                    cpu_time_user_sec REAL,
                    cpu_time_system_sec REAL,
                    exit_code INTEGER
                )",
                [],
            )?;
            Ok(Self { conn })
        }

        pub fn record_start(&self, script_path: &str, pid: u32) -> Result<()> {
            let now = Utc::now().to_rfc3339();
            // Avoid duplicates: check if this pid is already running (not yet ended)
            let mut stmt = self.conn.prepare("SELECT id FROM python_runs WHERE pid = ? AND end_time IS NULL")?;
            let existing: Option<i64> = stmt.query_row([pid], |row| row.get(0)).ok();
            if existing.is_none() {
                self.conn.execute(
                    "INSERT INTO python_runs (script_path, start_time, pid) VALUES (?, ?, ?)",
                    params![script_path, now, pid],
                )?;
            }
            Ok(())
        }

        pub fn update_finished_processes(&self, still_running_pids: &[Pid]) -> Result<()> {
            // Find all recorded runs that have no end_time and whose pid is NOT in still_running_pids
            let still_pids: Vec<u32> = still_running_pids.iter().map(|p| p.as_u32()).collect();
            let mut stmt = self.conn.prepare("SELECT id, pid FROM python_runs WHERE end_time IS NULL")?;
            let rows = stmt.query_map([], |row| {
                let id: i64 = row.get(0)?;
                let pid: u32 = row.get(1)?;
                Ok((id, pid))
            })?;

            let now = Utc::now().to_rfc3339();
            for row in rows {
                let (id, pid) = row?;
                if !still_pids.contains(&pid) {
                    // Process is gone – attempt to get final resources (using sysinfo again or just leave null)
                    // For simplicity, we just set end_time. In a full implementation you would query /proc stats.
                    self.conn.execute(
                        "UPDATE python_runs SET end_time = ? WHERE id = ?",
                        params![now, id],
            )?;
                }
            }
            Ok(())
        }

        pub fn get_history_by_path(&self, path: &str) -> Vec<HistoryEntry> {
            let mut stmt = self.conn.prepare(
                "SELECT start_time, end_time, peak_memory_kb, cpu_time_user_sec, cpu_time_system_sec, exit_code
                 FROM python_runs WHERE script_path = ? ORDER BY start_time DESC LIMIT 100",
            ).unwrap();
            let rows = stmt.query_map([path], |row| {
                let start_time: String = row.get(0)?;
                let end_time: Option<String> = row.get(1)?;
                let peak_memory_kb: Option<u64> = row.get(2)?;
                let cpu_user: Option<f64> = row.get(3)?;
                let cpu_system: Option<f64> = row.get(4)?;
                let exit_code: Option<i32> = row.get(5)?;
                Ok(HistoryEntry {
                    start_time: DateTime::parse_from_rfc3339(&start_time).unwrap().with_timezone(&Utc),
                    end_time: end_time.map(|e| DateTime::parse_from_rfc3339(&e).unwrap().with_timezone(&Utc)),
                    peak_memory_kb,
                    cpu_time_user_sec: cpu_user,
                    cpu_time_system_sec: cpu_system,
                    exit_code,
                })
            }).unwrap();
            rows.filter_map(|r| r.ok()).collect()
        }
    }
}

/// Process scanning logic
mod monitor {
    use super::*;

    #[derive(Debug)]
    pub struct PythonProcess {
        pub pid: Pid,
        pub script_path: String,
        pub cpu_percent: f32,
        pub memory_mb: u64,
        pub start_time: DateTime<Utc>, // not available from sysinfo, we'll use current time as proxy
    }

    pub struct ProcessScanner {
        system: System,
    }

    impl ProcessScanner {
        pub fn new() -> Self {
            Self { system: System::new_all() }
        }

        pub fn scan(&mut self) -> Vec<PythonProcess> {
            self.system.refresh_all();
            let mut results = Vec::new();

            for (pid, proc) in self.system.processes() {
                let name = proc.name().to_lowercase();
                if name == "python" || name == "python3" || name.contains("python") {
                    if let Some(script_path) = extract_script_path(proc) {
                        results.push(PythonProcess {
                            pid: *pid,
                            script_path,
                            cpu_percent: proc.cpu_usage(),
                            memory_mb: proc.memory() / 1024 / 1024,
                            start_time: Utc::now(), // sysinfo doesn't provide start time; we use now as approximation
                        });
                    }
                }
            }
            results
        }
    }

    fn extract_script_path(proc: &sysinfo::Process) -> Option<String> {
        let cmd = proc.cmd();
        // Typical: python3 /path/to/script.py [args...]
        // Also handle: python3 -m module -> we treat as "module:module"
        for arg in cmd.iter().skip(1) {
            if !arg.starts_with('-') && !arg.starts_with("--") {
                // Heuristic: if it contains a slash or .py, consider it a script path
                if arg.contains('/') || arg.ends_with(".py") {
                    return Some(arg.to_string());
                }
                // Otherwise, it might be a module name, but we'll still record as "module:foo"
                if !arg.chars().all(|c| c.is_ascii_alphabetic() || c == '.' || c == '_') {
                    // unlikely to be a module name; skip
                    continue;
                }
                return Some(format!("module:{}", arg));
            }
        }
        None
    }
}
