/*
 * @Author: Yorn Qiu
 * @Date: 2023-02-10 14:41:02
 * @LastEditors: Yorn Qiu
 * @LastEditTime: 2023-02-21 17:07:10
 * @FilePath: /power-copy/src-tauri/src/setup.rs
 * @Description: app setup
 */

use tauri::{
    App, CustomMenuItem, GlobalShortcutManager, Manager, SystemTray, SystemTrayEvent,
    SystemTrayMenu, SystemTrayMenuItem,
};
use window_shadows::set_shadow;
use window_vibrancy::{self, NSVisualEffectMaterial, NSVisualEffectState};

use crate::clipboard::Clipboard;
use crate::storage::Storage;
use crate::tray_handles;

fn set_window_vibrancy(app: &mut App) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let window = app.get_window("main").unwrap();

    #[cfg(target_os = "macos")]
    window_vibrancy::apply_vibrancy(
        &window,
        NSVisualEffectMaterial::HudWindow,
        Some(NSVisualEffectState::Active),
        Some(8.0),
    )
    .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

    #[cfg(target_os = "windows")]
    window_vibrancy::apply_blur(&window, Some((18, 18, 18, 125)))
        .expect("Unsupported platform! 'apply_blur' is only supported on Windows");

    app.set_activation_policy(tauri::ActivationPolicy::Accessory);

    Ok(())
}

fn set_window_shadow(app: &mut App) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let window = app.get_window("main").unwrap();
    set_shadow(&window, true).expect("Unsupported platform!");

    Ok(())
}

fn set_system_tray(app: &mut App) -> std::result::Result<(), Box<dyn std::error::Error>> {
    SystemTray::new()
        .with_menu(
            SystemTrayMenu::new()
                .add_item(CustomMenuItem::new("about", "About"))
                .add_native_item(SystemTrayMenuItem::Separator)
                .add_item(CustomMenuItem::new("quit", "Quit")),
        )
        .on_event(move |event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "about" => tray_handles::display_about(),
                "quit" => tray_handles::quit(),
                _ => {}
            },
            _ => {}
        })
        .build(app)?;

    Ok(())
}

fn register_shortcut(app: &mut App) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let mut shortcut = app.global_shortcut_manager();
    let app_handler = app.handle();
    shortcut.register("shift+command+v", move || {
        let window = app_handler.get_window("main").unwrap();
        if window.is_visible().unwrap() {
            app_handler.hide().unwrap();
        } else {
            app_handler.show().unwrap();
            window.set_focus().unwrap();
        }
    })?;

    Ok(())
}

pub fn setup(app: &mut App) -> std::result::Result<(), Box<dyn std::error::Error>> {
    set_window_vibrancy(app)?;
    set_window_shadow(app)?;
    set_system_tray(app)?;
    register_shortcut(app)?;
    Storage::init();
    Clipboard::watch();

    Ok(())
}
