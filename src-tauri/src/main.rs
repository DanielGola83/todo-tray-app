#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, Manager, PhysicalPosition};

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Zamknij");
    let tray_menu = SystemTrayMenu::new().add_item(quit);
    let system_tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick { position, size, .. } => {
                let window = app.get_window("main").unwrap();
                if window.is_visible().unwrap() {
                    window.hide().unwrap();
                } else {
                    // Pozycjonowanie okna tuż nad ikonką w trayu
                    let win_size = window.outer_size().unwrap();
                    let x = position.x as i32 - (win_size.width as i32 / 2) + (size.width as i32 / 2);
                    let y = position.y as i32 - win_size.height as i32;

                    window.set_position(PhysicalPosition::new(x, y)).unwrap();
                    window.show().unwrap();
                    window.set_focus().unwrap();
                }
            }
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                _ => {}
            },
            _ => {}
        })
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::Focused(false) => {
                // Auto-ukrywanie okna po kliknięciu poza nim
                event.window().hide().unwrap();
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("błąd podczas uruchamiania aplikacji Tauri");
}
