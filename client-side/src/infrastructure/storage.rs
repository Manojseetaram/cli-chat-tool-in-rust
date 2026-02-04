use rusqlite::{Connection, params};
use crate::domain::message::Message;

pub struct MessageStore {
    conn: Connection,
}

impl MessageStore {
    pub fn new() -> Self {
        let conn = Connection::open("messages.db").unwrap();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS messages (
                id TEXT PRIMARY KEY,
                account TEXT,
                body TEXT,
                timestamp INTEGER,
                synced INTEGER
            )",
            []
        ).unwrap();

        Self { conn }
    }

    pub fn save(&self, msg: &Message) {
        self.conn.execute(
            "INSERT OR REPLACE INTO messages VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                msg.id.to_string(),
                msg.account,
                msg.body,
                msg.timestamp,
                msg.synced as i32
            ],
        ).unwrap();
    }

    pub fn unsynced(&self) -> Vec<Message> {
        let mut stmt = self.conn.prepare(
            "SELECT id, account, body, timestamp FROM messages WHERE synced = 0"
        ).unwrap();

        stmt.query_map([], |row| {
            Ok(Message {
                id: row.get::<_, String>(0)?.parse().unwrap(),
                account: row.get(1)?,
                body: row.get(2)?,
                timestamp: row.get(3)?,
                synced: false,
            })
        }).unwrap().map(|r| r.unwrap()).collect()
    }

    pub fn mark_synced(&self, id: &str) {
        self.conn.execute(
            "UPDATE messages SET synced = 1 WHERE id = ?1",
            params![id]
        ).unwrap();
    }
}
