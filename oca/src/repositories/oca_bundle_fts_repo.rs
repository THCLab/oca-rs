use std::str::FromStr;

use said::SelfAddressingIdentifier;

use crate::facade::Connection;

#[derive(Debug)]
pub struct OCABundleFTSRecord {
    pub name: String,
    pub description: String,
    pub language_code: String,
    pub oca_bundle_said: String,
}

impl OCABundleFTSRecord {
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

pub struct OCABundleFTSRepo {
    connection: Connection,
}

impl OCABundleFTSRepo {
    pub fn new(connection: Connection) -> Self {
        let create_table_query = r#"
        CREATE VIRTUAL TABLE IF NOT EXISTS oca_bundle_fts
        USING FTS5(
            name,
            description,
            language_code,
            oca_bundle_said UNINDEXED,
            tokenize="trigram"
        )"#;
        connection.execute(create_table_query, ()).unwrap();

        Self { connection }
    }

    pub fn insert(&self, model: OCABundleFTSRecord) {
        let query = r#"
        INSERT INTO oca_bundle_fts
        (rowid, name, description, language_code, oca_bundle_said)
        VALUES (
            (
                SELECT rowid FROM oca_bundle_fts
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

    pub fn search(
        &self,
        language: Option<isolang::Language>,
        meta_query: String,
        limit: usize,
        page: usize,
    ) -> SearchResult {
        let offset = (page - 1) * limit;
        let query = match language {
            Some(lang) => {
                let lang_code = isolang::Language::to_639_3(&lang).to_string();
                format!("({{name description}}:{meta_query:} AND language_code:{lang_code:}) OR ({{name description}}:{meta_query:} NOT language_code:{lang_code:})")
            }
            None => format!("{{name description}}:{meta_query:}"),
        };

        let sql_query = r#"
        SELECT results.*, count.total
        FROM
        (
            SELECT COUNT(*) OVER() AS total
            FROM (
                SELECT *
                FROM oca_bundle_fts
                WHERE oca_bundle_fts MATCH ?1
            ) AS inner_query
            GROUP BY oca_bundle_said
        ) AS count
        LEFT JOIN
        (
            SELECT *, COUNT(*) OVER()
            FROM (
                SELECT *,
                    bm25(oca_bundle_fts, 1.0, 1.0, 100.0) as rank,
                    snippet(oca_bundle_fts, -1, '<mark>', '</mark>', '...', 64)
                FROM oca_bundle_fts
                WHERE oca_bundle_fts MATCH ?1
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
            pub oca_bundle_said: Option<String>,
            pub rank: Option<f32>,
            pub total: i32,
            pub snippet: Option<String>,
        }

        impl Record {
            fn get_scope(&self) -> String {
                let mut snippet_regex = self.snippet.clone().unwrap();
                snippet_regex = snippet_regex.replace("<mark>", "");
                snippet_regex = snippet_regex.replace("</mark>", "");
                let mut v: Vec<String> =
                    snippet_regex.split("...").map(|x| x.to_string()).collect();
                if v.first().unwrap().is_empty() {
                    v.remove(0);
                    if let Some(x) = v.first_mut() {
                        *x = format!(".*{}", x);
                    }
                }
                if v.last().unwrap().is_empty() {
                    v.pop();
                    if let Some(x) = v.last_mut() {
                        *x = format!("{}.*", x);
                    }
                }
                snippet_regex = v.join("...");
                let re = regex::Regex::new(&format!("(?m)^([^:]+):{snippet_regex:}$")).unwrap();
                let hay = format!(
                    "\
meta_overlay:{}
meta_overlay:{}
",
                    self.name.clone().unwrap(),
                    self.description.clone().unwrap()
                );
                let mut scope = String::new();
                if let Some((_, [s])) = re.captures_iter(&hay).map(|c| c.extract()).next() {
                    scope = s.to_string();
                }
                scope
            }
        }

        let connection = self.connection.connection.lock().unwrap();
        let mut statement = connection.prepare(sql_query).unwrap();

        let rows = statement
            .query_map(
                [query.clone(), limit.to_string(), offset.to_string()],
                |row| {
                    Ok(Record {
                        name: row.get(0).unwrap(),
                        description: row.get(1).unwrap(),
                        oca_bundle_said: row.get(3).unwrap(),
                        rank: row.get(4).unwrap(),
                        total: row.get(7).unwrap(),
                        snippet: row.get(5).unwrap(),
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
            let metdata = SearchRecordMetadata {
                phrase: record.snippet.clone().unwrap(),
                scope: record.get_scope().clone(),
                score: record.rank.unwrap().abs(),
            };

            records.push(SearchRecord {
                oca_bundle_said: SelfAddressingIdentifier::from_str(
                    &record.oca_bundle_said.unwrap(),
                )
                .unwrap(), //TODO
                metadata: metdata,
            });
        }

        SearchResult {
            records,
            metadata: SearchMetadata { total, page },
        }
    }
}

#[derive(Debug)]
pub struct SearchResult {
    pub records: Vec<SearchRecord>,
    pub metadata: SearchMetadata,
}

#[derive(Debug)]
pub struct SearchRecord {
    pub oca_bundle_said: SelfAddressingIdentifier,
    pub metadata: SearchRecordMetadata,
}

#[derive(Debug)]
pub struct SearchRecordMetadata {
    pub phrase: String,
    pub scope: String,
    pub score: f32,
}

#[derive(Debug)]
pub struct SearchMetadata {
    pub total: usize,
    pub page: usize,
}
