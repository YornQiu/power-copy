/*
 * @Author: Yorn Qiu
 * @Date: 2023-02-10 08:59:25
 * @LastEditors: Yorn Qiu
 * @LastEditTime: 2023-02-22 15:23:38
 * @Description: file content
 * @FilePath: /power-copy/src-tauri/src/main.rs
 */

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::RunEvent;

mod clipboard;
mod commands;
mod setup;
mod storage;
mod tray;

fn main() {
    tauri::Builder::default()
        .setup(setup::setup)
        .system_tray(tray::menu())
        .on_system_tray_event(tray::handler)
        .invoke_handler(tauri::generate_handler![
            commands::get_records,
            commands::delete_record,
            commands::delete_records,
            commands::clear_record,
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|app_handle, event| match event {
            RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            RunEvent::Exit => {
                app_handle.exit(0);
            }
            _ => {}
        });
}
