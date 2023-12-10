pub mod build;
pub mod data_storage;
pub mod versioning;

use oca_ast::ast;
use said::{derivation::HashFunction, SelfAddressingIdentifier};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct CommandModel {
    pub digest: SelfAddressingIdentifier,
    pub json: String,
}
impl CommandModel {
    fn new(command: ast::Command) -> Self {
        Self {
            digest: Self::calculate_command_digest(&command),
            json: serde_json::to_string(&command).unwrap(),
        }
    }

    fn calculate_command_digest(
        command: &ast::Command,
    ) -> SelfAddressingIdentifier {
        let command_json = serde_json::to_string(command).unwrap();
        let hash_algorithm = HashFunction::from_str("E").unwrap();
        hash_algorithm.derive(command_json.as_bytes())
    }
}

#[derive(Debug, Clone)]
pub struct CaptureBaseModel {
    pub capture_base_said: SelfAddressingIdentifier,
    pub parent: Option<SelfAddressingIdentifier>,
    pub command_digest: SelfAddressingIdentifier,
}

#[derive(Debug, Clone)]
pub struct OverlayModel {
    pub overlay_said: SelfAddressingIdentifier,
    pub parent: Option<SelfAddressingIdentifier>,
    pub command_digest: SelfAddressingIdentifier,
}

#[derive(Debug, Clone)]
pub struct OCABundleModel {
    pub oca_bundle_said: SelfAddressingIdentifier,
    pub parent: Option<SelfAddressingIdentifier>,
    pub capture_base_said: SelfAddressingIdentifier,
    pub overlays_said: Vec<SelfAddressingIdentifier>,
}

#[derive(Debug, Clone)]
struct State {
    oca_bundle: Option<SelfAddressingIdentifier>,
    capture_base: Option<SelfAddressingIdentifier>,
    overlays: HashMap<String, SelfAddressingIdentifier>,
}
impl State {
    fn new() -> Self {
        Self {
            oca_bundle: None,
            capture_base: None,
            overlays: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct ResultModel {
    pub command: Option<CommandModel>,
    pub oca_bundle: Option<OCABundleModel>,
    pub capture_base: Option<CaptureBaseModel>,
    pub overlay: Option<OverlayModel>,
}
impl ResultModel {
    fn new() -> Self {
        Self {
            command: None,
            oca_bundle: None,
            capture_base: None,
            overlay: None,
        }
    }
}

pub fn build_core_db_model(
    oca_build: &oca_bundle::build::OCABuild,
) -> Vec<ResultModel> {
    let mut state = State::new();
    let mut result_models = vec![];

    for step in &oca_build.steps {
        let (new_state, result_model) = apply_step(state, step);
        state = new_state;
        result_models.push(result_model);
    }

    result_models
}

fn apply_step(
    state: State,
    step: &oca_bundle::build::OCABuildStep,
) -> (State, ResultModel) {
    let mut current_state = state.clone();
    let mut result = ResultModel::new();
    let command_model = CommandModel::new(step.command.clone());
    result.command = Some(command_model.clone());

    match &step.command.object_kind {
        ast::ObjectKind::CaptureBase(_) => {
            let capture_base_model = CaptureBaseModel {
                capture_base_said: step
                    .result
                    .capture_base
                    .said
                    .clone()
                    .unwrap(),
                parent: state.capture_base,
                command_digest: command_model.digest,
            };
            result.capture_base = Some(capture_base_model.clone());
            current_state.capture_base =
                Some(capture_base_model.capture_base_said.clone());
        }
        ast::ObjectKind::Overlay(overlay_type, content) => {
            let mut lang = None;
            match &content.properties {
                Some(properties) => {
                    if let Some(ast::NestedValue::Value(lang_value)) =
                        properties.get("lang")
                    {
                        lang = isolang::Language::from_639_1(lang_value);
                    }
                }
                _ => (),
            }

            let overlay = step.result.overlays.iter().find(|overlay| {
                overlay.overlay_type().eq(overlay_type)
                  && overlay.language() == lang.as_ref()
            });

            if let Some(overlay) = overlay {
                let overlay_key = match overlay.language() {
                    Some(lang) => {
                        format!(
                            "{}-{}",
                            &overlay.overlay_type(),
                            lang.to_639_1().unwrap()
                        )
                    }
                    None => format!("{}", overlay.overlay_type()),
                };
                let parent_overlay = state.overlays.get(&overlay_key);
                let overlay_model = OverlayModel {
                    overlay_said: overlay.said().clone().unwrap(),
                    parent: parent_overlay.cloned(),
                    command_digest: command_model.digest,
                };
                result.overlay = Some(overlay_model.clone());
                current_state
                    .overlays
                    .insert(overlay_key, overlay_model.overlay_said.clone());
            }
        }
        ast::ObjectKind::OCABundle(_) => {
            dbg!("OCABundle");
        }
    }

    let oca_bundle_model = OCABundleModel {
        oca_bundle_said: step.result.said.clone().unwrap(),
        parent: state.oca_bundle,
        capture_base_said: step.result.capture_base.said.clone().unwrap(),
        overlays_said: step
            .result
            .overlays
            .iter()
            .map(|overlay| overlay.said().clone().unwrap())
            .collect(),
    };
    result.oca_bundle = Some(oca_bundle_model.clone());
    current_state.oca_bundle = Some(oca_bundle_model.oca_bundle_said.clone());

    (current_state, result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indexmap::{indexmap, IndexMap};
    use oca_ast::ast::Content;

    #[test]
    fn test_build_core_db_model() -> Result<(), Vec<String>> {
        let mut commands = vec![];

        // 1. ADD ATTR abc
        commands.push(ast::Command {
            kind: ast::CommandType::Add,
            object_kind: ast::ObjectKind::CaptureBase( ast::CaptureContent {
                attributes: Some(indexmap! {
                    "abc".to_string() => ast::NestedAttrType::Value(ast::AttributeType::Text)
                }),
                properties: None,
            })
        });

        // 2. add label en abc "ble"
        commands.push(ast::Command {
            kind: ast::CommandType::Add,
            object_kind: ast::ObjectKind::Overlay(ast::OverlayType::Label, Content {
                attributes: Some(indexmap! {
                    "abc".to_string() => ast::NestedValue::Value("ble".to_string())
                }),
                properties: Some(indexmap! {
                    "lang".to_string() => ast::NestedValue::Value("en".to_string())
                }),
            }),
        });

        // 3. add attr def
        commands.push(ast::Command {
            kind: ast::CommandType::Add,
            object_kind: ast::ObjectKind::CaptureBase(ast::CaptureContent {
                attributes: Some(indexmap! {
                    "def".to_string() => ast::NestedAttrType::Value(ast::AttributeType::Text)
                }),
                properties: None,
            })
        });

        // 4. add label fr abc "ble"
        commands.push(ast::Command {
            kind: ast::CommandType::Add,
            object_kind: ast::ObjectKind::Overlay(ast::OverlayType::Label, Content {
                attributes: Some(indexmap! {
                    "abc".to_string() => ast::NestedValue::Value("ble".to_string())
                }),
                properties: Some(indexmap! {
                    "lang".to_string() => ast::NestedValue::Value("fr".to_string())
                }),
            }),
        });

        // 5. update attr "en" abc "bererg"
        /*
        commands.push(
            ast::Command {
                kind: ast::CommandType::Modify,
                object_kind: ast::ObjectKind::Overlay(ast::OverlayType::Label),
                content: Some(ast::Content {
                    attributes: Some(indexmap! {
                        "abc".to_string() => ast::NestedValue::Value("bererg".to_string())
                    }),
                    properties: Some(indexmap! {
                        "lang".to_string() => ast::NestedValue::Value("en".to_string())
                    }),
                }),
            }
        );
        */

        let ast = ast::OCAAst {
            version: "0.1.0".to_string(),
            commands,
            commands_meta: IndexMap::new(),
            meta: HashMap::new(),
        };
        let refs = HashMap::new();
        let oca_build = oca_bundle::build::from_ast(None, ast, refs).unwrap();

        let result = build_core_db_model(&oca_build);
        assert_eq!(result.len(), 4);
        assert_eq!(
            result[0].command.clone().unwrap().digest.to_string(),
            "EJnPP-iVzRutbWJF4tt3pX_89NB77854DMyYdl_aVaWH"
        );
        assert!(result[0].oca_bundle.is_some());
        assert!(result[0].capture_base.is_some());
        assert!(result[0].overlay.is_none());

        assert!(result[3].oca_bundle.is_some());
        assert!(result[3].capture_base.is_none());
        assert!(result[3].overlay.is_some());

        Ok(())
    }
}
