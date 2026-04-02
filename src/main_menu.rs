use crossterm::event::KeyCode;
use ratatui::{
    layout::Alignment,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::config::Config;
use crate::screen::{Action, Screen};

pub fn draw(frame: &mut Frame<'_>, config: &Config) {
    let display = format!(
        "{}Username: {}\n{}Hostname: {}\n{}Password: {}\n{}Confirm your password{}: {}\n{}WiFi\n{}Timezone\n{}Disk partitioning",
        marker(config.main_cursor == 0), &config.username,
        marker(config.main_cursor == 1), &config.hostname,
        marker(config.main_cursor == 2), &"*".repeat(config.password.len()),
        marker(config.main_cursor == 3),
        if config.password == config.password_conf { "" } else { "(MISMATCH)" },
        &"*".repeat(config.password_conf.len()), marker(config.main_cursor == 4),
        marker(config.main_cursor == 5),
        marker(config.main_cursor == 6),
    );

    let paragraph = Paragraph::new(display)
        .block(Block::default().title(" Setup ").borders(Borders::ALL))
        .alignment(Alignment::Left);

    frame.render_widget(&paragraph, frame.size());
}

pub fn handle_input(key: KeyCode, config: &mut Config) -> Action {
    match key {
        KeyCode::Esc => Action::Exit,

        KeyCode::Down => {
            if config.main_cursor < 6 {
                config.main_cursor += 1;
            } else {
                config.main_cursor = 0;
            }
            Action::Stay
        }
        KeyCode::Up => {
            if config.main_cursor > 0 {
                config.main_cursor -= 1;
            } else {
                config.main_cursor = 6;
            }
            Action::Stay
        }

        KeyCode::Char(c) => {
            match config.main_cursor {
                0 => config.username.push(c),
                1 => config.hostname.push(c),
                2 => config.password.push(c),
                3 => config.password_conf.push(c),
                _ => {}
            }
            Action::Stay
        }

        KeyCode::Backspace => {
            match config.main_cursor {
                0 => { config.username.pop(); }
                1 => { config.hostname.pop(); }
                2 => { config.password.pop(); }
                3 => { config.password_conf.pop(); }
                _ => {}
            }
            Action::Stay
        }

        KeyCode::Enter => {
           return match config.main_cursor {
                4 => Action::GoTo(Screen::WifiMenu),
                5 => Action::GoTo(Screen::TimezoneMenu),
                6 => Action::GoTo(Screen::DiskMenu),
                _ => Action::Stay,
            };
        }
            _ => Action::Stay

    }
}

fn marker(is_active: bool) -> &'static str {
    if is_active { "> " } else { "  " }
}
