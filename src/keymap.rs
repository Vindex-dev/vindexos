use crossterm::event::KeyCode;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use std::sync::OnceLock;

use crate::config::Config;
use crate::screen::{Action, Screen};

static KEYMAPS: OnceLock<Vec<String>> = OnceLock::new();

fn load_keymaps() -> Vec<String> {
    let output = std::process::Command::new("localectl")
        .args(["list-keymaps"])
        .output();
    if let Ok(out) = output {
        let s = String::from_utf8_lossy(&out.stdout);
        let maps: Vec<String> = s.lines().map(|l| l.trim().to_string()).filter(|l| !l.is_empty()).collect();
        if !maps.is_empty() {
            return maps;
        }
    }
    vec!["us".to_string(), "ru".to_string(), "de".to_string(), "fr".to_string(), "uk".to_string()]
}

fn keymaps() -> &'static Vec<String> {
    KEYMAPS.get_or_init(load_keymaps)
}

fn fuzzy_match(haystack: &str, query: &str) -> bool {
    let mut chars = haystack.chars();
    query.chars().all(|q| chars.any(|c| c.eq_ignore_ascii_case(&q)))
}

fn filtered(query: &str) -> Vec<&'static str> {
    keymaps().iter().map(|s| s.as_str()).filter(|k| fuzzy_match(k, query)).collect()
}

pub fn draw(frame: &mut Frame<'_>, config: &Config) {
    let matches = filtered(&config.keymap_query);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)])
        .split(frame.size());

    frame.render_widget(
        Paragraph::new(format!("> {}_", config.keymap_query))
            .block(Block::default().title(" Keyboard Layout ").borders(Borders::ALL)),
        chunks[0],
    );

    let items: Vec<ListItem> = matches.iter().enumerate().map(|(i, k)| {
        let prefix = if i == config.keymap_cursor { "> " } else { "  " };
        let suffix = if config.keyboard.as_deref() == Some(k) { " [✓]" } else { "" };
        ListItem::new(format!("{}{}{}", prefix, k, suffix))
    }).collect();

    frame.render_widget(
        List::new(items).block(Block::default().borders(Borders::ALL)),
        chunks[1],
    );

    frame.render_widget(
        Paragraph::new("Enter: select | Esc: back")
            .block(Block::default().borders(Borders::ALL))
            .alignment(ratatui::layout::Alignment::Center),
        chunks[2],
    );
}

pub fn handle_input(key: KeyCode, config: &mut Config) -> Action {
    let matches = filtered(&config.keymap_query);
    let max = matches.len().saturating_sub(1);

    match key {
        KeyCode::Esc => {
            config.keymap_query.clear();
            config.keymap_cursor = 0;
            Action::GoTo(Screen::MainMenu)
        }
        KeyCode::Up => {
            if config.keymap_cursor > 0 { config.keymap_cursor -= 1; }
            Action::Stay
        }
        KeyCode::Down => {
            if config.keymap_cursor < max { config.keymap_cursor += 1; }
            Action::Stay
        }
        KeyCode::Enter => {
            if let Some(k) = matches.get(config.keymap_cursor) {
                config.keyboard = Some(k.to_string());
                config.keymap_query.clear();
                config.keymap_cursor = 0;
                return Action::GoTo(Screen::MainMenu);
            }
            Action::Stay
        }
        KeyCode::Backspace => {
            config.keymap_query.pop();
            config.keymap_cursor = 0;
            Action::Stay
        }
        KeyCode::Char(c) => {
            config.keymap_query.push(c);
            config.keymap_cursor = 0;
            Action::Stay
        }
        _ => Action::Stay,
    }
}
