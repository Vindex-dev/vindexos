use crossterm::{
    event,
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

    terminal.draw(|frame: &mut Frame<'_>| {
        let paragraph = Paragraph::new("Hello, Arch!")
            .block(
                Block::default()
                    .title(" Installer ")
                    .borders(Borders::ALL)
            )
            .alignment(Alignment::Center);

        frame.render_widget(&paragraph, frame.size());
    })?;

    event::read()?;

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}
