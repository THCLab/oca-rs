use oca_bundle::state::oca::capture_base::CaptureBase;
use std::rc::Rc;

#[derive(Debug)]
pub struct CaptureBaseCacheRecord {
    pub said: String,
    pub capture_base: String,
}

impl CaptureBaseCacheRecord {
    pub fn new(capture_base: &CaptureBase) -> Self {
        Self {
            said: capture_base.said.clone().unwrap().to_string(),
            capture_base: serde_json::to_string(capture_base).unwrap(),
        }
    }
}

pub struct CaptureBaseCacheRepo {
    connection: Rc<rusqlite::Connection>,
}

impl CaptureBaseCacheRepo {
    pub fn new(connection: Rc<rusqlite::Connection>) -> Self {
        let create_table_query = r#"
        CREATE TABLE IF NOT EXISTS capture_base_cache(
            said TEXT PRIMARY KEY,
            capture_base TEXT
        )"#;
        connection.execute(create_table_query, ()).unwrap();

        Self {
            connection: Rc::clone(&connection),
        }
    }

    pub fn insert(&self, model: CaptureBaseCacheRecord) {
        let query = r#"
        INSERT INTO capture_base_cache(said, capture_base)
            VALUES (?1, ?2)"#;
        let _ = self
            .connection
            .execute(query, [&model.said, &model.capture_base]);
    }

    pub fn fetch_all(&self, limit: i32) -> Vec<CaptureBaseCacheRecord> {
        let mut results = vec![];
        let query = "
        SELECT *
            FROM capture_base_cache
            LIMIT ?1";
        let mut statement = self.connection.prepare(query).unwrap();
        let models = statement
            .query_map([limit], |row| {
                Ok(CaptureBaseCacheRecord {
                    said: row.get(0).unwrap(),
                    capture_base: row.get(1).unwrap(),
                })
            })
            .unwrap();
        models.for_each(|model| results.push(model.unwrap()));
        results
    }
}
