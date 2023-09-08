pub mod capture_base_cache_repo;
pub use capture_base_cache_repo::*;
pub mod oca_bundle_fts_repo;
pub use oca_bundle_fts_repo::*;
pub mod oca_bundle_cache_repo;
pub use oca_bundle_cache_repo::*;

use std::path::PathBuf;

#[derive(Clone)]
pub struct SQLiteConfig {
    pub path: String,
}

impl SQLiteConfig {
    pub fn build() -> SQLiteConfigBuilder {
        SQLiteConfigBuilder { path: None }
    }
}

pub struct SQLiteConfigBuilder {
    path: Option<PathBuf>,
}

impl SQLiteConfigBuilder {
    pub fn path(mut self, path: PathBuf) -> Self {
        self.path = Some(path.join("search_data.db"));
        self
    }

    pub fn finalize(&self) -> Result<SQLiteConfig, String> {
        let mut config = SQLiteConfig {
            path: "".to_string(),
        };

        config.path = match &self.path {
            Some(path) => path
                .clone()
                .into_os_string()
                .into_string()
                .map_err(|e| e.into_string().unwrap())?,
            None => ":memory:".to_string(),
        };

        Ok(config)
    }

    pub fn unwrap(&self) -> SQLiteConfig {
        self.finalize().unwrap()
    }
}
