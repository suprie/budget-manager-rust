use rusqlite::{Connection, Result};
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

pub fn open_database(app_handle: AppHandle) -> Result<Connection, String> {
    let path = database_path(app_handle)?;
    let conn = Connection::open(path).map_err(|_| "unable to open the file".to_string())?;

    let _ = migrate_database(&conn)?;
    Ok(conn)
}

pub fn reset_db(app_handle: AppHandle) -> Result<(), String> {
    let path = database_path(app_handle)?;
    fs::remove_file(path).map_err(|error| error.to_string());
    println!("Database is deleted");
    
    Ok(())
}

fn database_path(app: AppHandle) -> Result<PathBuf, String> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|error| error.to_string())?;

    if !Path::new(&app_data_dir).exists() {
        fs::create_dir_all(&app_data_dir).map_err(|error| error.to_string())?;
    }
    let db_path = app_data_dir.join("budget-manager.sqlite");

    Ok(db_path)
}

fn migrate_database(conn: &Connection) -> Result<(), String> {
    let version: i64 = conn
        .query_row("PRAGMA user_version", [], |row| row.get(0))
        .map_err(|error| error.to_string())?;

    if version < 1 {
        conn.execute_batch(
            "
              CREATE TABLE transactions (
                  id INTEGER PRIMARY KEY AUTOINCREMENT,
                  trx_date TEXT NOT NULL,
                  description TEXT NOT NULL,
                  amount REAL NOT NULL,
                  trx_type TEXT NOT NULL,
                  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                  UNIQUE (trx_date, description, amount, trx_type)
              );

              PRAGMA user_version = 1;
              ",
        )
        .map_err(|error| error.to_string())?;

        println!("Migration completed");
    } else {
        println!("DB already migrated");
    }

    Ok(())
}
