/*
 * @Author: Yorn Qiu
 * @Date: 2023-02-10 08:59:25
 * @LastEditors: Yorn Qiu
 * @LastEditTime: 2023-02-16 16:38:03
 * @Description: file content
 * @FilePath: /power-copy/src-tauri/src/main.rs
 */

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod clipboard;
mod commands;
mod database;
mod setup;

fn main() {
    tauri::Builder::default()
        .setup(setup::setup)
        // .invoke_handler(tauri::generate_handler![commands::get_records])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
