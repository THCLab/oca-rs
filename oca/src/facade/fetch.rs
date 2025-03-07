use super::Facade;
use crate::data_storage::Namespace;
#[cfg(feature = "local-references")]
use crate::local_references;
use crate::{
    data_storage::DataStorage,
    repositories::{OCABundleCacheRepo, OCABundleFTSRepo},
};
use oca_ast_semantics::ast::{self, OCAAst, ObjectKind, RefValue};
use oca_bundle_semantics::build::OCABuildStep;
use oca_bundle_semantics::state::oca::{capture_base::CaptureBase, DynOverlay, OCABundle};
use said::{
    derivation::HashFunctionCode,
    sad::{SerializationFormats, SAD},
    version::SerializationInfo,
    SelfAddressingIdentifier,
};

use serde::Serialize;
use std::borrow::Borrow;
#[cfg(feature = "local-references")]
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum OCAObject {
    CaptureBase(CaptureBase),
    Overlay(DynOverlay),
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
    pub oca_bundle: OCABundle,
    pub metadata: SearchRecordMetadata,
}

#[derive(Debug, Serialize)]
pub struct SearchRecordMetadata {
    pub phrase: String,
    pub scope: String,
    pub score: f32,
}

#[derive(Debug, Serialize)]
pub struct SearchMetadata {
    pub total: usize,
    pub page: usize,
}

#[derive(Debug, Serialize)]
pub struct AllOCABundleResult {
    pub records: Vec<OCABundle>,
    pub metadata: AllOCABundleMetadata,
}

#[derive(Debug, Serialize)]
pub struct AllOCABundleMetadata {
    pub total: usize,
    pub page: usize,
}

#[derive(Debug, Serialize)]
pub struct AllCaptureBaseResult {
    pub records: Vec<CaptureBase>,
    pub metadata: AllCaptureBaseMetadata,
}

#[derive(SAD, Debug, Serialize)]
#[version(protocol = "OCAA", major = 1, minor = 1)]
pub struct BundleWithDependencies {
    pub bundle: OCABundle,
    pub dependencies: Vec<OCABundle>,
}

#[derive(Debug, Serialize)]
pub struct AllCaptureBaseMetadata {
    pub total: usize,
    pub page: usize,
}

impl Facade {
    pub fn search_oca_bundle(
        &self,
        language: Option<isolang::Language>,
        query: String,
        limit: usize,
        page: usize,
    ) -> SearchResult {
        let oca_bundle_fts_repo = OCABundleFTSRepo::new(self.connection());
        let search_result = oca_bundle_fts_repo.search(language, query, limit, page);
        let records = search_result
            .records
            .iter()
            .map(|record| SearchRecord {
                // TODO
                oca_bundle: self
                    .get_oca_bundle(record.oca_bundle_said.clone(), false)
                    .unwrap()
                    .bundle
                    .clone(),
                metadata: SearchRecordMetadata {
                    phrase: record.metadata.phrase.clone(),
                    scope: record.metadata.scope.clone(),
                    score: record.metadata.score,
                },
            })
            .collect();
        SearchResult {
            records,
            metadata: SearchMetadata {
                total: search_result.metadata.total,
                page: search_result.metadata.page,
            },
        }
    }
    #[cfg(feature = "local-references")]
    pub fn fetch_all_refs(&self) -> Result<HashMap<String, String>, String> {
        let mut refs: HashMap<String, String> = HashMap::new();
        self.db
            .get_all(Namespace::OCAReferences)
            .unwrap()
            .iter()
            .for_each(|(k, v)| {
                refs.insert(k.clone(), String::from_utf8(v.to_vec()).unwrap());
            });
        Ok(refs)
    }

    pub fn fetch_all_oca_bundle(
        &self,
        limit: usize,
        page: usize,
    ) -> Result<AllOCABundleResult, Vec<String>> {
        let mut oca_bundles = vec![];
        let mut total: usize = 0;
        let mut errors = vec![];

        let oca_bundle_cache_repo = OCABundleCacheRepo::new(self.connection());
        let all_oca_bundle_records = oca_bundle_cache_repo.fetch_all(limit, page);
        for all_oca_bundle_record in all_oca_bundle_records {
            if total == 0 {
                total = all_oca_bundle_record.total;
            }
            if let Some(cache_record) = all_oca_bundle_record.cache_record {
                match serde_json::from_str(&cache_record.oca_bundle) {
                    Ok(oca_bundle) => {
                        oca_bundles.push(oca_bundle);
                    }
                    Err(e) => {
                        errors.push(format!("Failed to parse oca bundle: {}", e));
                    }
                }
            }
        }
        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(AllOCABundleResult {
            records: oca_bundles,
            metadata: AllOCABundleMetadata { total, page },
        })
    }

    pub fn fetch_all_capture_base(
        &self,
        limit: usize,
        page: usize,
    ) -> Result<AllCaptureBaseResult, Vec<String>> {
        let mut capture_bases = vec![];
        let mut total: usize = 0;
        let mut errors = vec![];

        let capture_base_cache_repo =
            crate::repositories::CaptureBaseCacheRepo::new(self.connection());
        let all_capture_base_records = capture_base_cache_repo.fetch_all(limit, page);
        for all_capture_base_record in all_capture_base_records {
            if total == 0 {
                total = all_capture_base_record.total;
            }
            if let Some(cache_record) = all_capture_base_record.cache_record {
                match serde_json::from_str(&cache_record.capture_base) {
                    Ok(capture_base) => {
                        capture_bases.push(capture_base);
                    }
                    Err(e) => {
                        errors.push(format!("Failed to parse capture base: {}", e));
                    }
                }
            }
        }
        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(AllCaptureBaseResult {
            records: capture_bases,
            metadata: AllCaptureBaseMetadata { total, page },
        })
    }

    pub fn get_oca_objects(&self, saids: Vec<String>) -> Result<Vec<OCAObject>, Vec<String>> {
        let mut result: Vec<OCAObject> = vec![];
        let mut errors: Vec<String> = vec![];

        for said in saids {
            let r = self
                .db_cache
                .get(Namespace::OCAObjectsJSON, &said)
                .map_err(|e| {
                    errors.push(e.to_string());
                    errors.clone()
                })?;
            let object_str = String::from_utf8(r.ok_or_else(|| {
                errors.push(format!("No OCA Object found for said: {}", said));
                errors.clone()
            })?)
            .unwrap();
            let r_type = self
                .db
                .get(Namespace::OCARelations, &format!("{}.metadata", said))
                .map_err(|e| {
                    errors.push(e.to_string());
                    errors.clone()
                })?;
            let o_type: ObjectKind = (*r_type.unwrap().first().unwrap()).into();
            match o_type {
                ObjectKind::CaptureBase(_) => result.push(OCAObject::CaptureBase(
                    serde_json::from_str::<CaptureBase>(&object_str).map_err(|e| {
                        errors.push(format!("Failed to parse OCA object: {}", e));
                        errors.clone()
                    })?,
                )),
                ObjectKind::Overlay(_, _) => {
                    let oca_overlay = OCAObject::Overlay(
                        serde_json::from_str::<DynOverlay>(&object_str).map_err(|e| {
                            errors.push(format!("Failed to parse OCA object: {}", e));
                            errors.clone()
                        })?,
                    );
                    result.push(oca_overlay);
                }
                _ => {}
            };
        }

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(result)
    }

    pub fn get_oca_bundle(
        &self,
        said: SelfAddressingIdentifier,
        with_dep: bool,
    ) -> Result<BundleWithDependencies, Vec<String>> {
        get_oca_bundle(self.db_cache.borrow(), said, with_dep)
    }

    pub fn get_oca_bundle_steps(
        &self,
        said: SelfAddressingIdentifier,
    ) -> Result<Vec<OCABuildStep>, Vec<String>> {
        let mut said = said.to_string();
        #[allow(clippy::borrowed_box)]
        fn extract_operation(
            db: &Box<dyn DataStorage>,
            said: &String,
        ) -> Result<(String, oca_ast_semantics::ast::Command), Vec<String>> {
            let r = db
                .get(Namespace::OCA, &format!("oca.{}.operation", said))
                .map_err(|e| vec![format!("{}", e)])?
                .ok_or_else(|| vec![format!("No history found for said: {}", said)])?;

            let said_length = r.first().unwrap();
            let parent_said = String::from_utf8_lossy(&r[1..*said_length as usize + 1]).to_string();
            let op_length = r.len() - *said_length as usize - 1;
            let op = String::from_utf8_lossy(
                &r[*said_length as usize + 1..*said_length as usize + 1 + op_length],
            )
            .to_string();

            match serde_json::from_str::<oca_ast_semantics::ast::Command>(&op) {
                Ok(command) => Ok((parent_said, command)),
                Err(e) => Err(vec![format!("Failed to parse command: {}", e)]),
            }
        }

        let mut history: Vec<OCABuildStep> = vec![];

        loop {
            let (parent_said, command) = extract_operation(&self.db, &said)?;
            if parent_said == said {
                dbg!("Malformed history for said: {}", said);
                return Err(vec![format!("Malformed history")]);
            }
            let s = SelfAddressingIdentifier::from_str(&said).unwrap(); // TODO
            history.push(OCABuildStep {
                parent_said: parent_said.clone().parse().ok(),
                command,
                result: self.get_oca_bundle(s, false).unwrap().bundle.clone(),
            });
            said = parent_said;

            if said.is_empty() {
                break;
            }
        }
        history.reverse();
        Ok(history)
    }

    /// Retrive the ocafile for a given said
    /// If dereference is true, all local references will be dereferenced to SAID
    pub fn get_oca_bundle_ocafile(
        &self,
        said: SelfAddressingIdentifier,
        dereference: bool,
    ) -> Result<String, Vec<String>> {
        let oca_bundle_steps = self.get_oca_bundle_steps(said)?;
        let mut oca_ast = OCAAst::new();
        for step in oca_bundle_steps {
            oca_ast.commands.push(step.command);
        }

        if dereference {
            #[cfg(feature = "local-references")]
            local_references::replace_refn_with_refs(&mut oca_ast, &self.db)
                .map_err(|e| vec![e.to_string()])?;
        }

        Ok(oca_file_semantics::ocafile::generate_from_ast(&oca_ast))
    }

    /// Retrive steps (AST representation) for a given said
    ///
    pub fn get_oca_bundle_ast(
        &self,
        said: SelfAddressingIdentifier,
    ) -> Result<OCAAst, Vec<String>> {
        let oca_bundle_steps = self.get_oca_bundle_steps(said)?;
        let mut oca_ast = OCAAst::new();
        for step in oca_bundle_steps {
            oca_ast.commands.push(step.command);
        }
        Ok(oca_ast)
    }

    pub fn parse_oca_bundle_to_ocafile(&self, bundle: &OCABundle) -> Result<String, Vec<String>> {
        let oca_ast = bundle.to_ast();
        Ok(oca_file_semantics::ocafile::generate_from_ast(&oca_ast))
    }
}

pub fn get_oca_bundle(
    storage: &dyn DataStorage,
    said: SelfAddressingIdentifier,
    with_dep: bool,
) -> Result<BundleWithDependencies, Vec<String>> {
    let r = storage
        .get(Namespace::OCABundlesJSON, &said.to_string())
        .map_err(|e| vec![format!("{}", e)])?;
    let oca_bundle_str = String::from_utf8(
        r.ok_or_else(|| vec![format!("No OCA Bundle found for said: {}", said)])?,
    )
    .unwrap();
    let oca_bundle: Result<OCABundle, Vec<String>> = serde_json::from_str(&oca_bundle_str)
        .map_err(|e| vec![format!("Failed to parse oca bundle: {}", e)]);

    match oca_bundle {
        Ok(oca_bundle) => {
            let mut dep_bundles = vec![];
            if with_dep {
                for refs in retrive_all_references(oca_bundle.clone()) {
                    let dep_bundle = get_oca_bundle(storage, refs, true)?;
                    dep_bundles.push(dep_bundle.bundle);
                    dep_bundles.extend(dep_bundle.dependencies);
                }
            }
            let result = BundleWithDependencies {
                bundle: oca_bundle,
                dependencies: dep_bundles,
            };
            Ok(result)
        }
        Err(e) => Err(e),
    }
}

/// Retrive all existing references from given OCA Bundle
///
/// # Arguments
/// * `said` - SAID of the OCA Bundle
///
/// # Return
/// * `Vec<String>` - Vector of all SAID references
fn retrive_all_references(bundle: OCABundle) -> Vec<SelfAddressingIdentifier> {
    let mut refs: Vec<SelfAddressingIdentifier> = vec![];

    for (_, value) in bundle.capture_base.attributes {
        match value {
            ast::NestedAttrType::Reference(RefValue::Said(said)) => {
                refs.push(said);
            }
            // TODO(recursion) handle nested arrays
            ast::NestedAttrType::Array(box_attr_type) => {
                if let ast::NestedAttrType::Reference(RefValue::Said(said)) = &*box_attr_type {
                    refs.push(said.clone());
                }
            }
            _ => {}
        }
    }
    refs
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::data_storage::InMemoryDataStorage;
    use crate::repositories::SQLiteConfig;

    #[test]
    fn facade_get_ocafile() -> Result<(), Vec<String>> {
        let db = InMemoryDataStorage::new();
        let db_cache = InMemoryDataStorage::new();
        let cache_storage_config = SQLiteConfig::build().unwrap();
        let mut facade = Facade::new(Box::new(db), Box::new(db_cache), cache_storage_config);
        let ocafile_input = r#"
ADD ATTRIBUTE d=Text i=Text passed=Boolean
ADD META en PROPS description="Entrance credential" name="Entrance credential"
ADD CHARACTER_ENCODING ATTRS d="utf-8" i="utf-8" passed="utf-8"
ADD CONFORMANCE ATTRS d="M" i="M" passed="M"
ADD LABEL en ATTRS d="Schema digest" i="Credential Issuee" passed="Passed"
ADD INFORMATION en ATTRS d="Schema digest" i="Credential Issuee" passed="Enables or disables passing"
ADD FORMAT ATTRS d="image/jpeg"
ADD UNIT ATTRS i=m
ADD ATTRIBUTE list=Array[Text] el=Text
ADD CARDINALITY ATTRS list="1-2"
ADD ENTRY_CODE ATTRS list="entry_code_said" el=["o1", "o2", "o3"]
ADD ENTRY en ATTRS list="refs:ENrf7niTCnz7HD-Ci88rlxHlxkpQ2NIZNNv08fQnXANI" el={"o1": "o1_label", "o2": "o2_label", "o3": "o3_label"}
"#.to_string();
        let oca_bundle = facade.build_from_ocafile(ocafile_input);
        let oca_bundle = oca_bundle.unwrap();
        let ocafile = facade.parse_oca_bundle_to_ocafile(&oca_bundle)?;
        let new_bundle = facade.build_from_ocafile(ocafile);
        match new_bundle {
            Ok(new_bundle) => {
                assert_eq!(oca_bundle.said, new_bundle.said);
            }
            Err(e) => {
                println!("{:#?}", e);
                panic!("Faild to load oca bundle");
            }
        }

        Ok(())
    }
}
