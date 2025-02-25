use crate::ocafile::{error::ExtractingAttributeError, Pair, Rule};
use log::debug;

pub fn extract_rename_attribute(
    attr_pair: Pair,
) -> Result<(String, String), ExtractingAttributeError> {
    let mut attr_name_opt = None;
    let mut attr_new_name_opt = None;

    debug!(
        "Extracting the attribute rename values from: {:?}",
        attr_pair
    );
    for item in attr_pair.into_inner() {
        match item.as_rule() {
            Rule::attr_key => {
                debug!("Extracting attribute key {:?}", attr_name_opt);
                if attr_name_opt.is_none() {
                    attr_name_opt = Some(item.as_str().to_string());
                } else {
                    attr_new_name_opt = Some(item.as_str().to_string());
                }
            }
            rule => {
                return Err(ExtractingAttributeError::Unexpected(format!(
                    "Unexpected pest rule: {:?}",
                    rule
                )))
            }
        }
    }
    if attr_name_opt.is_none() {
        return Err(ExtractingAttributeError::Unexpected(
            "Missing attribute name".to_string(),
        ));
    }
    if attr_new_name_opt.is_none() {
        return Err(ExtractingAttributeError::Unexpected(
            "Missing new attribute name".to_string(),
        ));
    }
    Ok((attr_name_opt.unwrap(), attr_new_name_opt.unwrap()))
}

pub fn extract_link_attribute(
    attr_pair: Pair,
) -> Result<(String, String), ExtractingAttributeError> {
    extract_rename_attribute(attr_pair)
}
