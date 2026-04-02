#[derive(Clone, Copy, PartialEq)]
pub enum Screen {
    MainMenu,
    WifiMenu,
    // DiskMenu,
}

pub enum Action {
    Stay,
    GoTo(Screen),
    Exit,
}
