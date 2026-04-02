use crossterm::event::KeyCode;
use ratatui::{
    layout::Alignment,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::config::Config;
use crate::screen::{Action, Screen};

pub fn draw(frame: &mut Frame<'_>, config: &Config) {

    let display_text = "Timezone Menu (WIP)\nUse Up/Down to navigate\nPress Enter to select".to_string();

    let paragraph = Paragraph::new(display_text)
        .block(
            Block::default()
                .title(" Select Timezone ")
                .borders(Borders::ALL)
        )
        .alignment(Alignment::Left);

    frame.render_widget(&paragraph, frame.size());
}

pub fn handle_input(key: KeyCode, config: &mut Config) -> Action {
    match key {
        KeyCode::Esc => Action::GoTo(Screen::MainMenu),

        KeyCode::Enter => {
            Action::Stay
        }

        _ => Action::Stay,
    }
}
