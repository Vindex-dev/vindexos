mod config;
mod screen;
mod main_menu;
mod wifi;

use config::Config;
use screen::{Screen, Action};
use main_menu::{draw as draw_main, handle_input as handle_main_input};
use wifi::{draw as draw_wifi, handle_input as handle_wifi_input};

use crossterm::{
    event::{self, Event},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io::{self, stdout};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    let mut config = Config::new();
    let mut current_screen = Screen::MainMenu;
    let mut running = true;

    while running {
        terminal.draw(|frame| {
            match current_screen {
                Screen::MainMenu => draw_main(frame, &config),
                Screen::WifiMenu => draw_wifi(frame, &config),
            }
        })?;

        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                let action = match current_screen {
                    Screen::MainMenu => handle_main_input(key.code, &mut config),
                    Screen::WifiMenu => handle_wifi_input(key.code, &mut config),
                };

                match action {
                    Action::Stay => {}
                    Action::GoTo(screen) => current_screen = screen,
                    Action::Exit => running = false,
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    println!("\n=== Config ===");
    println!("Username: {}", config.username);
    println!("Hostname: {}", config.hostname);
    println!("WiFi: {:?}", config.wifi_ssid);

    Ok(())
}
