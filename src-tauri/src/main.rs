/*
 * @Author: Yorn Qiu
 * @Date: 2023-02-10 08:59:25
 * @LastEditors: Yorn Qiu
 * @LastEditTime: 2023-02-21 15:53:54
 * @Description: file content
 * @FilePath: /power-copy/src-tauri/src/main.rs
 */

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::SystemTray;

mod clipboard;
mod commands;
mod setup;
mod storage;

fn main() {
    tauri::Builder::default()
        .setup(setup::setup)
        .system_tray(SystemTray::new())
        .invoke_handler(tauri::generate_handler![
            commands::get_records,
            commands::delete_record,
            commands::delete_records,
            commands::clear_record,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
