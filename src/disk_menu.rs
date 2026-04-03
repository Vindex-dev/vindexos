use crossterm::event::KeyCode;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use std::process::{Command, Stdio};
use std::sync::OnceLock;

use crate::config::Config;
use crate::screen::{Action, Screen};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DiskMode {
    SelectRoot,
    SelectHome,
}

static DISK_CACHE: OnceLock<DiskList> = OnceLock::new();

#[derive(Debug, Clone)]
pub struct DiskList {
    pub items: Vec<String>,
    pub names: Vec<String>,
}

impl DiskList {
    pub fn get() -> &'static DiskList {
        DISK_CACHE.get_or_init(|| {
            let output = Command::new("/bin/bash")
                .arg("./scripts/disk_list.sh")
                .current_dir(std::env::current_dir().unwrap())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output();

            match output {
                Ok(out) => {
                    if out.status.success() {
                        let text = String::from_utf8_lossy(&out.stdout);
                        let lines: Vec<String> = text.lines().map(|s| s.to_string()).collect();
                        let names: Vec<String> = lines
                            .iter()
                            .filter_map(|l| l.split_whitespace().next().map(String::from))
                            .collect();
                        if !lines.is_empty() {
                            return Self { items: lines, names };
                        }
                    } else {
                        let err = String::from_utf8_lossy(&out.stderr);
                        eprintln!("️ Script error: {}", err);
                    }
                }
                Err(e) => eprintln!("⚠️ Failed to run script: {}", e),
            }
            Self {
                items: vec!["sda 500G".to_string(), "nvme0n1 1T".to_string()],
                names: vec!["sda".to_string(), "nvme0n1".to_string()],
            }
        })
    }
}

pub fn draw(frame: &mut Frame<'_>, config: &Config) {
    let disks = DiskList::get();

    let mode = if config.root_disk.is_some() && config.home_disk.is_none() {
        DiskMode::SelectHome
    } else {
        DiskMode::SelectRoot
    };

    if disks.items.is_empty() {
        frame.render_widget(Paragraph::new("No disks"), frame.size());
        return;
    }

    let list_items: Vec<ListItem> = disks.items.iter().enumerate().map(|(i, line)| {
        let name = disks.names.get(i).map(|s| s.as_str()).unwrap_or("");
        let mut status = String::new();

        if config.root_disk.as_deref() == Some(name) { status.push_str(" [ROOT]"); }
        if config.home_disk.as_deref() == Some(name) { status.push_str(" [HOME]"); }

        ListItem::new(format!("{}{}{}", marker(config.disk_cursor == i), line, status))
    }).collect();

    let list = List::new(list_items)
        .block(Block::default().title(match mode {
            DiskMode::SelectRoot => " Select ROOT (/) ",
            DiskMode::SelectHome => " Select HOME (/home) ",
        }).borders(Borders::ALL));

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(frame.size());

    frame.render_widget(list, chunks[0]);

    let hint = match mode {
        DiskMode::SelectRoot => "↑/↓: Move | Enter: Set ROOT | Tab: Switch to HOME | Esc: Back",
        DiskMode::SelectHome => "↑/↓: Move | Enter: Set HOME | Tab: Switch to ROOT | Esc: Back",
    };

    frame.render_widget(Paragraph::new(hint).alignment(Alignment::Center), chunks[1]);
}

pub fn handle_input(key: KeyCode, config: &mut Config) -> Action {
    let disks = DiskList::get();
    let max_idx = disks.names.len().saturating_sub(1);

    let current_mode = if config.root_disk.is_some() && config.home_disk.is_none() {
        DiskMode::SelectHome
    } else {
        DiskMode::SelectRoot
    };

    match key {
        KeyCode::Esc => Action::GoTo(Screen::MainMenu),

        KeyCode::Tab => {
            if config.home_disk.is_some() {
                config.home_disk = None;
            } else if config.root_disk.is_some() {
            } else {
            }
            Action::Stay
        }

        KeyCode::Down => {
            if config.disk_cursor < max_idx {
                config.disk_cursor += 1;
            }
            Action::Stay
        }

        KeyCode::Up => {
            if config.disk_cursor > 0 {
                config.disk_cursor -= 1;
            }
            Action::Stay
        }

        KeyCode::Enter => {
            if let Some(name) = disks.names.get(config.disk_cursor) {
                if current_mode == DiskMode::SelectRoot {
                    config.root_disk = Some(name.clone());
                } else {
                    config.home_disk = Some(name.clone());
                }
            }
            Action::Stay
        }

        _ => Action::Stay,
    }
}

fn marker(is_active: bool) -> &'static str {
    if is_active { "> " } else { "  " }
}
