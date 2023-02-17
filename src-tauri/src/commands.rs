/*
 * @Author: Yorn Qiu
 * @Date: 2023-02-10 15:00:57
 * @LastEditors: Yorn Qiu
 * @LastEditTime: 2023-02-16 14:11:13
 * @Description: file content
 * @FilePath: /power-copy/src-tauri/src/commands.rs
 */

use crate::database::{Record,DB};

#[tauri::command]
pub fn get_records() -> Vec<Record> {
  DB::new().find_all().unwrap()
}

pub fn insert_record(id: String) {}

pub fn delete_record() {}

pub fn clear_record() {}
