use serde::Serialize;
use std::rc::Rc;

#[derive(Debug)]
pub struct OCABundleReadModel {
    pub name: String,
    pub description: String,
    pub language_code: String,
    pub oca_bundle_said: String,
}

impl OCABundleReadModel {
    pub fn new(
        oca_bundle_said: String,
        name: String,
        description: String,
        language: isolang::Language,
    ) -> Self {
        Self {
            name,
            description,
            language_code: isolang::Language::to_639_3(&language).to_string(),
            oca_bundle_said,
        }
    }
}

pub struct OCABundleReadModelRepo {
    connection: Rc<rusqlite::Connection>,
}

impl OCABundleReadModelRepo {
    pub fn new(connection: Rc<rusqlite::Connection>) -> Self {
        let create_table_query = r#"
        CREATE VIRTUAL TABLE IF NOT EXISTS oca_bundle_read_model
        USING FTS5(
            name,
            description,
            language_code UNINDEXED,
            oca_bundle_said UNINDEXED,
            tokenize="trigram"
        )"#;
        connection.execute(create_table_query, ()).unwrap();

        Self {
            connection: Rc::clone(&connection),
        }
    }

    pub fn insert(&self, model: OCABundleReadModel) {
        let query = r#"
        INSERT INTO oca_bundle_read_model
        (rowid, name, description, language_code, oca_bundle_said)
        VALUES (
            (
                SELECT rowid FROM oca_bundle_read_model
                WHERE oca_bundle_said = ?4 AND language_code = ?3
                LIMIT 1
            ), ?1, ?2, ?3, ?4
        )"#;
        let _ = self.connection.execute(
            query,
            [
                &model.name,
                &model.description,
                &model.language_code,
                &model.oca_bundle_said,
            ],
        );
    }

    pub fn find_all(&self) -> Vec<OCABundleReadModel> {
        let mut results = vec![];
        let query = "SELECT * FROM oca_bundle_read_model";
        let mut statement = self.connection.prepare(query).unwrap();
        let models = statement
            .query_map((), |row| {
                Ok(OCABundleReadModel {
                    name: row.get(0).unwrap(),
                    description: row.get(1).unwrap(),
                    language_code: row.get(2).unwrap(),
                    oca_bundle_said: row.get(3).unwrap(),
                })
            })
            .unwrap();
        models.for_each(|model| results.push(model.unwrap()));
        results
    }

    pub fn search(
        &self,
        query: String,
        limit: usize,
        page: usize,
    ) -> SearchResult {
        let offset = (page - 1) * limit;
        let sql_query = r#"
        SELECT results.*, count.total
        FROM
        (
            SELECT COUNT(*) OVER() AS total
            FROM (
                SELECT *, rank
                FROM oca_bundle_read_model
                WHERE oca_bundle_read_model MATCH ?1
            ) AS inner_query
            GROUP BY oca_bundle_said
        ) AS count
        LEFT JOIN
        (
            SELECT *, COUNT(*) OVER()
            FROM (
                SELECT *,
                    highlight(oca_bundle_read_model, 0, '<mark>', '</mark>'),
                    highlight(oca_bundle_read_model, 1, '<mark>', '</mark>'),
                    rank
                FROM oca_bundle_read_model
                WHERE oca_bundle_read_model MATCH ?1
                ORDER BY rank
            ) AS subquery
            GROUP BY oca_bundle_said
            ORDER BY rank
            LIMIT ?2 OFFSET ?3
        ) AS results
        ON true
        GROUP BY oca_bundle_said
        ORDER BY rank
        "#;

        struct Record {
            pub name: Option<String>,
            pub description: Option<String>,
            pub language_code: Option<String>,
            pub oca_bundle_said: Option<String>,
            pub highlights_name: Option<String>,
            pub highlights_description: Option<String>,
            pub rank: Option<f32>,
            pub total: i32,
        }

        let mut statement = self.connection.prepare(sql_query).unwrap();

        let rows = statement
            .query_map(
                [query.clone(), limit.to_string(), offset.to_string()],
                |row| {
                    Ok(Record {
                        name: row.get(0).unwrap(),
                        description: row.get(1).unwrap(),
                        language_code: row.get(2).unwrap(),
                        oca_bundle_said: row.get(3).unwrap(),
                        highlights_name: row.get(4).unwrap(),
                        highlights_description: row.get(5).unwrap(),
                        rank: row.get(6).unwrap(),
                        total: row.get(8).unwrap(),
                    })
                },
            )
            .unwrap();

        let mut records = vec![];
        let mut total: usize = 0;

        for row in rows {
            let record = row.unwrap();
            if total == 0 {
                total = record.total as usize;
            }
            if record.oca_bundle_said.is_none() {
                continue;
            }
            records.push(SearchRecord {
                oca_bundle_said: record.oca_bundle_said.unwrap(),
                name: record.name.unwrap(),
                description: record.description.unwrap(),
                language_code: record.language_code.unwrap(),
                highlights: SearchRecordHighlights {
                    name: record.highlights_name.unwrap(),
                    description: record.highlights_description.unwrap(),
                },
                score: record.rank.unwrap().abs(),
            });
        }

        SearchResult {
            records,
            metadata: SearchMetadata {
                total,
                page,
            },
        }
    }
}

#[derive(Debug, Serialize)]
pub struct SearchRecordHighlights {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Serialize)]
pub struct SearchResult {
    #[serde(rename = "r")]
    pub records: Vec<SearchRecord>,
    #[serde(rename = "m")]
    pub metadata: SearchMetadata,
}

#[derive(Debug, Serialize)]
pub struct SearchRecord {
    pub oca_bundle_said: String,
    pub name: String,
    pub description: String,
    pub language_code: String,
    pub highlights: SearchRecordHighlights,
    pub score: f32,
}

#[derive(Debug, Serialize)]
pub struct SearchMetadata {
    pub total: usize,
    pub page: usize,
}

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
    path: Option<String>,
}

impl SQLiteConfigBuilder {
    pub fn path(mut self, path: String) -> Self {
        self.path = Some(path);
        self
    }

    pub fn finalize(&self) -> Result<SQLiteConfig, String> {
        let mut config = SQLiteConfig {
            path: "".to_string(),
        };

        match &self.path {
            Some(path) => config.path = path.to_string(),
            None => return Err("path is required".to_string()),
        };

        Ok(config)
    }

    pub fn unwrap(&self) -> SQLiteConfig {
        self.finalize().unwrap()
    }
}
