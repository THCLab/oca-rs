use crate::state::Transformation;
use oca_ast_transformation::ast;

#[derive(Debug, Clone, serde::Serialize)]
pub struct FromASTError {
    pub line_number: usize,
    pub raw_line: String,
    pub message: String,
}

#[derive(thiserror::Error, Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum Error {
    #[error("Error at line {line_number} ({raw_line}): {message}")]
    FromASTError {
        #[serde(rename = "ln")]
        line_number: usize,
        #[serde(rename = "c")]
        raw_line: String,
        #[serde(rename = "e")]
        message: String,
    },
}

pub fn from_ast(ast: &ast::TransformationAST) -> Result<Transformation, Vec<Error>> {
    let mut errors = vec![];

    let mut base: Option<Transformation> = None;
    if !ast.meta.is_empty() {
        let source = ast.meta.get("source").expect("missing source meta");
        let source_said = source.replace("refs:", "");
        let target = ast.meta.get("target").expect("missing target meta");
        let target_said = target.replace("refs:", "");
        let mut transformation = Transformation::new();
        transformation.set_source(source_said);
        transformation.set_target(target_said);
        base = Some(transformation)
    }

    let default_command_meta = ast::CommandMeta {
        line_number: 0,
        raw_line: "unknown".to_string(),
    };
    for (i, command) in ast.commands.iter().enumerate() {
        let command_index = i;
        // todo pass the references
        let command_meta = ast
            .commands_meta
            .get(&command_index)
            .unwrap_or(&default_command_meta);
        match apply_command(base.clone(), command.clone()) {
            Ok(transformation) => {
                base = Some(transformation);
            }
            Err(mut err) => {
                errors.extend(err.iter_mut().map(|e| Error::FromASTError {
                    line_number: command_meta.line_number,
                    raw_line: command_meta.raw_line.clone(),
                    message: e.clone(),
                }));
            }
        }
    }
    if errors.is_empty() {
        let mut transformation = base.unwrap().clone();
        transformation.fill_said();
        Ok(transformation)
    } else {
        Err(errors)
    }
}

pub fn apply_command(
    base: Option<Transformation>,
    op: ast::Command,
) -> Result<Transformation, Vec<String>> {
    let errors = vec![];
    let mut transformation: Transformation = match base {
        Some(transformation) => transformation,
        None => Transformation::new(),
    };

    match (op.kind, op.object_kind) {
        (ast::CommandType::Rename, ast::ObjectKind::Rename(content)) => {
            if let Some(attributes) = content.attributes {
                transformation.rename(attributes);
            }
        }
        (ast::CommandType::Link, ast::ObjectKind::Link(content)) => {
            if let Some(attributes) = content.attributes {
                transformation.link(attributes);
            }
        }
        _ => {}
    }

    if errors.is_empty() {
        Ok(transformation)
    } else {
        Err(errors)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use indexmap::IndexMap;
    use said::{derivation::HashFunctionCode, sad::SerializationFormats, version::Encode};

    #[test]
    fn build_from_ast() {
        let mut commands = vec![];

        let mut attributes = IndexMap::new();
        attributes.insert("digest".to_string(), "d".to_string());

        commands.push(ast::Command {
            kind: ast::CommandType::Rename,
            object_kind: ast::ObjectKind::Rename(ast::RenameContent {
                attributes: Some(attributes),
            }),
        });

        let ast = ast::TransformationAST {
            version: "1.0".to_string(),
            commands,
            commands_meta: IndexMap::new(),
            meta: HashMap::new(),
        };

        let build_result = from_ast(&ast);
        match build_result {
            Ok(transformation) => {
                let code = HashFunctionCode::Blake3_256;
                let format = SerializationFormats::JSON;
                let transformation_encoded = transformation.encode(&code, &format).unwrap();
                let transformation_json = String::from_utf8(transformation_encoded).unwrap();
                println!("{}", transformation_json);
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }
}
