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
    let Ok(content) = std::fs::read_to_string("/etc/locale.gen") else {
        return vec!["en_US.UTF-8".to_string(), "ru_RU.UTF-8".to_string()];
    };
    content
        .lines()
        .filter_map(|line| line.strip_prefix('#'))
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty() && s.contains('.'))
        .collect()
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
    let matches = filtered(&config.kb_query);

    let title = if config.kb_picking_second {
        " Locale — Second (optional) "
    } else {
        " Locale — Primary "
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)])
        .split(frame.size());

    frame.render_widget(
        Paragraph::new(format!("> {}_", config.kb_query))
            .block(Block::default().title(title).borders(Borders::ALL)),
        chunks[0],
    );

    let items: Vec<ListItem> = matches.iter().enumerate().map(|(i, k)| {
        let prefix = if i == config.kb_cursor { "> " } else { "  " };
        let suffix = if config.locale.as_deref() == Some(k)
            || config.locale2.as_deref() == Some(k)
        { " [✓]" } else { "" };
        ListItem::new(format!("{}{}{}", prefix, k, suffix))
    }).collect();

    frame.render_widget(
        List::new(items).block(Block::default().borders(Borders::ALL)),
        chunks[1],
    );

    let hint = if config.kb_picking_second {
        "Enter: set 2nd layout | Esc: skip / back"
    } else {
        "Enter: set layout | Esc: back"
    };
    frame.render_widget(
        Paragraph::new(hint)
            .block(Block::default().borders(Borders::ALL))
            .alignment(ratatui::layout::Alignment::Center),
        chunks[2],
    );
}

pub fn handle_input(key: KeyCode, config: &mut Config) -> Action {
    let matches = filtered(&config.kb_query);
    let max = matches.len().saturating_sub(1);

    match key {
        KeyCode::Esc => {
            config.kb_query.clear();
            config.kb_cursor = 0;
            config.kb_picking_second = false;
            Action::GoTo(Screen::MainMenu)
        }
        KeyCode::Up => {
            if config.kb_cursor > 0 { config.kb_cursor -= 1; }
            Action::Stay
        }
        KeyCode::Down => {
            if config.kb_cursor < max { config.kb_cursor += 1; }
            Action::Stay
        }
        KeyCode::Enter => {
            if let Some(k) = matches.get(config.kb_cursor) {
                if config.kb_picking_second {
                    config.locale2 = Some(k.to_string());
                    config.kb_picking_second = false;
                    config.kb_query.clear();
                    config.kb_cursor = 0;
                    return Action::GoTo(Screen::MainMenu);
                } else {
                    config.locale = Some(k.to_string());
                    config.kb_picking_second = true;
                    config.kb_query.clear();
                    config.kb_cursor = 0;
                }
            }
            Action::Stay
        }
        KeyCode::Backspace => {
            config.kb_query.pop();
            config.kb_cursor = 0;
            Action::Stay
        }
        KeyCode::Char(c) => {
            config.kb_query.push(c);
            config.kb_cursor = 0;
            Action::Stay
        }
        _ => Action::Stay,
    }
}
