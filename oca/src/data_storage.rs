use dyn_clonable::*;
use std::collections::HashMap;

#[clonable]
pub trait DataStorage: Clone {
    fn get(&self, key: &str) -> Result<Option<Vec<u8>>, String>;
    fn insert(&self, key: &str, value: &[u8]) -> Result<(), String>;
    fn new() -> Self
    where
        Self: Sized;
    fn config(&self, config: HashMap<String, String>) -> Self
    where
        Self: Sized;
    fn open(_path: &str) -> Self
    where
        Self: Sized,
    {
        panic!("DEPRECATED: use new() and config() instead of open()");
    }
}

#[derive(Clone)]
pub struct SledDataStorage {
    db: Option<sled::Db>,
}

#[derive(Clone)]
pub struct SledDataStorageConfig {
    pub path: String,
}

impl SledDataStorageConfig {
    pub fn build() -> SledDataStorageConfigBuilder {
        SledDataStorageConfigBuilder { path: None }
    }
}

pub struct SledDataStorageConfigBuilder {
    path: Option<String>,
}

impl SledDataStorageConfigBuilder {
    pub fn path(mut self, path: String) -> Self {
        self.path = Some(path);
        self
    }

    pub fn finalize(&self) -> Result<HashMap<String, String>, String> {
        let mut config = HashMap::new();

        match &self.path {
            Some(path) => config.insert("path".to_string(), path.to_string()),
            None => return Err("path is required".to_string()),
        };

        Ok(config)
    }

    pub fn unwrap(&self) -> HashMap<String, String> {
        self.finalize().unwrap()
    }
}

impl DataStorage for SledDataStorage {
    fn new() -> Self {
        Self { db: None }
    }

    fn config(&self, config: HashMap<String, String>) -> Self {
        if let Some(path) = config.get("path") {
            if let Ok(db) = sled::open(path) {
                return Self { db: Some(db) };
            }
        }
        self.clone()
    }

    fn get(&self, key: &str) -> Result<Option<Vec<u8>>, String> {
        if let Some(ref db) = self.db {
            match db.get(key.as_bytes()).unwrap() {
                Some(value) => Ok(Some(value.to_vec())),
                None => Ok(None),
            }
        } else {
            Err("Data Storage must be opened first".to_string())
        }
    }

    fn insert(&self, key: &str, value: &[u8]) -> Result<(), String> {
        if let Some(ref db) = self.db {
            match db.insert(key.as_bytes(), value) {
                Ok(_) => Ok(()),
                Err(e) => Err(e.to_string()),
            }
        } else {
            Err("Data Storage must be opened first".to_string())
        }
    }
}
