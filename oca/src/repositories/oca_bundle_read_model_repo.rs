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

    pub fn search(&self, query: String) -> Vec<SearchResult> {
        let sql_query = r#"
        SELECT *,
            highlight(oca_bundle_read_model, 0, '<mark>', '</mark>'),
            highlight(oca_bundle_read_model, 1, '<mark>', '</mark>'),
            rank
        FROM oca_bundle_read_model
        WHERE oca_bundle_read_model MATCH ?1
        ORDER BY rank"#;
        let mut statement = self.connection.prepare(sql_query).unwrap();
        let mut rows = statement.query([query.clone()]).unwrap();
        let mut results = vec![];
        while let Ok(Some(row)) = rows.next() {
            results.push(SearchResult {
                oca_bundle_said: row.get(3).unwrap(),
                name: row.get(0).unwrap(),
                description: row.get(1).unwrap(),
                language_code: row.get(2).unwrap(),
                highlights: SearchResultHighlights {
                    name: row.get(4).unwrap(),
                    description: row.get(5).unwrap(),
                },
                score: row.get(6).unwrap(),
            });
        }
        results
    }
}

#[derive(Debug, Serialize)]
pub struct SearchResultHighlights {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Serialize)]
pub struct SearchResult {
    pub oca_bundle_said: String,
    pub name: String,
    pub description: String,
    pub language_code: String,
    pub highlights: SearchResultHighlights,
    pub score: f64,
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
