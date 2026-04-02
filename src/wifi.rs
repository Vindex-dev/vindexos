use crossterm::event::KeyCode;
use ratatui::{
    layout::Alignment,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::config::Config;
use crate::screen::{Action, Screen};

const AVAILABLE_NETWORKS: &[&str] = &[
    "Home_WiFi_5G",
    "Cafe_Free_Access",
    "Neighbor_Network",
    "Hidden_Network",
    "Guest_Network",
];

pub fn draw(frame: &mut Frame<'_>, config: &Config) {
    let mut lines = Vec::new();

    for (index, ssid) in AVAILABLE_NETWORKS.iter().enumerate() {
        let marker = if index == config.wifi_cursor { "> " } else { "  " };
        let status = if config.wifi_ssid.as_deref() == Some(*ssid) { " [✓]" } else { "" };

        lines.push(format!("{}{}{}", marker, ssid, status));
    }

    lines.push(String::new());
    lines.push(format!("Selected: {}",
        config.wifi_ssid.as_deref().unwrap_or("none")));
    lines.push(format!("Password: {}",
        if config.wifi_pass.is_empty() { "[not set]" } else { "***" }));
    lines.push(String::new());
    lines.push("Press: ↑↓ navigate | Enter select | Esc back".to_string());

    let display_text = lines.join("\n");

    let paragraph = Paragraph::new(display_text)
        .block(
            Block::default()
                .title(" WiFi Networks ")
                .borders(Borders::ALL)
        )
        .alignment(Alignment::Left);

    frame.render_widget(&paragraph, frame.size());
}

pub fn handle_input(key: KeyCode, config: &mut Config) -> Action {
    match key {
            KeyCode::Esc => Action::GoTo(Screen::MainMenu),

    KeyCode::Up => {
        if config.wifi_cursor > 0 { config.wifi_cursor -= 1; }
        Action::Stay
    }
    KeyCode::Down => {
        if config.wifi_cursor < AVAILABLE_NETWORKS.len() - 1 { config.wifi_cursor += 1; }
        Action::Stay
    }

    KeyCode::Enter => {
        if let Some(ssid) = AVAILABLE_NETWORKS.get(config.wifi_cursor) {
            config.wifi_ssid = Some(ssid.to_string());
        }
        Action::Stay
    }

    KeyCode::Char('c') => {
        if config.wifi_ssid.is_some() { config.wifi_pass.clear(); }
        Action::Stay
    }

    KeyCode::Char(c) => {
        if config.wifi_ssid.is_some() { config.wifi_pass.push(c); }
        Action::Stay
    }

    KeyCode::Backspace => {
        if config.wifi_ssid.is_some() { config.wifi_pass.pop(); }
        Action::Stay
    }

    _ => Action::Stay,
    }
}
