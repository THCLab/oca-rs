use std::str::FromStr;

use crate::ocafile::{error::ExtractingAttributeError, Pair, Rule};
use indexmap::IndexMap;
use log::debug;
use oca_ast::ast::{
    recursive_attributes::{AttributeTypeResult, NestedAttrTypeFrame},
    AttributeType, Content, NestedAttrType, NestedValue, RefValue,
};
use recursion::ExpandableExt;
use said::SelfAddressingIdentifier;

fn extract_attr_type(input: Pair) -> Result<NestedAttrType, ExtractingAttributeError> {
    let res = AttributeTypeResult::expand_frames(input, |seed| match seed.as_rule() {
        Rule::array_attr_type => match seed.into_inner().next() {
            Some(next) => NestedAttrTypeFrame::Array(next).into(),
            None => {
                ExtractingAttributeError::Unexpected("Missing attribute type".to_string()).into()
            }
        },
        Rule::alias => {
            NestedAttrTypeFrame::Reference(oca_ast::ast::RefValue::Name(seed.as_str().to_string()))
                .into()
        }
        Rule::said => match SelfAddressingIdentifier::from_str(seed.as_str()) {
            Ok(said) => NestedAttrTypeFrame::Reference(RefValue::Said(said)).into(),
            Err(e) => ExtractingAttributeError::SaidError(e).into(),
        },
        Rule::base_attr_type => {
            let seed_str = seed.as_span().as_str();
            match AttributeType::from_str(seed_str) {
                Ok(attr_type) => NestedAttrTypeFrame::Value(attr_type).into(),
                Err(_) => ExtractingAttributeError::Unexpected(format!(
                    "Unknown attribute type {}",
                    seed_str
                ))
                .into(),
            }
        }
        rule => {
            ExtractingAttributeError::Unexpected(format!("Unexpected pest rule: {:?}", rule)).into()
        }
    });
    res.value()
}

pub fn extract_attribute(
    attr_pair: Pair,
) -> Result<(String, NestedAttrType), ExtractingAttributeError> {
    let mut attr_name = String::new();
    let mut attr_type = NestedAttrType::Value(AttributeType::Text);

    debug!("Extracting the attribute type from: {:?}", attr_pair);
    for item in attr_pair.into_inner() {
        match item.as_rule() {
            Rule::attr_key => {
                debug!("Extracting attribute key {:?}", attr_name);
                attr_name = item.as_str().to_string();
            }
            Rule::_attr_type => {
                debug!("Attribute type to parse: {:?}", item);
                let item_field_label = item.as_span().as_str();
                let mut inner = item.into_inner();
                let inner_pair =
                    inner
                        .next()
                        .ok_or(ExtractingAttributeError::Unexpected(format!(
                            "Missing attribute type for {} field",
                            item_field_label
                        )))?;
                attr_type = extract_attr_type(inner_pair)?;
            }
            rule => {
                return Err(ExtractingAttributeError::Unexpected(format!(
                    "Unexpected pest rule: {:?}",
                    rule
                )))
            }
        }
    }
    Ok((attr_name, attr_type))
}

/// Extract attributes key pairs for ADD and MODIFY command
pub fn extract_attribute_key_pairs(attr_pair: Pair) -> Option<(String, NestedValue)> {
    let mut key = String::new();
    let mut value = NestedValue::Value(String::new());

    debug!("Extracting the attribute from: {:?}", attr_pair);
    for item in attr_pair.into_inner() {
        match item.as_rule() {
            Rule::attr_key => {
                key = item.as_str().to_string();
                debug!("Extracting attribute key {:?}", key);
            }
            Rule::key_value => {
                if let Some(nested_item) = item.clone().into_inner().next() {
                    match nested_item.as_rule() {
                        Rule::string => {
                            value = NestedValue::Value(
                                nested_item
                                    .clone()
                                    .into_inner()
                                    .last()
                                    .unwrap()
                                    .as_span()
                                    .as_str()
                                    .to_string(),
                            );
                        }
                        _ => {
                            value = NestedValue::Value(item.as_str().to_string());
                        }
                    }
                }
            }
            Rule::entry_code_key_value => {
                if let Some(entry_code_item) = item.clone().into_inner().next() {
                    match entry_code_item.as_rule() {
                        Rule::key_value => {
                            if let Some(nested_item) = entry_code_item.clone().into_inner().next() {
                                match nested_item.as_rule() {
                                    Rule::string => {
                                        value = NestedValue::Value(
                                            nested_item
                                                .clone()
                                                .into_inner()
                                                .last()
                                                .unwrap()
                                                .as_span()
                                                .as_str()
                                                .to_string(),
                                        );
                                    }
                                    _ => {
                                        value = NestedValue::Value(item.as_str().to_string());
                                    }
                                }
                            }
                        }
                        Rule::entry_code_list => {
                            let mut entry_codes = Vec::new();
                            for el in entry_code_item.clone().into_inner() {
                                match el.as_rule() {
                                    Rule::string => {
                                        entry_codes.push(NestedValue::Value(
                                            el.clone()
                                                .into_inner()
                                                .last()
                                                .unwrap()
                                                .as_span()
                                                .as_str()
                                                .to_string(),
                                        ));
                                    }
                                    _ => {
                                        panic!("Invalid entry code value in {:?}", el.as_rule());
                                    }
                                }
                            }
                            value = NestedValue::Array(entry_codes);
                        }
                        Rule::entry_code_object => {
                            let mut entry_codes_grouped = IndexMap::new();
                            for el in entry_code_item.clone().into_inner() {
                                let (entry_key, entry_value) =
                                    extract_attribute_key_pairs(el).unwrap();
                                entry_codes_grouped.insert(entry_key, entry_value);
                            }
                            value = NestedValue::Object(entry_codes_grouped);
                        }
                        _ => {
                            panic!(
                                "Invalid entry code value in {:?}",
                                entry_code_item.as_rule()
                            );
                        }
                    }
                }
            }
            Rule::entry_code_group_key => {
                if let Some(nested_item) = item.clone().into_inner().next() {
                    if let Rule::string = nested_item.as_rule() {
                        key = nested_item
                            .clone()
                            .into_inner()
                            .last()
                            .unwrap()
                            .as_span()
                            .as_str()
                            .to_string();
                    }
                }
            }
            Rule::entry_code_list => {
                let mut entry_codes = Vec::new();
                for el in item.clone().into_inner() {
                    match el.as_rule() {
                        Rule::string => {
                            entry_codes.push(NestedValue::Value(
                                el.clone()
                                    .into_inner()
                                    .last()
                                    .unwrap()
                                    .as_span()
                                    .as_str()
                                    .to_string(),
                            ));
                        }
                        _ => {
                            panic!("Invalid entry code value in {:?}", el.as_rule());
                        }
                    }
                }
                value = NestedValue::Array(entry_codes);
            }
            Rule::entry_key => {
                if let Some(nested_item) = item.clone().into_inner().next() {
                    if let Rule::string = nested_item.as_rule() {
                        key = nested_item
                            .clone()
                            .into_inner()
                            .last()
                            .unwrap()
                            .as_span()
                            .as_str()
                            .to_string();
                    }
                }
            }
            Rule::entry_value => {
                if let Some(nested_item) = item.clone().into_inner().next() {
                    if let Rule::string = nested_item.as_rule() {
                        value = NestedValue::Value(
                            nested_item
                                .clone()
                                .into_inner()
                                .last()
                                .unwrap()
                                .as_span()
                                .as_str()
                                .to_string(),
                        );
                    }
                }
            }
            Rule::entry_key_value => {
                if let Some(entry_item) = item.clone().into_inner().next() {
                    match entry_item.as_rule() {
                        Rule::key_value => {
                            if let Some(nested_item) = entry_item.clone().into_inner().next() {
                                match nested_item.as_rule() {
                                    Rule::string => {
                                        value = NestedValue::Value(
                                            nested_item
                                                .clone()
                                                .into_inner()
                                                .last()
                                                .unwrap()
                                                .as_span()
                                                .as_str()
                                                .to_string(),
                                        );
                                    }
                                    _ => {
                                        value = NestedValue::Value(item.as_str().to_string());
                                    }
                                }
                            }
                        }
                        Rule::entry_object => {
                            let mut entries = IndexMap::new();
                            for el in entry_item.clone().into_inner() {
                                let (entry_key, entry_value) =
                                    extract_attribute_key_pairs(el).unwrap();
                                entries.insert(entry_key, entry_value);
                            }
                            value = NestedValue::Object(entries);
                        }
                        _ => {
                            panic!("Invalid entry value in {:?}", entry_item.as_rule());
                        }
                    }
                }
            }
            _ => {
                panic!("Invalid attribute in {:?}", item.as_rule());
            }
        }
    }
    Some((key, value))
}

pub fn extract_attributes_key_paris(object: Pair) -> Option<IndexMap<String, NestedValue>> {
    let mut attributes: IndexMap<String, NestedValue> = IndexMap::new();

    debug!("Extracting content of the attributes: {:?}", object);
    for attr in object.into_inner() {
        debug!("Inside the object: {:?}", attr);
        match attr.as_rule() {
            Rule::attr_key_pairs | Rule::attr_entry_code_key_pairs | Rule::attr_entry_key_pairs => {
                for attr in attr.into_inner() {
                    debug!("Parsing attribute {:?}", attr);
                    if let Some((key, value)) = extract_attribute_key_pairs(attr) {
                        debug!("Parsed attribute: {:?} = {:?}", key, value);
                        // TODO find out how to parse nested objects
                        attributes.insert(key, value);
                    } else {
                        debug!("Skipping attribute");
                    }
                }
            }
            _ => {
                debug!(
                    "Unexpected token: Skipping invalid attribute in instruction {:?}",
                    attr.as_rule()
                );
            }
        }
    }

    Some(attributes)
}

/// Extract properties key pairs for any command
pub fn extract_properites_key_pairs(object: Pair) -> Option<IndexMap<String, NestedValue>> {
    let mut properties: IndexMap<String, NestedValue> = IndexMap::new();

    debug!("Extracting properties from the object: {:?}", object);
    for attr in object.into_inner() {
        debug!("Inside the object: {:?}", attr);
        match attr.as_rule() {
            Rule::prop_key_pairs => {
                for prop in attr.into_inner() {
                    debug!("Parsing property {:?}", prop);
                    if let Some((key, value)) = extract_attribute_key_pairs(prop) {
                        debug!("Parsed property: {:?} = {:?}", key, value);
                        properties.insert(key, value);
                    } else {
                        debug!("Skipping property");
                    }
                }
            }
            Rule::lang => {
                debug!("Parsing language: {:?}", attr.as_str());
                properties.insert(
                    "lang".to_string(),
                    NestedValue::Value(attr.as_str().to_string()),
                );
            }
            Rule::unit_system => {
                debug!("Parsing unit system: {:?}", attr.as_str());
                properties.insert(
                    "unit_system".to_string(),
                    NestedValue::Value(attr.as_str().to_string()),
                );
            }
            _ => {
                debug!(
                    "Unexpected token: Invalid attribute in instruction {:?}",
                    attr.as_rule()
                );
            }
        }
    }
    Some(properties)
}

/// Extract content from any instruction related to any overlay
pub fn extract_content(object: Pair) -> Content {
    let properties: Option<IndexMap<String, NestedValue>> =
        extract_properites_key_pairs(object.clone());
    let attributes: Option<IndexMap<String, NestedValue>> =
        extract_attributes_key_paris(object.clone());

    Content {
        properties,
        attributes,
    }
}

/// Extract flagged attributes
pub(crate) fn extract_flagged_attrs(object: pest::iterators::Pair<'_, Rule>) -> Vec<String> {
    let mut flagged_attrs: Vec<String> = vec![];
    debug!("Parsing flagged attribute");
    for attr in object.into_inner() {
        match attr.as_rule() {
            Rule::list_value => {
                flagged_attrs.push(attr.as_str().to_string());
            }
            _ => {
                debug!(
                    "Unexpected token: Invalid attribute in instruction {:?}",
                    attr.as_rule()
                );
            }
        }
    }
    flagged_attrs
}
