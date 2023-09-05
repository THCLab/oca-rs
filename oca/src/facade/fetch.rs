use super::Facade;
use crate::{data_storage::DataStorage, repositories::OCABundleReadModelRepo};
use oca_bundle::state::oca::OCABundle;
use oca_bundle::build::OCABuildStep;

use std::rc::Rc;

use convert_case::{Case, Casing};

impl Facade {
    pub fn search_oca_bundle(
        &self,
        language: Option<isolang::Language>,
        query: String,
        limit: usize,
        page: usize,
    ) -> crate::repositories::SearchResult {
        let oca_bundle_read_model_repo =
            OCABundleReadModelRepo::new(Rc::clone(&self.connection));
        oca_bundle_read_model_repo.search(language, query, limit, page)
    }

    pub fn get_oca_bundle(&self, said: String) -> Result<OCABundle, Vec<String>> {
        let r = self.db.get(&format!("oca.{}", said)).map_err(|e| vec![format!("{}", e)])?;
        let oca_bundle_str = String::from_utf8(
            r.ok_or_else(|| vec![format!("No OCA Bundle found for said: {}", said)])?
        ).unwrap();
        serde_json::from_str(&oca_bundle_str)
            .map_err(|e| vec![format!("Failed to parse oca bundle: {}", e)])
    }

    pub fn get_oca_bundle_steps(&self, said: String) -> Result<Vec<OCABuildStep>, Vec<String>> {
        let mut said = said;
        #[allow(clippy::borrowed_box)]
        fn extract_operation(db: &Box<dyn DataStorage>, said: &String) -> Result<(String, oca_ast::ast::Command), Vec<String>> {
            let r = db.get(&format!("oca.{}.operation", said))
                .map_err(|e| vec![format!("{}", e)])?
                .ok_or_else(|| vec![format!("No history found for said: {}", said)])?;

            let said_length = r.first().unwrap();
            let parent_said = String::from_utf8_lossy(&r[1..*said_length as usize + 1]).to_string();
            let op_length = r[*said_length as usize + 1];
            let op = String::from_utf8_lossy(&r[*said_length as usize + 2..*said_length as usize + 2 + op_length as usize]).to_string();

            Ok((
                parent_said,
                serde_json::from_str(&op).unwrap()
            ))
        }

        let mut history: Vec<OCABuildStep> = vec![];

        loop {
            let (parent_said, command) = extract_operation(&self.db, &said)?;
            if parent_said == said {
                dbg!("Malformed history for said: {}", said);
                return Err(vec![format!("Malformed history")]);
            }
            history.push(
                OCABuildStep {
                    parent_said: parent_said.clone().parse().ok(),
                    command,
                    result: self.get_oca_bundle(said.clone()).unwrap(),
                }
            );
            said = parent_said;

            if said.is_empty() {
                break;
            }
        };
        history.reverse();
        Ok(history)
    }

    pub fn get_oca_bundle_ocafile(&self, said: String) -> Result<String, Vec<String>> {
        let mut steps = self.get_oca_bundle_steps(said)?;
        let mut ocafile = String::new();

        steps.iter_mut().for_each(|step| {
            let mut line = String::new();

            if let oca_ast::ast::CommandType::Add = step.command.kind {
                line.push_str("ADD ");
                match &step.command.object_kind {
                    oca_ast::ast::ObjectKind::CaptureBase => {
                        if let Some(ref content) = step.command.content {
                            if let Some(ref attributes) = content.attributes {
                                line.push_str("ATTRIBUTE ");
                                attributes.iter().for_each(|(key, value)| {
                                    if let oca_ast::ast::NestedValue::Value(value) = value {
                                        line.push_str(format!("{}={} ", key, value).as_str());
                                    }
                                });
                            }
                        };
                    },
                    oca_ast::ast::ObjectKind::Overlay(o_type) => {
                        match o_type {
                            oca_ast::ast::OverlayType::Meta => {
                                line.push_str("META ");
                                if let Some(ref mut content) = step.command.content {
                                    if let Some(ref mut properties) = content.properties {
                                        if let Some(
                                            oca_ast::ast::NestedValue::Value(lang)
                                        ) = properties.remove("lang") {
                                            line.push_str(format!("{} ", lang).as_str());
                                        }
                                        if !properties.is_empty() {
                                            line.push_str("PROPS ");
                                            properties.iter().for_each(|(key, value)| {
                                                if let oca_ast::ast::NestedValue::Value(value) = value {
                                                    line.push_str(format!("{}=\"{}\" ", key, value).as_str());
                                                }
                                            });
                                        }
                                    }
                                };
                            },
                            oca_ast::ast::OverlayType::Unit => {
                                line.push_str("UNIT ");
                                if let Some(ref mut content) = step.command.content {
                                    if let Some(ref mut properties) = content.properties {
                                        if let Some(
                                            oca_ast::ast::NestedValue::Value(unit_system)
                                        ) = properties.remove("unit_system") {
                                            line.push_str(format!("{} ", unit_system).as_str());
                                        }
                                        if !properties.is_empty() {
                                            line.push_str("PROPS ");
                                            properties.iter().for_each(|(key, value)| {
                                                if let oca_ast::ast::NestedValue::Value(value) = value {
                                                    line.push_str(format!("{}=\"{}\" ", key, value).as_str());
                                                }
                                            });
                                        }
                                        if let Some(ref attributes) = content.attributes {
                                            line.push_str("ATTRS ");
                                            attributes.iter().for_each(|(key, value)| {
                                                if let oca_ast::ast::NestedValue::Value(value) = value {
                                                    line.push_str(format!("{}=\"{}\" ", key, value).as_str());
                                                }
                                            });
                                        }
                                    }
                                };
                            },
                            oca_ast::ast::OverlayType::EntryCode => {
                                line.push_str("ENTRY_CODE ");
                                if let Some(ref mut content) = step.command.content {
                                    if let Some(ref mut properties) = content.properties {
                                        if !properties.is_empty() {
                                            line.push_str("PROPS ");
                                            properties.iter().for_each(|(key, value)| {
                                                if let oca_ast::ast::NestedValue::Value(value) = value {
                                                    line.push_str(format!("{}={} ", key, value).as_str());
                                                }
                                            });
                                        }
                                        if let Some(ref attributes) = content.attributes {
                                            line.push_str("ATTRS ");
                                            attributes.iter().for_each(|(key, value)| {
                                                if let oca_ast::ast::NestedValue::Array(values) = value {
                                                    let codes = values.iter().filter_map(|value| {
                                                        if let oca_ast::ast::NestedValue::Value(value) = value {
                                                            Some(format!("\"{}\"", value))
                                                        } else {
                                                            None
                                                        }
                                                    }).collect::<Vec<String>>().join(", ");
                                                    line.push_str(format!("{}=[{}] ", key, codes).as_str());
                                                }
                                            });
                                        }
                                    }
                                };
                            },
                            oca_ast::ast::OverlayType::Entry => {
                                line.push_str("ENTRY ");
                                if let Some(ref mut content) = step.command.content {
                                    if let Some(ref mut properties) = content.properties {
                                        if let Some(
                                            oca_ast::ast::NestedValue::Value(lang)
                                        ) = properties.remove("lang") {
                                            line.push_str(format!("{} ", lang).as_str());
                                        }
                                        if !properties.is_empty() {
                                            line.push_str("PROPS ");
                                            properties.iter().for_each(|(key, value)| {
                                                if let oca_ast::ast::NestedValue::Value(value) = value {
                                                    line.push_str(format!("{}={} ", key, value).as_str());
                                                }
                                            });
                                        }
                                        if let Some(ref attributes) = content.attributes {
                                            line.push_str("ATTRS ");
                                            attributes.iter().for_each(|(key, value)| {
                                                if let oca_ast::ast::NestedValue::Object(values) = value {
                                                    let codes = values.iter().filter_map(|(code, label)| {
                                                        if let oca_ast::ast::NestedValue::Value(label) = label {
                                                            Some(format!("\"{}\": \"{}\"", code, label))
                                                        } else {
                                                            None
                                                        }
                                                    }).collect::<Vec<String>>().join(", ");
                                                    line.push_str(format!("{}={{ {} }} ", key, codes).as_str());
                                                }
                                            });
                                        }
                                    }
                                };
                            },
                            _ => {
                                line.push_str(
                                    format!(
                                        "{} ",
                                        o_type.to_string().to_case(Case::UpperSnake)
                                    ).as_str()
                                );

                                if let Some(ref mut content) = step.command.content {
                                    if let Some(ref mut properties) = content.properties {
                                        if let Some(
                                            oca_ast::ast::NestedValue::Value(lang)
                                        ) = properties.remove("lang") {
                                            line.push_str(format!("{} ", lang).as_str());
                                        }
                                        if !properties.is_empty() {
                                            line.push_str("PROPS ");
                                            properties.iter().for_each(|(key, value)| {
                                                if let oca_ast::ast::NestedValue::Value(value) = value {
                                                    line.push_str(format!("{}=\"{}\" ", key, value).as_str());
                                                }
                                            });
                                        }
                                    }
                                    if let Some(ref attributes) = content.attributes {
                                        line.push_str("ATTRS ");
                                        attributes.iter().for_each(|(key, value)| {
                                            if let oca_ast::ast::NestedValue::Value(value) = value {
                                                line.push_str(format!("{}=\"{}\" ", key, value).as_str());
                                            }
                                        });
                                    }
                                };
                            }
                        }
                    },
                    _ => {}
                }
            }

            ocafile.push_str(format!("{}\n", line).as_str());
        });

        Ok(ocafile)
    }
}
