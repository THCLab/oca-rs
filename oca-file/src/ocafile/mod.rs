pub mod error;

use std::collections::HashMap;

use self::error::ParseError;
use oca_file_semantics::ocafile::{
    parse_from_string as semantics_parse_from_string, OCAAst as SemanticsAst,
};
use oca_file_transformation::ocafile::{
    parse_from_string as transformation_parse_from_string, TransformationAST,
};
use pest::Parser;

#[derive(pest_derive::Parser)]
#[grammar = "ocafile.pest"]
pub struct OCAfileParser;

#[derive(Debug)]
pub enum OCAAst {
    TransformationAst(TransformationAST),
    SemanticsAst(SemanticsAst),
}

pub type Pair<'a> = pest::iterators::Pair<'a, Rule>;

pub fn parse_from_string(unparsed_file: String) -> Result<OCAAst, ParseError> {
    let file = OCAfileParser::parse(Rule::file, &unparsed_file)
        .map_err(|e| {
            let (line_number, column_number) = match e.line_col {
                pest::error::LineColLocation::Pos((line, column)) => (line, column),
                pest::error::LineColLocation::Span((line, column), _) => (line, column),
            };
            ParseError::GrammarError {
                line_number,
                column_number,
                raw_line: e.line().to_string(),
                message: e.variant.to_string(),
            }
        })?
        .next()
        .unwrap();

    let mut meta = HashMap::new();

    for line in file.into_inner() {
        if let Rule::EOI = line.as_rule() {
            continue;
        }
        if let Rule::comment = line.as_rule() {
            continue;
        }
        if let Rule::meta_comment = line.as_rule() {
            let mut key = "".to_string();
            let mut value = "".to_string();
            for attr in line.into_inner() {
                match attr.as_rule() {
                    Rule::meta_attr_key => {
                        key = attr.as_str().to_string();
                    }
                    Rule::meta_attr_value => {
                        value = attr.as_str().to_string();
                    }
                    _ => {
                        return Err(ParseError::MetaError(attr.as_str().to_string()));
                    }
                }
            }
            if key.is_empty() {
                return Err(ParseError::MetaError("key is empty".to_string()));
            }
            if value.is_empty() {
                return Err(ParseError::MetaError("value is empty".to_string()));
            }
            meta.insert(key, value);
            continue;
        }
        if let Rule::commands = line.as_rule() {
            continue;
        }
        if let Rule::empty_line = line.as_rule() {
            continue;
        }
    }

    if let Some(value) = meta.get("precompiler") {
        if value == "transformation" {
            return Ok(OCAAst::TransformationAst(
                transformation_parse_from_string(unparsed_file)
                    .map_err(ParseError::TransformationError)?,
            ));
        } else if value == "semantics" {
            return Ok(OCAAst::SemanticsAst(
                semantics_parse_from_string(unparsed_file).map_err(ParseError::SemanticsError)?,
            ));
        } else {
            return Err(ParseError::MetaError("unknown precompiler".to_string()));
        }
    }

    Ok(OCAAst::SemanticsAst(
        semantics_parse_from_string(unparsed_file).map_err(ParseError::SemanticsError)?,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_transformation_from_string_valid() {
        let _ = env_logger::builder().is_test(true).try_init();

        let unparsed_file = r#"
-- precompiler=transformation
-- version=0.0.1
-- name=Objekt
RENAME ATTRIBUTE surname=last_name
"#;
        let oca_ast = parse_from_string(unparsed_file.to_string()).unwrap();
        assert!(matches!(oca_ast, OCAAst::TransformationAst(_)));
    }

    #[test]
    fn parse_semantics_from_string_valid() {
        let _ = env_logger::builder().is_test(true).try_init();

        let unparsed_file = r#"
-- precompiler=semantics
-- version=0.0.1
-- name=Objekt
ADD ATTRIBUTE surname=Text
"#;
        let oca_ast = parse_from_string(unparsed_file.to_string()).unwrap();
        assert!(matches!(oca_ast, OCAAst::SemanticsAst(_)));
    }

    #[test]
    fn parse_semantics_from_string_by_default() {
        let _ = env_logger::builder().is_test(true).try_init();

        let unparsed_file = r#"
-- version=0.0.1
-- name=Objekt
ADD ATTRIBUTE surname=Text
"#;
        let oca_ast = parse_from_string(unparsed_file.to_string()).unwrap();
        assert!(matches!(oca_ast, OCAAst::SemanticsAst(_)));
    }
}
