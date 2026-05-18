mod config;
mod screen;
mod main_menu;
mod wifi;
mod timezones;
mod disk_menu;
mod keyboard;
mod installing;

use config::Config;
use screen::{Screen, Action};
use serde_json;
use main_menu::{draw as draw_main, handle_input as handle_main_input};
use wifi::{draw as draw_wifi, handle_input as handle_wifi_input};
use timezones::{draw as draw_tz, handle_input as handle_tz_input};
use disk_menu::{draw as draw_disk, handle_input as handle_disk_input};
use keyboard::{draw as draw_kb, handle_input as handle_kb_input};
use installing::{draw as draw_install, handle_input as handle_install_input, InstallState};

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
    let mut install_state = InstallState::new();
    let mut current_screen = Screen::MainMenu;
    let mut running = true;

    while running {
        if current_screen == Screen::Installing {
            install_state.update();
        }

        terminal.draw(|frame| {
            match current_screen {
                Screen::MainMenu => draw_main(frame, &mut config),
                Screen::WifiMenu => draw_wifi(frame, &mut config),
                Screen::TimezoneMenu => draw_tz(frame, &config),
                Screen::DiskMenu => draw_disk(frame, &config),
                Screen::KeyboardMenu => draw_kb(frame, &config),
                Screen::Installing => draw_install(frame, &install_state),
            }
        })?;

        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                let action = match current_screen {
                    Screen::MainMenu => handle_main_input(key.code, &mut config),
                    Screen::WifiMenu => handle_wifi_input(key.code, &mut config),
                    Screen::TimezoneMenu => handle_tz_input(key.code, &mut config),
                    Screen::DiskMenu => handle_disk_input(key.code, &mut config),
                    Screen::KeyboardMenu => handle_kb_input(key.code, &mut config),
                    Screen::Installing => handle_install_input(key.code, &install_state),
                };

                match action {
                    Action::Stay => {}
                    Action::GoTo(screen) => {
                        current_screen = screen;
                    }
                    Action::Exit => running = false,
                    Action::Install => {
                        let json = serde_json::to_string_pretty(&config).unwrap();
                        std::fs::write("install_config.json", &json).unwrap();
                        install_state.start();
                        current_screen = Screen::Installing;
                    }
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}
