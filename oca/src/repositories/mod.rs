pub mod capture_base_cache_repo;
pub use capture_base_cache_repo::*;
pub mod oca_bundle_fts_repo;
pub use oca_bundle_fts_repo::*;
pub mod oca_bundle_cache_repo;
pub use oca_bundle_cache_repo::*;

use std::path::PathBuf;

#[derive(Clone)]
pub struct SQLiteConfig {
    pub path: Option<PathBuf>,
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
        self.path = Some(path);
        self
    }

    pub fn finalize(&self) -> Result<SQLiteConfig, String> {
        Ok(SQLiteConfig {
            path: self.path.clone(),
        })
    }

    pub fn unwrap(&self) -> SQLiteConfig {
        self.finalize().unwrap()
    }
}
