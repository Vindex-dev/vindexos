use crossterm::event::{Event, KeyCode};
use ratatui::{
    layout::Alignment,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::config::Config;
use crate::screen::{Action, Screen};

pub fn draw(frame: &mut Frame<'_>, config: &Config) {
    let display = format!(
        "{}Username: {}\n {}Hostname: {}\n {}Password: {}\n {}Confirm({}): {}",
        marker(config.active_field == 0), &config.username,
        marker(config.active_field == 1), &config.hostname,
        marker(config.active_field == 2), &"*".repeat(config.password.len()),
        marker(config.active_field == 3),
        if config.password == config.password_conf { "OK" } else { "MISMATCH" },
        &config.password_conf
    );

    let paragraph = Paragraph::new(display)
        .block(Block::default().title(" Setup ").borders(Borders::ALL))
        .alignment(Alignment::Center);

    frame.render_widget(&paragraph, frame.size());
}

pub fn handle_input(key: KeyCode, config: &mut Config) -> Action {
    match key {
        KeyCode::Esc => Action::Exit,

        KeyCode::Down => {
            if config.active_field < 3 {
                config.active_field += 1;
            } else {
                config.active_field = 0;
            }
            Action::Stay
        }
        KeyCode::Up => {
            if config.active_field > 0 {
                config.active_field -= 1;
            } else {
                config.active_field = 3;
            }
            Action::Stay
        }

        // Ввод текста в активное поле
        KeyCode::Char(c) => {
            match config.active_field {
                0 => config.username.push(c),
                1 => config.hostname.push(c),
                2 => config.password.push(c),
                3 => config.password_conf.push(c),
                _ => {}
            }
            Action::Stay
        }

        // Удаление символа
        KeyCode::Backspace => {
            match config.active_field {
                0 => { config.username.pop(); }
                1 => { config.hostname.pop(); }
                2 => { config.password.pop(); }
                3 => { config.password_conf.pop(); }
                _ => {}
            }
            Action::Stay
        }

        // Переход в меню WiFi по Enter (заглушка)
        KeyCode::Enter => {
            if config.active_field == 1 { // Например, переход с Hostname
                Action::GoTo(Screen::WifiMenu)
            } else {
                Action::Stay
            }
        }

        _ => Action::Stay,
    }
}

/// Вспомогательная функция для маркера активного поля
fn marker(is_active: bool) -> &'static str {
    if is_active { "> " } else { "  " }
}
