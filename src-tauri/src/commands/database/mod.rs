mod entities;

pub use entities::*;
use once_cell::sync::OnceCell;
use surrealdb::{Surreal, engine::local::RocksDb};

use crate::MAIN_FOLDER_PREFIX;
pub static DB: OnceCell<DbRepo> = OnceCell::new();

pub async fn init() {
    let db_path = format!("{}db", MAIN_FOLDER_PREFIX);
    let db = Surreal::new::<RocksDb>(db_path).await.unwrap();
    db.use_ns("mindbreaker").use_db("config").await.unwrap();
    let db = DbRepo::new(db);
    DB.set(db).map_err(|_| EntityError::DbQueryError).unwrap();
}

#[tauri::command]
pub async fn create_first_database(app: tauri::AppHandle) -> Result<(), String> {
    init().await;
    match DB
        .get()
        .unwrap()
        .get::<MainConfig>("singletone", "main_config")
        .await
    {
        Ok(_) => Ok(()),
        Err(_) => {
            let input = vec![
                MainConfigFields::Name("".to_string()),
                MainConfigFields::FirstRun("true".to_string()),
                MainConfigFields::Workdir("".to_string()),
                MainConfigFields::NvimPath("/opt/homebrew/bin/nvim".to_string()),
            ];
            DB.get()
                .unwrap()
                .create::<MainConfigFields, MainConfig>(input, app, "main_config", "singletone")
                .await
                .map_err(|e| e.to_string())
        }
    }
}
