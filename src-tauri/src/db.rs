use rusqlite::Connection;
use std::sync::Mutex;
use tauri::{AppHandle, Manager};

pub struct DbState(pub Mutex<Connection>);

pub fn init_db(app: &AppHandle) -> Result<Connection, rusqlite::Error> {
    let app_dir = app.path().app_data_dir().expect("Failed to get app data dir");
    std::fs::create_dir_all(&app_dir).expect("Failed to create app data dir");
    let db_path = app_dir.join("database.sqlite");
    
    let conn = Connection::open(db_path)?;
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            username TEXT UNIQUE NOT NULL,
            password_hash TEXT NOT NULL,
            role TEXT NOT NULL,
            status TEXT NOT NULL,
            created_at TEXT NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS sessions (
            id INTEGER PRIMARY KEY CHECK (id = 1),
            user_id TEXT NOT NULL,
            username TEXT NOT NULL,
            FOREIGN KEY(user_id) REFERENCES users(id)
        )",
        [],
    )?;

    Ok(conn)
}
