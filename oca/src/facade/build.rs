use super::Facade;
use oca_bundle::state::oca::OCABundle;
use oca_bundle::Encode;

impl Facade {
    pub fn build_from_ocafile(&self, ocafile: String) -> Result<OCABundle, Vec<String>> {
        let mut errors = vec![];
        let mut oca_ast = oca_file::ocafile::parse_from_string(ocafile)
            .map_err(|e| vec![format!("Failed to parse ocafile: {}", e)])
            ?;

        let mut base: Option<OCABundle> = None;
        if let Some(first_command) = oca_ast.commands.get(0) {
            if let oca_ast::ast::CommandType::From = first_command.kind {
                if let Some(ref content) = first_command.content {
                    if let Some(ref properties) = content.properties {
                        if let Some(oca_ast::ast::NestedValue::Value(said)) = properties.get("said") {
                            match self.get_oca_bundle(said.clone()) {
                                Ok(oca_bundle) => {
                                    base = Some(oca_bundle);
                                },
                                Err(e) => {
                                    let default_command_meta = oca_ast::ast::CommandMeta { line_number: 0, raw_line: "unknown".to_string() };
                                    let command_meta = oca_ast.commands_meta.get(&0).unwrap_or(&default_command_meta);
                                    e.iter().for_each(|e| errors.push(
                                        format!("Error at line {} ({}): {}", command_meta.line_number, command_meta.raw_line, e)
                                    ));
                                }
                            }
                        }
                    }
                }

                oca_ast.commands.remove(0);
            }
        }
        if !errors.is_empty() {
            return Err(errors);
        }

        let oca_build = oca_bundle::build::from_ast(base, oca_ast)?;

        oca_build.steps.iter().for_each(|step| {
            let mut input: Vec<u8> = vec![];
            match &step.parent_said {
                Some(said) => {
                    input.push(said.to_string().as_bytes().len().try_into().unwrap());
                    input.extend(said.to_string().as_bytes());
                },
                None => {
                    input.push(0);
                }
            }

            let command_str = serde_json::to_string(&step.command).unwrap();
            input.push(command_str.as_bytes().len().try_into().unwrap());
            input.extend(command_str.as_bytes());
            let result_bundle = step.result.clone();
            self.db.insert(
                &format!("oca.{}.operation", result_bundle.said.clone().unwrap()),
                &input,
            ).unwrap();
            self.db.insert(
                &format!("oca.{}", result_bundle.said.clone().unwrap()),
                &result_bundle.encode().unwrap(),
            ).unwrap();
        });

        Ok(oca_build.oca_bundle)
    }
}
