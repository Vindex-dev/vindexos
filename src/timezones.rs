use crossterm::event::KeyCode;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use std::sync::OnceLock;

use crate::config::Config;
use crate::screen::{Action, Screen};

static TIMEZONES: OnceLock<Vec<String>> = OnceLock::new();

fn load_timezones() -> Vec<String> {
    const ROOT: &str = "/usr/share/zoneinfo";
    const SKIP: &[&str] = &["posix", "right", "Etc", "SystemV", "leap-seconds.list", "posixrules", "tzdata.zi", "+VERSION", "leapseconds"];

    let mut result = Vec::new();
    let Ok(regions) = std::fs::read_dir(ROOT) else { return result };

    for region in regions.flatten() {
        let name = region.file_name();
        let name = name.to_string_lossy();
        if SKIP.contains(&&*name) { continue; }

        let path = region.path();
        if path.is_dir() {
            let Ok(cities) = std::fs::read_dir(&path) else { continue };
            for city in cities.flatten() {
                if city.path().is_file() {
                    result.push(format!("{}/{}", name, city.file_name().to_string_lossy()));
                }
            }
        } else if path.is_file() {
            result.push(name.to_string());
        }
    }

    result.sort();
    result
}

fn timezones() -> &'static Vec<String> {
    TIMEZONES.get_or_init(load_timezones)
}

fn fuzzy_match(haystack: &str, query: &str) -> bool {
    let mut chars = haystack.chars();
    query.chars().all(|q| chars.any(|c| c.eq_ignore_ascii_case(&q)))
}

fn filtered(query: &str) -> Vec<&'static str> {
    timezones().iter().map(|s| s.as_str()).filter(|tz| fuzzy_match(tz, query)).collect()
}

pub fn draw(frame: &mut Frame<'_>, config: &Config) {
    let matches = filtered(&config.tz_query);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(frame.size());

    let search = Paragraph::new(format!("> {}_", config.tz_query))
        .block(Block::default().title(" Select Timezone ").borders(Borders::ALL));
    frame.render_widget(search, chunks[0]);

    let items: Vec<ListItem> = matches.iter().enumerate().map(|(i, tz)| {
        let prefix = if i == config.tz_cursor { "> " } else { "  " };
        let suffix = if config.timezone.as_deref() == Some(tz) { " [✓]" } else { "" };
        ListItem::new(format!("{}{}{}", prefix, tz, suffix))
    }).collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(list, chunks[1]);
}

pub fn handle_input(key: KeyCode, config: &mut Config) -> Action {
    let matches = filtered(&config.tz_query);
    let max = matches.len().saturating_sub(1);

    match key {
        KeyCode::Esc => {
            config.tz_query.clear();
            config.tz_cursor = 0;
            Action::GoTo(Screen::MainMenu)
        }
        KeyCode::Up => {
            if config.tz_cursor > 0 { config.tz_cursor -= 1; }
            Action::Stay
        }
        KeyCode::Down => {
            if config.tz_cursor < max { config.tz_cursor += 1; }
            Action::Stay
        }
        KeyCode::Enter => {
            if let Some(tz) = matches.get(config.tz_cursor) {
                config.timezone = Some(tz.to_string());
            }
            config.tz_query.clear();
            config.tz_cursor = 0;
            Action::GoTo(Screen::MainMenu)
        }
        KeyCode::Backspace => {
            config.tz_query.pop();
            config.tz_cursor = 0;
            Action::Stay
        }
        KeyCode::Char(c) => {
            config.tz_query.push(c);
            config.tz_cursor = 0;
            Action::Stay
        }
        _ => Action::Stay,
    }
}
