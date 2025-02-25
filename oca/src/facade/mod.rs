use rusqlite::Params;

use crate::data_storage::DataStorage;
use crate::repositories::SQLiteConfig;
use std::borrow::Borrow;
use std::sync::{Arc, Mutex};

pub mod build;
pub mod bundle;
mod explore;
mod fetch;
pub use said::{derivation::HashFunctionCode, sad::SerializationFormats, version::Encode};

#[derive(Clone)]
pub struct Connection {
    pub connection: Arc<Mutex<rusqlite::Connection>>,
}

impl Connection {
    pub fn new(path: &str) -> Self {
        let conn = rusqlite::Connection::open(path).unwrap();
        Self {
            connection: Arc::new(Mutex::new(conn)),
        }
    }

    pub fn execute<P>(&self, sql: &str, params: P) -> rusqlite::Result<usize>
    where
        P: Params,
    {
        let connection = self.connection.lock().unwrap();
        connection.execute(sql, params)
    }
}

pub struct Facade {
    db: Box<dyn DataStorage>,
    db_cache: Box<dyn DataStorage>,
    connection: Connection,
}

impl Facade {
    pub fn new(
        db: Box<dyn DataStorage>,
        db_cache: Box<dyn DataStorage>,
        cache_storage_config: SQLiteConfig,
    ) -> Self {
        let cache_path: String = match cache_storage_config.path {
            Some(path) => {
                if !path.try_exists().unwrap() {
                    std::fs::create_dir_all(path.clone()).unwrap();
                }
                path.join("search_data.db")
                    .clone()
                    .into_os_string()
                    .into_string()
                    .unwrap()
            }
            None => ":memory:".to_string(),
        };

        Self {
            db,
            db_cache,
            connection: Connection::new(&cache_path),
        }
    }

    pub(crate) fn connection(&self) -> Connection {
        self.connection.clone()
    }

    pub fn storage(&self) -> &dyn DataStorage {
        self.db_cache.borrow()
    }
}
