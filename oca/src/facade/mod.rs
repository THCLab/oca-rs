use crate::data_storage::DataStorage;
use crate::repositories::SQLiteConfig;
use std::rc::Rc;

mod build;
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
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::data_storage::InMemoryDataStorage;

    #[test]
    fn facade_build_from_ocafile_from_base() -> Result<(), Vec<String>> {
        let db = InMemoryDataStorage::new();
        let db_cache = InMemoryDataStorage::new();
        let cache_storage_config = SQLiteConfig::build().unwrap();
        let ocafile = r#"
ADD ATTRIBUTE d=Text i=Text passed=Boolean
ADD META en PROPS name="Entrance credential" description="Entrance credential"
ADD CHARACTER_ENCODING ATTRS d=utf-8 i=utf-8 passed=utf-8
ADD CONFORMANCE ATTRS d=M i=M passed=M
ADD LABEL en ATTRS d="Schema digest" i="Credential Issuee" passed="Passed"
ADD INFORMATION en ATTRS d="Schema digest" i="Credential Issuee" passed="Enables or disables passing"
"#.to_string();
        let mut facade =
            Facade::new(Box::new(db), Box::new(db_cache), cache_storage_config);

        let result = facade.build_from_ocafile(ocafile)?;

        assert_eq!(
            result.said.unwrap().to_string(),
            "EF5ERATRBBN_ewEo9buQbznirhBmvrSSC0O2GIR4Gbfs"
        );
        Ok(())
    }
}
