// src/wifi.rs
// Пример меню выбора WiFi для друга

use crossterm::event::KeyCode;
use ratatui::{
    layout::Alignment,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::config::Config;
use crate::screen::{Action, Screen};

/// Список сетей для отображения (хардкод, потом заменим на сканирование)
const AVAILABLE_NETWORKS: &[&str] = &[
    "Home_WiFi_5G",
    "Cafe_Free_Access",
    "Neighbor_Network",
    "Hidden_Network",
    "Guest_Network",
];

/// Отрисовка меню WiFi
pub fn draw(frame: &mut Frame<'_>, config: &Config) {
    // 1. Формируем список сетей для отображения
    let mut lines = Vec::new();

    for (index, ssid) in AVAILABLE_NETWORKS.iter().enumerate() {
        // Маркер выбранной сети
        let marker = if index == config.active_field { "> " } else { "  " };
        // Статус подключения
        let status = if config.wifi_ssid.as_deref() == Some(*ssid) { " [✓]" } else { "" };

        lines.push(format!("{}{}{}", marker, ssid, status));
    }

    // 2. Добавля информацию о выбранной сети и пароле
    lines.push(String::new()); // пустая строка-разделитель
    lines.push(format!("Selected: {}",
        config.wifi_ssid.as_deref().unwrap_or("none")));
    lines.push(format!("Password: {}",
        if config.wifi_pass.is_empty() { "[not set]" } else { "***" }));
    lines.push(String::new());
    lines.push("Press: ↑↓ navigate | Enter select | Esc back".to_string());

    // 3. Собираем всё в один текст с переносами строк
    let display_text = lines.join("\n");

    // 4. Создаём виджет
    let paragraph = Paragraph::new(display_text)
        .block(
            Block::default()
                .title(" WiFi Networks ")
                .borders(Borders::ALL)
        )
        .alignment(Alignment::Left); // Выравнивание по левому краю для списка

    // 5. Рендерим
    frame.render_widget(&paragraph, frame.size());
}

/// Обработка ввода в меню WiFi
pub fn handle_input(key: KeyCode, config: &mut Config) -> Action {
    match key {
        // Выход в главное меню
        KeyCode::Esc => Action::GoTo(Screen::MainMenu),

        // Навигация вверх/вниз по списку сетей
        KeyCode::Up => {
            if config.active_field > 0 {
                config.active_field -= 1;
            }
            Action::Stay
        }
        KeyCode::Down => {
            if config.active_field < AVAILABLE_NETWORKS.len() - 1 {
                config.active_field += 1;
            }
            Action::Stay
        }

        // Выбор сети по Enter
        KeyCode::Enter => {
            // Сохраняем выбранную сеть в конфиг
            if let Some(ssid) = AVAILABLE_NETWORKS.get(config.active_field) {
                config.wifi_ssid = Some(ssid.to_string());
                // Можно сразу перейти к вводу пароля или вернуться в главное
                // Пока просто остаёмся на экране
            }
            Action::Stay
        }

        // Ввод пароля (простая реализация: любые символы идут в пароль)
        // В реальном проекте тут нужно отдельное поле для ввода пароля
        KeyCode::Char(c) => {
            // Разрешаем вводить пароль только если сеть уже выбрана
            if config.wifi_ssid.is_some() {
                config.wifi_pass.push(c);
            }
            Action::Stay
        }

        // Удаление символов пароля
        KeyCode::Backspace => {
            if config.wifi_ssid.is_some() {
                config.wifi_pass.pop();
            }
            Action::Stay
        }

        // Очистка пароля по 'C' (Ctrl+C не ловим, это выход из программы)
        KeyCode::Char('c') => {
            if config.wifi_ssid.is_some() {
                config.wifi_pass.clear();
            }
            Action::Stay
        }

        // Все остальные клавиши игнорируем
        _ => Action::Stay,
    }
}
