use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    backend::CrosstermBackend,
    layout::Alignment,
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use std::io::{self, stdout};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    let mut hostname = String::new();
    let mut username = String::new();
    let mut running = true;

    let mut active_field: usize = 0; // 0 = username, 1 = hostname


    while running {
        terminal.draw(|frame: &mut Frame<'_>| {
            let display_text = format!("{}Username: {}\n {}Hostname: {}", (if active_field == 0 {">"} else {" "}),username, (if active_field == 1 {">"} else {" "}),hostname);

            let paragraph = Paragraph::new(display_text)
                .block(Block::default().title(" Setup ").borders(Borders::ALL))
                .alignment(Alignment::Center);

            frame.render_widget(&paragraph, frame.size());
        })?;

        if event::poll(std::time::Duration::from_millis(5))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Down => active_field = {
                        if active_field == 0 {
                            1
                        } else { 0 }
                    },
                    KeyCode::Up => active_field = {
                        if active_field == 0 {
                            1
                        } else { 0 }
                    },
                    KeyCode::Esc => running = false,
                    KeyCode::Char(c) => {
                        if active_field == 0 {
                            username.push(c);
                        } else {
                            hostname.push(c);
                        }
                    },
                    KeyCode::Backspace => {
                        if active_field == 0{
                            username.pop();
                        } else {
                            hostname.pop();
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    println!("\n✅ Готово! Введённое имя: {}", username);
    Ok(())
}
