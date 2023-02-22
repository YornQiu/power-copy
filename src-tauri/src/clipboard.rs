use std::{thread, time};

use crate::storage::{Record, Storage};
use anyhow::Result;
use arboard;

pub struct Clipboard;

impl Clipboard {
    pub fn watch() {
        tauri::async_runtime::spawn(async move {
            let duration = 500;
            let mut clipboard = arboard::Clipboard::new().unwrap();
            let mut last_content = String::new();

            loop {
                let text = clipboard.get_text();

                match text {
                    Ok(content) => {
                        if !content.is_empty() && content != last_content {
                            Storage::new()
                                .insert_one(Record {
                                    id: 0,
                                    ctype: "text".to_string(),
                                    content: content.clone(),
                                    create_at: "".to_string(),
                                })
                                .unwrap();

                            last_content = content.to_string();
                        }
                    }
                    Err(_) => (),
                }

                thread::sleep(time::Duration::from_millis(duration));
            }
        });
    }
}
