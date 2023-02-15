/*
 * @Author: Yorn Qiu
 * @Date: 2023-02-10 08:59:25
 * @LastEditors: Yorn Qiu
 * @LastEditTime: 2023-02-15 13:06:54
 * @Description: file content
 * @FilePath: /power-copy/src-tauri/src/main.rs
 */

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use setup::set_window_shadow;

mod setup;
mod database;

fn main() {
    tauri::Builder::default()
        .setup(set_window_shadow)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}