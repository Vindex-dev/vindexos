use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::sync::mpsc::{channel, Receiver};
use std::thread;

use crossterm::event::KeyCode;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Gauge, Paragraph},
    Frame,
};

use crate::screen::Action;

pub struct InstallState {
    pub progress: u16,
    pub message: String,
    pub logs: Vec<String>,
    pub finished: bool,
    pub error: Option<String>,
    receiver: Option<Receiver<InstallMessage>>,
}

enum InstallMessage {
    Progress(u16, String),
    Log(String),
    Error(String),
    Done,
}

impl InstallState {
    pub fn new() -> Self {
        Self {
            progress: 0,
            message: String::from("Starting..."),
            logs: Vec::new(),
            finished: false,
            error: None,
            receiver: None,
        }
    }

    pub fn start(&mut self) {
        let (tx, rx) = channel();
        self.receiver = Some(rx);

        thread::spawn(move || {
            let mut child = match Command::new("python")
                .arg("scripts/install.py")
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
            {
                Ok(c) => c,
                Err(e) => {
                    let _ = tx.send(InstallMessage::Error(format!("Failed to start: {}", e)));
                    return;
                }
            };

            if let Some(stdout) = child.stdout.take() {
                let reader = BufReader::new(stdout);
                for line in reader.lines().flatten() {
                    if line.starts_with("PROGRESS:") {
                        let parts: Vec<&str> = line.splitn(3, ':').collect();
                        if parts.len() == 3 {
                            if let Ok(pct) = parts[1].parse::<u16>() {
                                let _ = tx.send(InstallMessage::Progress(pct, parts[2].to_string()));
                            }
                        }
                    } else if line.starts_with("LOG:") {
                        let _ = tx.send(InstallMessage::Log(line[4..].to_string()));
                    } else if line.starts_with("ERROR:") {
                        let _ = tx.send(InstallMessage::Error(line[6..].to_string()));
                    } else {
                        let _ = tx.send(InstallMessage::Log(line));
                    }
                }
            }

            match child.wait() {
                Ok(status) if status.success() => {
                    let _ = tx.send(InstallMessage::Done);
                }
                Ok(_) => {
                    let _ = tx.send(InstallMessage::Error("Installation failed".to_string()));
                }
                Err(e) => {
                    let _ = tx.send(InstallMessage::Error(format!("Process error: {}", e)));
                }
            }
        });
    }

    pub fn update(&mut self) {
        if let Some(ref rx) = self.receiver {
            while let Ok(msg) = rx.try_recv() {
                match msg {
                    InstallMessage::Progress(pct, text) => {
                        self.progress = pct;
                        self.message = text.clone();
                        self.logs.push(format!("[{}%] {}", pct, text));
                        if self.logs.len() > 100 {
                            self.logs.remove(0);
                        }
                    }
                    InstallMessage::Log(text) => {
                        self.logs.push(text);
                        if self.logs.len() > 100 {
                            self.logs.remove(0);
                        }
                    }
                    InstallMessage::Error(err) => {
                        self.error = Some(err.clone());
                        self.logs.push(format!("ERROR: {}", err));
                        self.finished = true;
                    }
                    InstallMessage::Done => {
                        self.finished = true;
                        self.progress = 100;
                    }
                }
            }
        }
    }
}

pub fn draw(frame: &mut Frame<'_>, state: &InstallState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(frame.size());

    let gauge = Gauge::default()
        .block(Block::default().title(" Installing VindexOS ").borders(Borders::ALL))
        .gauge_style(ratatui::style::Style::default().fg(ratatui::style::Color::Green))
        .percent(state.progress);
    frame.render_widget(gauge, chunks[0]);

    let log_height = chunks[1].height as usize;
    let visible_logs: Vec<String> = state.logs.iter()
        .rev()
        .take(log_height.saturating_sub(2))
        .rev()
        .cloned()
        .collect();

    let logs_text = visible_logs.join("\n");
    let logs = Paragraph::new(logs_text)
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(logs, chunks[1]);

    let hint = if state.finished {
        if state.error.is_some() {
            "Installation failed. Press any key to exit."
        } else {
            "Installation complete! Press any key to exit."
        }
    } else {
        &state.message
    };

    let status = Paragraph::new(hint)
        .block(Block::default().borders(Borders::ALL))
        .alignment(ratatui::layout::Alignment::Center);
    frame.render_widget(status, chunks[2]);
}

pub fn handle_input(_key: KeyCode, state: &InstallState) -> Action {
    if state.finished {
        Action::Exit
    } else {
        Action::Stay
    }
}
