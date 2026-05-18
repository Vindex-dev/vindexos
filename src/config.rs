use serde::Serialize;

#[derive(Serialize)]
pub struct Config {
    pub username: String,
    pub hostname: String,
    pub password: String,
    pub password_conf: String,

    pub wifi_ssid: Option<String>,
    pub wifi_pass: String,

    #[serde(skip)] pub main_cursor: usize,
    #[serde(skip)] pub wifi_cursor: usize,

    #[serde(skip)] pub wifi_networks: Vec<String>,
    #[serde(skip)] pub wifi_needs_refresh: bool,
    #[serde(skip)] pub tz_cursor: usize,
    #[serde(skip)] pub tz_query: String,
    #[serde(skip)] pub disk_cursor: usize,

    pub timezone: Option<String>,

    pub root_disk: Option<String>,
    pub home_disk: Option<String>,

    pub locale: Option<String>,
    pub locale2: Option<String>,
    pub keyboard: Option<String>,
    #[serde(skip)] pub kb_query: String,
    #[serde(skip)] pub kb_cursor: usize,
    #[serde(skip)] pub kb_picking_second: bool,
    #[serde(skip)] pub keymap_query: String,
    #[serde(skip)] pub keymap_cursor: usize,
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
            wifi_networks: Vec::new(),
            wifi_needs_refresh: true,
            tz_cursor: 0,
            tz_query: String::new(),
            disk_cursor: 0,
            timezone: None,
            root_disk: None,
            home_disk: None,
            locale: None,
            locale2: None,
            keyboard: None,
            kb_query: String::new(),
            kb_cursor: 0,
            kb_picking_second: false,
            keymap_query: String::new(),
            keymap_cursor: 0,
        }
    }
    pub fn refresh_networks(&mut self) {
        use std::process::Command;

        let output = match Command::new("iwctl")
            .args(["station", "wlan0", "get-networks"])
            .output() {
                Ok(out) => out,
                Err(_) => return,
            };

        let stdout = String::from_utf8_lossy(&output.stdout);

        self.wifi_networks = stdout
            .lines()
            .skip(5)
            .filter_map(|line| line.split_whitespace().next())
            .filter(|ssid| *ssid != "SSID" && !ssid.is_empty())
            .map(|s| s.to_string())
            .collect();

        if !self.wifi_networks.is_empty() && self.wifi_cursor >= self.wifi_networks.len() {
            self.wifi_cursor = self.wifi_networks.len().saturating_sub(1);
        }
    }
}
