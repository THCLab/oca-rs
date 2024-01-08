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
            oca_bundle: String::from_utf8(oca_bundle.encode().unwrap()).unwrap(),
        }
    }
}

#[derive(Debug)]
pub struct AllOCABundleRecord {
    pub cache_record: Option<OCABundleCacheRecord>,
    pub total: usize,
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

    pub fn fetch_all(&self, limit: usize, page: usize) -> Vec<AllOCABundleRecord> {
        let offset = (page - 1) * limit;
        let mut results = vec![];
        let query = "
        SELECT results.*, count.total
        FROM
        (
            SELECT COUNT(*) OVER() AS total
            FROM oca_bundle_cache
        ) AS count
        LEFT JOIN
        (
            SELECT *
            FROM oca_bundle_cache
            LIMIT ?1 OFFSET ?2
        ) AS results
        ON true
        GROUP BY said";
        let mut statement = self.connection.prepare(query).unwrap();
        let models = statement
            .query_map([limit, offset], |row| {
                let cache_record =
                    row.get::<_, Option<String>>(0)
                        .unwrap()
                        .map(|said| OCABundleCacheRecord {
                            said,
                            oca_bundle: row.get(1).unwrap(),
                        });
                Ok(AllOCABundleRecord {
                    cache_record,
                    total: row.get(2).unwrap(),
                })
            })
            .unwrap();
        models.for_each(|model| results.push(model.unwrap()));
        results
    }
}
