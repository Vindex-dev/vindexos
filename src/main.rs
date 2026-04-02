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
    let mut password = String::new();
    let mut password_conf = String::new();

    let mut is_valid: bool = true;

    let mut running = true;
    let mut af = 0;
    // af - active field
    // 0 = username
    // 1 = hostname
    // 2 = password
    // 3 = password confirmation


    while running {
        terminal.draw(|frame: &mut Frame<'_>| {
            let display_text = format!("{}Username: {}\n {}Hostname: {}\n {}Password: {}\n {}Confirm your password({}): {}",
                (if af == 0 {">"} else {" "}),username, (if af == 1 {">"} else {" "}),hostname,
                (if af == 2 {">"} else {" "}),("*".repeat(password.len())), (if af == 3 {">"} else {" "}), (
                    if password == password_conf {is_valid = true; ""} else { is_valid = false; "Passwords must match"}
                ),password_conf);

            let paragraph = Paragraph::new(display_text)
                .block(Block::default().title(" Setup ").borders(Borders::ALL))
                .alignment(Alignment::Center);

            frame.render_widget(&paragraph, frame.size());
        })?;
        if event::poll(std::time::Duration::from_millis(5))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Down => {
                        if af < 3 {
                            af += 1;
                        } else { af = 0 }
                    },
                    KeyCode::Up => {
                        if af > 0 {
                            af -= 1;
                           } else { af = 3 }
                        },
                    KeyCode::Esc => running = false,
                    KeyCode::Char(c) => {
                        if af == 0 {
                            username.push(c);
                        } else if af == 1 {
                            hostname.push(c);
                        } else if af == 2 {
                            password.push(c);
                        } else if af == 3 {
                            password_conf.push(c);
                        }
                    },
                    KeyCode::Backspace => {
                        if af == 0{
                            username.pop();
                        } else if af == 1 {
                            hostname.pop();
                        } else if af == 2 {
                            password.pop();
                        } else if af == 3 {
                            password_conf.pop();
                        }
                    }
                    _ => {}
                }
            }
        }
    }



    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    println!("\n username : {}\n hostname : {}\n password : {}", username, hostname, password);
    Ok(())
}
