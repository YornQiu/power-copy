use std::{thread, time::Duration};

use crate::database::{Record, DB};
use anyhow::Result;
use arboard;

pub struct Clipboard;

impl Clipboard {
    pub fn set(content: String) -> Result<()> {
        let mut clipboard = arboard::Clipboard::new()?;
        clipboard.set_text(content)?;

        Ok(())
    }

    pub fn watch() {
        tauri::async_runtime::spawn(async move {
            let duration = 500;
            let mut clipboard = arboard::Clipboard::new().unwrap();
            let mut last_content = String::new();

            loop {
                let text = clipboard.get_text();

                text.map(|content| {
                    if !content.is_empty() && content != last_content {
                        let res = DB::new().insert_one(Record {
                            ctype: "".to_string(),
                            content,
                            id: todo!(),
                            create_at: todo!(),
                        });

                        match res {
                            Ok(_) => {}
                            Err(e) => {
                                println!("insert record error: {}", e);
                            }
                        }

                        last_content = content.to_string();
                    }
                });

                thread::sleep(Duration::from_millis(duration));
            }
        });
    }
}
