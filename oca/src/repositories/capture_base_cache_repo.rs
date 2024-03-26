use oca_bundle::state::oca::capture_base::CaptureBase;

use crate::facade::Connection;

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

#[derive(Debug)]
pub struct AllCaptureBaseRecord {
    pub cache_record: Option<CaptureBaseCacheRecord>,
    pub total: usize,
}

pub struct CaptureBaseCacheRepo {
    connection: Connection,
}

impl CaptureBaseCacheRepo {
    pub fn new(connection: Connection) -> Self {
        let create_table_query = r#"
        CREATE TABLE IF NOT EXISTS capture_base_cache(
            said TEXT PRIMARY KEY,
            capture_base TEXT
        )"#;
        connection.execute(create_table_query, ());

        Self {
            connection,
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

    pub fn fetch_all(&self, limit: usize, page: usize) -> Vec<AllCaptureBaseRecord> {
        let offset = (page - 1) * limit;
        let mut results = vec![];
        let query = "
        SELECT results.*, count.total
        FROM
        (
            SELECT COUNT(*) OVER() AS total
            FROM capture_base_cache
        ) AS count
        LEFT JOIN
        (
            SELECT *
            FROM capture_base_cache
            LIMIT ?1 OFFSET ?2
        ) AS results
        ON true
        GROUP BY said";
        
        let connection = self.connection.connection.lock().unwrap();
        let mut statement = connection.prepare(query).unwrap();

        let models = statement
            .query_map([limit, offset], |row| {
                let cache_record =
                    row.get::<_, Option<String>>(0)
                        .unwrap()
                        .map(|said| CaptureBaseCacheRecord {
                            said,
                            capture_base: row.get(1).unwrap(),
                        });
                Ok(AllCaptureBaseRecord {
                    cache_record,
                    total: row.get(2).unwrap(),
                })
            })
            .unwrap();
        models.for_each(|model| results.push(model.unwrap()));
        results
    }
}
