use crate::data_storage::DataStorage;
use crate::repositories::SQLiteConfig;
use std::rc::Rc;

pub mod build;
mod explore;
mod fetch;

pub struct Facade {
    db: Box<dyn DataStorage>,
    db_cache: Box<dyn DataStorage>,
    connection: Rc<rusqlite::Connection>,
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

        let conn = rusqlite::Connection::open(cache_path).unwrap();
        Self {
            db,
            db_cache,
            connection: Rc::new(conn),
        }
    }

    pub fn storage<'a>(&self) -> &Box<dyn DataStorage> {
        &self.db_cache
    }
}
