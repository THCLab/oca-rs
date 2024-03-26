use dyn_clonable::*;
use std::{collections::HashMap, path::PathBuf};

pub enum Namespace {
    OCA,
    OCABundlesJSON,
    OCAObjectsJSON,
    CoreModel,
    OCARelations,
    OCAReferences,
}

impl Namespace {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::OCA => "oca",
            Self::OCABundlesJSON => "oca_bundles_json",
            Self::OCAObjectsJSON => "oca_objects_json",
            Self::CoreModel => "core_model",
            Self::OCARelations => "oca_relations",
            Self::OCAReferences => "oca_refs",
        }
    }
}

#[clonable]
pub trait DataStorage: Clone + Send {
    fn get(&self, namespace: Namespace, key: &str) -> Result<Option<Vec<u8>>, String>;
    fn get_all(&self, namespace: Namespace) -> Result<HashMap<String, Vec<u8>>, String>;
    fn insert(&mut self, namespace: Namespace, key: &str, value: &[u8]) -> Result<(), String>;
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
    path: Option<PathBuf>,
}

impl SledDataStorageConfigBuilder {
    pub fn path(mut self, path: PathBuf) -> Self {
        self.path = Some(path);
        self
    }

    pub fn finalize(&self) -> Result<HashMap<String, String>, String> {
        let mut config = HashMap::new();

        match &self.path {
            Some(path) => config.insert(
                "path".to_string(),
                path.clone()
                    .into_os_string()
                    .into_string()
                    .map_err(|e| e.into_string().unwrap())?,
            ),
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

    fn get(&self, namespace: Namespace, key: &str) -> Result<Option<Vec<u8>>, String> {
        if let Some(ref db) = self.db {
            let tree = db.open_tree(namespace.as_str().as_bytes()).unwrap();
            match tree.get(key.as_bytes()).unwrap() {
                Some(value) => Ok(Some(value.to_vec())),
                None => Ok(None),
            }
        } else {
            Err("Data Storage must be opened first".to_string())
        }
    }

    fn get_all(&self, namespace: Namespace) -> Result<HashMap<String, Vec<u8>>, String> {
        if let Some(ref db) = self.db {
            let mut all = HashMap::new();
            let tree = db.open_tree(namespace.as_str().as_bytes()).unwrap();
            let mut iter = tree.iter();
            while let Some(Ok((key, value))) = iter.next() {
                all.insert(String::from_utf8(key.to_vec()).unwrap(), value.to_vec());
            }

            Ok(all)
        } else {
            Err("Data Storage must be opened first".to_string())
        }
    }

    fn insert(&mut self, namespace: Namespace, key: &str, value: &[u8]) -> Result<(), String> {
        if let Some(ref db) = self.db {
            let tree = db.open_tree(namespace.as_str().as_bytes()).unwrap();
            match tree.insert(key.as_bytes(), value) {
                Ok(_) => Ok(()),
                Err(e) => Err(e.to_string()),
            }
        } else {
            Err("Data Storage must be opened first".to_string())
        }
    }
}

#[derive(Clone)]
pub struct InMemoryDataStorage {
    db: HashMap<String, HashMap<String, Vec<u8>>>,
}

impl DataStorage for InMemoryDataStorage {
    fn new() -> Self {
        Self { db: HashMap::new() }
    }

    fn config(&self, _config: HashMap<String, String>) -> Self {
        self.clone()
    }

    fn get(&self, namespace: Namespace, key: &str) -> Result<Option<Vec<u8>>, String> {
        let namespace_storage = match self.db.get(namespace.as_str()) {
            Some(namespace_storage) => namespace_storage,
            None => return Ok(None),
        };
        match namespace_storage.get(key) {
            Some(value) => Ok(Some(value.to_vec())),
            None => Ok(None),
        }
    }

    fn get_all(&self, namespace: Namespace) -> Result<HashMap<String, Vec<u8>>, String> {
        match self.db.get(namespace.as_str()) {
            Some(namespace_storage) => Ok(namespace_storage.clone()),
            None => Ok(HashMap::new()),
        }
    }

    fn insert(&mut self, namespace: Namespace, key: &str, value: &[u8]) -> Result<(), String> {
        let mut namespace_storage = match self.db.get(namespace.as_str()) {
            Some(namespace_storage) => namespace_storage.clone(),
            None => HashMap::new(),
        };
        namespace_storage.insert(key.to_string(), value.to_vec());
        self.db
            .insert(namespace.as_str().to_string(), namespace_storage);

        Ok(())
    }
}

#[derive(Clone)]
pub struct FileSystemStorage {
    dir: Option<PathBuf>,
}

#[derive(Clone)]
pub struct FileSystemStorageConfig {
    pub path: String,
}

impl FileSystemStorageConfig {
    pub fn build() -> FileSystemStorageConfigBuilder {
        FileSystemStorageConfigBuilder { path: None }
    }
}

pub struct FileSystemStorageConfigBuilder {
    path: Option<PathBuf>,
}

impl FileSystemStorageConfigBuilder {
    pub fn path(mut self, path: PathBuf) -> Self {
        self.path = Some(path);
        self
    }

    pub fn finalize(&self) -> Result<HashMap<String, String>, String> {
        let mut config = HashMap::new();

        match &self.path {
            Some(path) => config.insert(
                "path".to_string(),
                path.clone()
                    .into_os_string()
                    .into_string()
                    .map_err(|e| e.into_string().unwrap())?,
            ),
            None => return Err("path is required".to_string()),
        };

        Ok(config)
    }

    pub fn unwrap(&self) -> HashMap<String, String> {
        self.finalize().unwrap()
    }
}

impl DataStorage for FileSystemStorage {
    fn new() -> Self {
        Self { dir: None }
    }

    fn config(&self, config: HashMap<String, String>) -> Self {
        if let Some(path) = config.get("path") {
            return Self {
                dir: Some(PathBuf::from(path)),
            };
        }
        self.clone()
    }

    fn get(&self, namespace: Namespace, key: &str) -> Result<Option<Vec<u8>>, String> {
        if let Some(ref dir) = self.dir {
            let mut path = dir.clone();
            path.push(namespace.as_str());
            if path.try_exists().unwrap() {
                path.push(key);
                Ok(std::fs::read(path.clone()).ok())
            } else {
                Ok(None)
            }
        } else {
            Err("File path is required".to_string())
        }
    }

    fn get_all(&self, _namespace: Namespace) -> Result<HashMap<String, Vec<u8>>, String> {
        Err("Not implemented".to_string())
    }

    fn insert(&mut self, namespace: Namespace, key: &str, value: &[u8]) -> Result<(), String> {
        if let Some(ref dir) = self.dir {
            let mut path = dir.clone();
            path.push(namespace.as_str());
            if !path.try_exists().unwrap() {
                std::fs::create_dir_all(path.clone()).unwrap();
            }

            path.push(key);
            std::fs::write(path.clone(), value).map_err(|e| e.to_string())
        } else {
            Err("File path is required".to_string())
        }
    }
}
