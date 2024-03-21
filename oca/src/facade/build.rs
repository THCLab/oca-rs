use super::Facade;
use crate::data_storage::{DataStorage, Namespace};
use crate::facade::fetch::get_oca_bundle;
#[cfg(feature = "local-references")]
use crate::local_references;
#[cfg(feature = "local-references")]
pub use crate::local_references::References;
use crate::repositories::{
    CaptureBaseCacheRecord, CaptureBaseCacheRepo, OCABundleCacheRecord, OCABundleCacheRepo,
    OCABundleFTSRecord, OCABundleFTSRepo,
};
#[cfg(feature = "local-references")]
use log::debug;
use oca_ast::ast::{OCAAst, ObjectKind, RefValue, ReferenceAttrType};
use oca_bundle::build::OCABuild;
use oca_bundle::state::oca::OCABundle;
use oca_bundle::Encode;
use oca_dag::build_core_db_model;

use std::borrow::Borrow;
use std::rc::Rc;

#[derive(thiserror::Error, Debug, serde::Serialize)]
#[serde(untagged)]
pub enum Error {
    #[error("Validation error")]
    ValidationError(Vec<ValidationError>),
}

#[derive(thiserror::Error, Debug, serde::Serialize)]
#[serde(untagged)]
pub enum ValidationError {
    #[error(transparent)]
    OCAFileParse(#[from] oca_file::ocafile::error::ParseError),
    #[error(transparent)]
    OCABundleBuild(#[from] oca_bundle::build::Error),
    #[error("Error at line {line_number} ({raw_line}): {message}")]
    InvalidCommand {
        #[serde(rename = "ln")]
        line_number: usize,
        #[serde(rename = "c")]
        raw_line: String,
        #[serde(rename = "e")]
        message: String,
    },
    #[cfg(feature = "local-references")]
    #[error("Reference {0} not found")]
    UnknownRefn(String),
}

#[cfg(feature = "local-references")]
impl References for Box<dyn DataStorage> {
    fn find(&self, refn: &str) -> Option<String> {
        self.get(Namespace::OCAReferences, refn)
            .unwrap()
            .map(|said| String::from_utf8(said).unwrap())
    }

    fn save(&mut self, refn: &str, value: String) {
        self.insert(Namespace::OCAReferences, refn, value.to_string().as_bytes())
            .unwrap()
    }
}

impl Facade {
    fn parse_and_check_base(
        storage: &dyn DataStorage,
        ocafile: String,
    ) -> Result<(Option<OCABundle>, OCAAst), Vec<ValidationError>> {
        let mut errors: Vec<ValidationError> = vec![];
        let mut oca_ast = oca_file::ocafile::parse_from_string(ocafile)
            .map_err(|e| vec![ValidationError::OCAFileParse(e)])?;

        if !errors.is_empty() {
            return Err(errors);
        }

        let mut base: Option<OCABundle> = None;
        // TODO this should be avoided if the ast is passed for further processing, the base is
        // checked again in generate bundle
        if let Some(first_command) = oca_ast.commands.first() {
            if let (oca_ast::ast::CommandType::From, ObjectKind::OCABundle(content)) = (
                first_command.clone().kind,
                first_command.clone().object_kind,
            ) {
                match content.said {
                    ReferenceAttrType::Reference(refs) => {
                        match refs {
                            RefValue::Said(said) => {
                                match get_oca_bundle(storage, said, false) {
                                    Ok(oca_bundle) => {
                                        // TODO
                                        base = Some(oca_bundle.bundle.clone());
                                    }
                                    Err(e) => {
                                        let default_command_meta = oca_ast::ast::CommandMeta {
                                            line_number: 0,
                                            raw_line: "unknown".to_string(),
                                        };
                                        let command_meta = oca_ast
                                            .commands_meta
                                            .get(&0)
                                            .unwrap_or(&default_command_meta);
                                        e.iter().for_each(|e| {
                                            errors.push(ValidationError::InvalidCommand {
                                                line_number: command_meta.line_number,
                                                raw_line: command_meta.raw_line.clone(),
                                                message: e.clone(),
                                            })
                                        });
                                    }
                                }
                            }
                            RefValue::Name(_) => todo!(),
                        }
                    }
                }
                oca_ast.commands.remove(0);
            }
        };
        Ok((base, oca_ast))
    }

    #[cfg(feature = "local-references")]
    fn oca_ast_to_oca_build_with_references<R: References>(
        base: Option<OCABundle>,
        mut oca_ast: OCAAst,
        references: &mut R,
    ) -> Result<OCABuild, Vec<ValidationError>> {
        // Dereference (refn -> refs) the AST before it start processing bundle steps, otherwise the SAID would
        // not match.
        local_references::replace_refn_with_refs(&mut oca_ast, references).map_err(|e| vec![e])?;

        let oca_build = oca_bundle::build::from_ast(base, &oca_ast).map_err(|e| {
            e.iter()
                .map(|e| ValidationError::OCABundleBuild(e.clone()))
                .collect::<Vec<_>>()
        })?;

        let schema_name = oca_ast.meta.get("name");
        debug!("Schema name found: {:?}", schema_name);

        if schema_name.is_some() {
            let schema_name = schema_name.unwrap();
            let said = oca_build.oca_bundle.said.clone().unwrap().to_string();
            references.save(schema_name, said.clone());
        };
        Ok(oca_build)
    }

    #[cfg(feature = "local-references")]
    pub fn validate_ocafile<R: References>(
        storage: &dyn DataStorage,
        ocafile: String,
        references: &mut R,
    ) -> Result<OCABuild, Vec<ValidationError>> {
        let (base, oca_ast) = Self::parse_and_check_base(storage, ocafile)?;
        Self::oca_ast_to_oca_build_with_references(base, oca_ast, references)
    }

    #[cfg(not(feature = "local-references"))]
    pub fn validate_ocafile(
        storage: &dyn DataStorage,
        ocafile: String,
    ) -> Result<OCABuild, Vec<ValidationError>> {
        let (base, oca_ast) = Self::parse_and_check_base(storage, ocafile)?;
        oca_bundle::build::from_ast(base, &oca_ast).map_err(|e| {
            e.iter()
                .map(|e| ValidationError::OCABundleBuild(e.clone()))
                .collect::<Vec<_>>()
        })
    }

    pub fn build_from_ocafile(&mut self, ocafile: String) -> Result<OCABundle, Vec<Error>> {
        let oca_build = Self::validate_ocafile(
            self.db_cache.borrow(),
            ocafile,
            #[cfg(feature = "local-references")]
            &mut self.db,
        )
        .map_err(|errs| vec![Error::ValidationError(errs)])?;

        let oca_bundle_cache_repo = OCABundleCacheRepo::new(Rc::clone(&self.connection));
        let oca_bundle_cache_record = OCABundleCacheRecord::new(&oca_build.oca_bundle);
        oca_bundle_cache_repo.insert(oca_bundle_cache_record);

        let capture_base_cache_repo = CaptureBaseCacheRepo::new(Rc::clone(&self.connection));
        let capture_base_cache_record =
            CaptureBaseCacheRecord::new(&oca_build.oca_bundle.capture_base);
        capture_base_cache_repo.insert(capture_base_cache_record);

        let meta_overlays = oca_build
            .oca_bundle
            .overlays
            .iter()
            .filter_map(|x| {
                x.as_any()
                    .downcast_ref::<oca_bundle::state::oca::overlay::Meta>()
            })
            .collect::<Vec<_>>();
        if !meta_overlays.is_empty() {
            let oca_bundle_fts_repo = OCABundleFTSRepo::new(Rc::clone(&self.connection));
            for meta_overlay in meta_overlays {
                let oca_bundle_fts_record = OCABundleFTSRecord::new(
                    oca_build.oca_bundle.said.clone().unwrap().to_string(),
                    meta_overlay
                        .attr_pairs
                        .get(&"name".to_string())
                        .unwrap_or(&"".to_string())
                        .clone(),
                    meta_overlay
                        .attr_pairs
                        .get(&"description".to_string())
                        .unwrap_or(&"".to_string())
                        .clone(),
                    meta_overlay.language,
                );

                oca_bundle_fts_repo.insert(oca_bundle_fts_record);
            }
        }

        oca_build.steps.iter().for_each(|step| {
            let mut input: Vec<u8> = vec![];
            match &step.parent_said {
                Some(said) => {
                    input.push(said.to_string().as_bytes().len().try_into().unwrap());
                    input.extend(said.to_string().as_bytes());
                }
                None => {
                    input.push(0);
                }
            }

            let command_str = serde_json::to_string(&step.command).unwrap();
            input.extend(command_str.as_bytes());
            let result_bundle = step.result.clone();
            self.db
                .insert(
                    Namespace::OCA,
                    &format!("oca.{}.operation", result_bundle.said.clone().unwrap()),
                    &input,
                )
                .unwrap();

            self.db_cache
                .insert(
                    Namespace::OCABundlesJSON,
                    &result_bundle.said.clone().unwrap().to_string(),
                    &result_bundle.encode().unwrap(),
                )
                .unwrap();
            self.db_cache
                .insert(
                    Namespace::OCAObjectsJSON,
                    &result_bundle.capture_base.said.clone().unwrap().to_string(),
                    &serde_json::to_string(&result_bundle.capture_base)
                        .unwrap()
                        .into_bytes(),
                )
                .unwrap();
            result_bundle.overlays.iter().for_each(|overlay| {
                self.db_cache
                    .insert(
                        Namespace::OCAObjectsJSON,
                        &overlay.said().clone().unwrap().to_string(),
                        &serde_json::to_string(&overlay).unwrap().into_bytes(),
                    )
                    .unwrap();
            });
        });

        let _ = self.add_relations(oca_build.oca_bundle.clone());

        let result_models = build_core_db_model(&oca_build);
        result_models.iter().for_each(|model| {
            if let Some(command_model) = &model.command {
                self.db
                    .insert(
                        Namespace::CoreModel,
                        &format!("core_model.{}", command_model.digest),
                        &command_model.json.clone().into_bytes(),
                    )
                    .unwrap();
            }

            if let Some(capture_base_model) = &model.capture_base {
                let mut input: Vec<u8> = vec![];
                match &capture_base_model.parent {
                    Some(said) => {
                        input.push(said.to_string().as_bytes().len().try_into().unwrap());
                        input.extend(said.to_string().as_bytes());
                    }
                    None => {
                        input.push(0);
                    }
                }

                input.push(
                    capture_base_model
                        .command_digest
                        .to_string()
                        .as_bytes()
                        .len()
                        .try_into()
                        .unwrap(),
                );
                input.extend(capture_base_model.command_digest.to_string().as_bytes());

                self.db
                    .insert(
                        Namespace::CoreModel,
                        &format!("core_model.{}", capture_base_model.capture_base_said),
                        &input,
                    )
                    .unwrap();
            }

            if let Some(overlay_model) = &model.overlay {
                let mut input: Vec<u8> = vec![];
                match &overlay_model.parent {
                    Some(said) => {
                        input.push(said.to_string().as_bytes().len().try_into().unwrap());
                        input.extend(said.to_string().as_bytes());
                    }
                    None => {
                        input.push(0);
                    }
                }

                input.push(
                    overlay_model
                        .command_digest
                        .to_string()
                        .as_bytes()
                        .len()
                        .try_into()
                        .unwrap(),
                );
                input.extend(overlay_model.command_digest.to_string().as_bytes());

                self.db
                    .insert(
                        Namespace::CoreModel,
                        &format!("core_model.{}", overlay_model.overlay_said),
                        &input,
                    )
                    .unwrap();
            }

            if let Some(oca_bundle_model) = &model.oca_bundle {
                let mut input: Vec<u8> = vec![];
                match &oca_bundle_model.parent {
                    Some(said) => {
                        input.push(said.to_string().as_bytes().len().try_into().unwrap());
                        input.extend(said.to_string().as_bytes());
                    }
                    None => {
                        input.push(0);
                    }
                }

                input.push(
                    oca_bundle_model
                        .capture_base_said
                        .to_string()
                        .as_bytes()
                        .len()
                        .try_into()
                        .unwrap(),
                );
                input.extend(oca_bundle_model.capture_base_said.to_string().as_bytes());

                for said in &oca_bundle_model.overlays_said {
                    input.push(said.to_string().as_bytes().len().try_into().unwrap());
                    input.extend(said.to_string().as_bytes());
                }

                self.db
                    .insert(
                        Namespace::CoreModel,
                        &format!("core_model.{}", oca_bundle_model.oca_bundle_said),
                        &input,
                    )
                    .unwrap();
            }
        });

        Ok(oca_build.oca_bundle)
    }
}
