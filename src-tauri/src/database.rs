use rusqlite::{params, Connection, Result};
use tauri::api::path::home_dir;

pub struct DB {
    conn: Connection,
}

pub struct Record {
    pub id: u32,
    pub ctype: String,
    pub content: String,
    pub create_at: u32,
}

impl DB {
    pub fn new() -> Self {
        let path = home_dir().unwrap();
        let conn = Connection::open(path).unwrap();
        DB { conn }
    }

    pub fn create_db() {
        let path = home_dir().unwrap();
        let conn = Connection::open(path).unwrap();
        let sql = r#"
        CREATE TABLE IF NOT EXISTS record
        (
          id INTERGER NOT NULL PRIMARY KEY AUTOINCREMENT,
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
        let rows = stmt.query_map([], |row| {
            Ok(Record {
                id: row.get(0)?,
                ctype: row.get(1)?,
                content: row.get(2)?,
                create_at: row.get(3)?,
            })
        })?;
        let mut res = Vec::new();
        for row in rows {
            res.push(row?);
        }

        Ok(res)
    }

    pub fn insert_one(&self, record: Record) -> Result<i64> {
        let sql = "INSERT INTO record (id,ctype,content) VALUES (?)";

        let row = self
            .conn
            .execute(sql, params![record.id, record.ctype, record.content])?;

        Ok(self.conn.last_insert_rowid())
    }

    pub fn delete_all(&self) {}

    pub fn delete_many(&self) {}

    pub fn delete_by_id(&self, id: u32) -> Result<()> {
        let sql = "DELETE FROM record WHERE id = ?";
        self.conn.execute(sql, [&id])?;

        Ok(())
    }
}
