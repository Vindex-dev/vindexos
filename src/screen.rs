#[derive(Clone, Copy, PartialEq)]
pub enum Screen {
    MainMenu,
    WifiMenu,
    TimezoneMenu,
    DiskMenu,
    KeyboardMenu,
    KeymapMenu,
    Installing,
}

pub enum Action {
    Stay,
    GoTo(Screen),
    Exit,
    Install,
}
