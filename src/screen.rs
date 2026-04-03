#[derive(Clone, Copy, PartialEq)]
pub enum Screen {
    MainMenu,
    WifiMenu,
    TimezoneMenu,
    DiskMenu,
}

pub enum Action {
    Stay,
    GoTo(Screen),
    Exit,
    Install,
}
