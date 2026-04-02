use crossterm::event::KeyCode;
use ratatui::{
    layout::Alignment,
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use std::process::{Command, Stdio};

use crate::config::Config;
use crate::screen::{Action, Screen};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DiskMode {
    SelectRoot,
    SelectHome,
}

pub struct DiskData {
    pub raw_lines: Vec<String>,
    pub names: Vec<String>,
}

impl DiskData {
    pub fn load() -> Self {
        let output = Command::new("./scripts/disk_list.sh")
            .stdout(Stdio::piped())
            .output();

        match output {
            Ok(out) => {
                let text = String::from_utf8_lossy(&out.stdout);
                let lines: Vec<String> = text.lines().map(|s| s.to_string()).collect();

                let names: Vec<String> = lines
                    .iter()
                    .filter_map(|line| line.split_whitespace().next().map(String::from))
                    .collect();

                Self { raw_lines: lines, names }
            }
            Err(e) => {
                eprintln!("Failed to run disk script: {}", e);
                Self { raw_lines: vec![], names: vec![] }
            }
        }
    }
}

pub fn draw(frame: &mut Frame<'_>, config: &Config) {
    let data = DiskData::load();

    if data.names.is_empty() {
        let msg = Paragraph::new("No disks found or script failed.")
            .block(Block::default().title(" Disk Error ").borders(Borders::ALL));
        frame.render_widget(msg, frame.size());
        return;
    }

    let mode = DiskMode::SelectRoot;

    let items: Vec<ListItem> = data.raw_lines
        .iter()
        .map(|line| {
            let content = format!("{} {}", line, if Some(line.split_whitespace().next().unwrap_or("")) == config.root_disk.as_deref() { "(ROOT)" } else { "" });
            ListItem::new(content)
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title(match mode {
                    DiskMode::SelectRoot => " Select ROOT Disk (/) ",
                    DiskMode::SelectHome => " Select HOME Disk (/home) ",
                })
                .borders(Borders::ALL)
        )
        .highlight_style(ratatui::style::Style::default().bg(ratatui::style::Color::Cyan));

    frame.render_widget(list, frame.size());

    let hint = Paragraph::new("Tab: Switch Mode | Enter: Select | Esc: Back")
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::NONE));

}

pub fn handle_input(key: KeyCode, config: &mut Config) -> Action {

    match key {
        KeyCode::Esc => Action::GoTo(Screen::MainMenu),

        KeyCode::Tab => {
            Action::Stay
        }

        KeyCode::Down => {
            Action::Stay
        }

        KeyCode::Up => {
            Action::Stay
        }

        KeyCode::Enter => {
            Action::Stay
        }

        _ => Action::Stay,
    }
}
