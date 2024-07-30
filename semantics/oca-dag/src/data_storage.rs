use dyn_clonable::*;

#[clonable]
pub trait DataStorage: Clone {
    fn get(&self, key: &str) -> Result<Option<Vec<u8>>, String>;
    fn insert(&self, key: &str, value: &[u8]) -> Result<(), String>;
    fn open(path: &str) -> Self
    where
        Self: Sized;
}

#[derive(Clone)]
pub struct SledDataStorage {
    db: Option<sled::Db>,
}

impl DataStorage for SledDataStorage {
    fn open(path: &str) -> Self {
        match sled::open(path) {
            Ok(db) => Self { db: Some(db) },
            Err(_e) => Self { db: None },
        }
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
