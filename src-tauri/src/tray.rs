/*
 * @Autoor: Yorn Qiu
 * @Date: 2023-02-22 10:11:32
 * @LastEditors: Yorn Qiu
 * @LastEditTime: 2023-02-22 15:19:40
 * @FilePath: /power-copy/src-tauri/src/tray.rs
 * @Description: system tray
 */

use tauri::{
    AppHandle, CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem,
    SystemTraySubmenu,
};

pub fn menu() -> SystemTray {
    let menu = SystemTrayMenu::new()
        .add_submenu(SystemTraySubmenu::new(
            "Theme",
            SystemTrayMenu::new()
                .add_item(CustomMenuItem::new("theme_light", "Light"))
                .add_item(CustomMenuItem::new("theme_dark", "Dark")),
        ))
        .add_submenu(SystemTraySubmenu::new(
            "Language",
            SystemTrayMenu::new()
                .add_item(CustomMenuItem::new("lang_en", "English"))
                .add_item(CustomMenuItem::new("lang_zh", "简体中文")),
        ))
        .add_item(CustomMenuItem::new("about", "About"))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("quit", "Quit"));

    SystemTray::new().with_menu(menu)
}

pub fn handler(app_handle: &AppHandle, event: SystemTrayEvent) {
    let handle = app_handle.tray_handle();

    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "theme_light" => {
                handle.get_item("theme_light").set_selected(true).unwrap();
                handle.get_item("theme_dark").set_selected(false).unwrap();
            }
            "theme_dark" => {
                handle.get_item("theme_dark").set_selected(true).unwrap();
                handle.get_item("theme_light").set_selected(false).unwrap();
            }
            "lang_en" => {
                handle.get_item("lang_en").set_selected(true).unwrap();
                handle.get_item("lang_zh").set_selected(false).unwrap();
            }
            "lang_zh" => {
                handle.get_item("lang_zh").set_selected(true).unwrap();
                handle.get_item("lang_en").set_selected(false).unwrap();
            }
            "about" => {}
            "quit" => app_handle.exit(0),
            _ => {}
        },
        _ => {}
    }
}
