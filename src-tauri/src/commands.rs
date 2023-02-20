/*
 * @Author: Yorn Qiu
 * @Date: 2023-02-10 15:00:57
 * @LastEditors: Yorn Qiu
 * @LastEditTime: 2023-02-20 14:57:15
 * @Description: file content
 * @FilePath: /power-copy/src-tauri/src/commands.rs
 */

use crate::database::{Record, DB};

#[tauri::command]
pub fn get_records() -> Vec<Record> {
    DB::new().find_all().unwrap()
}

#[tauri::command]
pub fn delete_record(id: u32) {
    DB::new().delete_by_id(id).unwrap()
}

#[tauri::command]
pub fn delete_records(ids: Vec<u32>) {
    DB::new().delete_many_by_ids(ids).unwrap()
}

#[tauri::command]
pub fn clear_record() {
    DB::new().delete_all().unwrap()
}
