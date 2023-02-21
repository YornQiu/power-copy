/*
 * @Author: Yorn Qiu
 * @Date: 2023-02-15 11:14:52
 * @LastEditors: Yorn Qiu
 * @LastEditTime: 2023-02-21 10:21:51
 * @FilePath: /power-copy/src-tauri/src/storage.rs
 * @Description: sqlite operation
 */

use std::{fs::File, path::Path};

use rusqlite::{params, params_from_iter, Connection, Result, Row};
use serde::Serialize;

// Clipboard record
#[derive(Serialize, Clone, Debug)]
pub struct Record {
    pub id: u32,
    pub ctype: String,
    pub content: String,
    pub create_at: u32,
}

impl Record {
    fn parse(row: &Row) -> Result<Record, rusqlite::Error> {
        Ok(Record {
            id: row.get(0)?,
            ctype: row.get(1)?,
            content: row.get(2)?,
            create_at: row.get(3)?,
        })
    }
}

// sqlite storage
pub struct Storage {
    conn: Connection,
}

const DATA_FILE_NAME: &str = "data.db";

impl Storage {
    pub fn new() -> Storage {
        let path = Path::new(".").join(DATA_FILE_NAME);
        let conn = Connection::open(path).unwrap();
        Storage { conn }
    }

    pub fn init() {
        let path = Path::new(".").join(DATA_FILE_NAME);
        if !Path::new(&path).exists() {
            File::create(&path).unwrap();
        }

        let conn = Connection::open(path).unwrap();
        let sql = r#"
        CREATE TABLE IF NOT EXISTS record
        (
          id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
          ctype VARCHAR(16) DEFAULT '',
          content TEXT,
          create_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )
        "#;

        conn.execute(sql, ()).unwrap();
    }

    pub fn find_all(&self) -> Result<Vec<Record>> {
        let sql = "SELECT * FROM record ORDER BY create_at DESC";
        let mut stmt = self.conn.prepare(sql)?;
        let rows = stmt.query_map([], Record::parse)?;

        let mut res = Vec::new();
        for row in rows {
            res.push(row.unwrap());
        }

        Ok(res)
    }

    pub fn insert_one(&self, record: Record) -> Result<i64> {
        let sql = "INSERT INTO record (id,ctype,content) VALUES (?)";

        self.conn
            .execute(sql, params![record.ctype, record.content])?;

        Ok(self.conn.last_insert_rowid())
    }

    pub fn delete_by_id(&self, id: u32) -> Result<()> {
        let sql = "DELETE FROM record WHERE in = ?";
        self.conn.execute(sql, [&id])?;

        Ok(())
    }

    pub fn delete_many_by_ids(&self, ids: Vec<u32>) -> Result<()> {
        let sql = "DELETE FROM record WHERE id IN (?)";
        self.conn.execute(sql, params_from_iter(&ids))?;

        Ok(())
    }

    pub fn delete_all(&self) -> Result<()> {
        let sql = "DELETE FROM record";
        self.conn.execute(sql, ())?;

        Ok(())
    }
}
