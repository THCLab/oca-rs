use oca_bundle::{state::oca::OCABundle, Encode};
use std::rc::Rc;

#[derive(Debug)]
pub struct OCABundleCacheRecord {
    pub said: String,
    pub oca_bundle: String,
}

impl OCABundleCacheRecord {
    pub fn new(oca_bundle: &OCABundle) -> Self {
        Self {
            said: oca_bundle.said.clone().unwrap().to_string(),
            oca_bundle: String::from_utf8(oca_bundle.encode().unwrap())
                .unwrap(),
        }
    }
}

pub struct OCABundleCacheRepo {
    connection: Rc<rusqlite::Connection>,
}

impl OCABundleCacheRepo {
    pub fn new(connection: Rc<rusqlite::Connection>) -> Self {
        let create_table_query = r#"
        CREATE TABLE IF NOT EXISTS oca_bundle_cache(
            said TEXT PRIMARY KEY,
            oca_bundle TEXT
        )"#;
        connection.execute(create_table_query, ()).unwrap();

        Self {
            connection: Rc::clone(&connection),
        }
    }

    pub fn insert(&self, model: OCABundleCacheRecord) {
        let query = r#"
        INSERT INTO oca_bundle_cache(said, oca_bundle)
            VALUES (?1, ?2)"#;
        let _ = self
            .connection
            .execute(query, [&model.said, &model.oca_bundle]);
    }

    pub fn fetch_all(&self, limit: i32) -> Vec<OCABundleCacheRecord> {
        let mut results = vec![];
        let query = "
        SELECT *
            FROM oca_bundle_cache
            LIMIT ?1";
        let mut statement = self.connection.prepare(query).unwrap();
        let models = statement
            .query_map([limit], |row| {
                Ok(OCABundleCacheRecord {
                    said: row.get(0).unwrap(),
                    oca_bundle: row.get(1).unwrap(),
                })
            })
            .unwrap();
        models.for_each(|model| results.push(model.unwrap()));
        results
    }
}
