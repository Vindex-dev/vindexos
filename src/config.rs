pub struct Config {
    pub username: String,
    pub hostname: String,
    pub password: String,
    pub password_conf: String,

    pub wifi_ssid: Option<String>,  // None = не выбран, Some("Name") = выбран
    pub wifi_pass: String,

    pub active_field: usize,  // Какое поле сейчас активно (0, 1, 2...)
    pub is_valid: bool,       // Валидны ли данные (например, пароли совпадают)
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
            active_field: 0,
            is_valid: true,
        }
    }
}
