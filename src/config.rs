pub struct Config {
    pub username: String,
    pub hostname: String,
    pub password: String,
    pub password_conf: String,

    pub wifi_ssid: Option<String>,
    pub wifi_pass: String,

    pub main_cursor: usize, // for main menu
    pub wifi_cursor: usize, // for wifi menu
    pub is_valid: bool,
}

impl Config {
    pub fn new() -> Self {
        Self {
            username: String::new(),
            hostname: String::new(),
            password: String::new(),
            password_conf: String::new(),
            wifi_ssid: None,
            wifi_pass: String::new(),
            main_cursor: 0,
            wifi_cursor: 0,
            is_valid: true,
        }
    }
}
