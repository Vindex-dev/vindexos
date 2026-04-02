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

    pub wifi_networks: Vec<String>,      // Кэш списка сетей
    pub wifi_needs_refresh: bool,        // Флаг: нужно ли обновить список
    pub tz_cursor:   usize, // for timezones fzf menu
    pub disk_cursor: usize,

    pub timezone: Option<String>,

    pub root_disk: Option<String>,
    pub home_disk: Option<String>,
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
            wifi_networks: Vec::new(),
            wifi_needs_refresh: true,
            tz_cursor: 0,
            disk_cursor: 0,
            timezone: None,
            root_disk: None,
            home_disk: None,
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
