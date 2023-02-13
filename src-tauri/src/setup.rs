/*
 * @Author: Yorn Qiu
 * @Date: 2023-02-10 14:41:02
 * @LastEditors: Yorn Qiu
 * @LastEditTime: 2023-02-13 16:34:04
 * @Description: file content
 * @FilePath: /power-copy/src-tauri/src/setup.rs
 */

use tauri::{App, Manager};
use window_vibrancy::{self, NSVisualEffectMaterial};
use window_shadows::set_shadow;

pub fn set_window_vibrancy(app: &mut App) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let window = app.get_window("main").unwrap();

    #[cfg(target_os = "macos")]
    window_vibrancy::apply_vibrancy(&window, NSVisualEffectMaterial::FullScreenUI, None, None)
        .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

    #[cfg(target_os = "windows")]
    window_vibrancy::apply_blur(&window, Some((18, 18, 18, 125)))
        .expect("Unsupported platform! 'apply_blur' is only supported on Windows");

    Ok(())
}

pub fn set_window_shadow(app: &mut App) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let window = app.get_window("main").unwrap();
    set_shadow(&window, true).expect("Unsupported platform!");

    Ok(())
}
