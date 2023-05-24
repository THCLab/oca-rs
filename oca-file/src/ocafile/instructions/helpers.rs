use std::str::FromStr;

use indexmap::IndexMap;
use log::debug;
use oca_bundle::state::attribute::AttributeType;
use ocaast::ast::{Content, NestedValue};
use crate::ocafile::{Pair, Rule};

/// Extract attributes key pairs for ADD and MODIFY command

pub fn extract_attribute_key_pairs(attr_pair: Pair) -> Option<(String, NestedValue)> {
    let mut key = String::new();
    let mut value = NestedValue::Value(String::new());

    debug!("Extract the attribute: {:?}", attr_pair);
    for item in attr_pair.into_inner() {
        match item.as_rule() {
            Rule::attr_key => {
                key = item.as_str().to_string();
            }
            Rule::attr_type => match AttributeType::from_str(item.as_span().as_str()) {
                Ok(attr_type) => {
                    debug!("Attribute type: {:?}", attr_type);
                    if let Ok(serde_json::Value::String(v)) = serde_json::to_value(attr_type) {
                        value = NestedValue::Value(v);
                    } else {
                        panic!("Invalid attribute type {:?}", attr_type);
                    }
                }
                Err(e) => {
                    panic!("Invalid attribute type {:?}", e);
                }
            },
            Rule::key_value => {
                if let Some(nested_item) = item.clone().into_inner().next() {
                    match nested_item.as_rule() {
                        Rule::string => {
                            value = NestedValue::Value(nested_item.clone().into_inner().last().unwrap().as_span().as_str().to_string());
                        }
                        _ => {
                            value = NestedValue::Value(item.as_str().to_string());
                        }
                    }
                }
            },
            Rule::entry_code_key_value => {
                if let Some(entry_code_item) = item.clone().into_inner().next() {
                    match entry_code_item.as_rule() {
                        Rule::key_value => {
                            if let Some(nested_item) = entry_code_item.clone().into_inner().next() {
                                match nested_item.as_rule() {
                                    Rule::string => {
                                        value = NestedValue::Value(nested_item.clone().into_inner().last().unwrap().as_span().as_str().to_string());
                                    }
                                    _ => {
                                        value = NestedValue::Value(item.as_str().to_string());
                                    }
                                }
                            }
                        },
                        Rule::entry_code_list => {
                            let mut entry_codes = Vec::new();
                            for el in entry_code_item.clone().into_inner() {
                                match el.as_rule() {
                                    Rule::string => {
                                        entry_codes.push(NestedValue::Value(el.clone().into_inner().last().unwrap().as_span().as_str().to_string()));
                                    },
                                    _ => {
                                        panic!("Invalid entry code value in {:?}", el.as_rule());
                                    }
                                }
                            }
                            value = NestedValue::Array(entry_codes);
                        },
                        _ => {
                            panic!("Invalid entry code value in {:?}", entry_code_item.as_rule());
                        }
                    }
                }
            },
            Rule::entry_key => {
                if let Some(nested_item) = item.clone().into_inner().next() {
                    if let Rule::string = nested_item.as_rule() {
                        key = nested_item.clone().into_inner().last().unwrap().as_span().as_str().to_string();
                    }
                }
            }
            Rule::entry_value => {
                if let Some(nested_item) = item.clone().into_inner().next() {
                    if let Rule::string = nested_item.as_rule() {
                        value = NestedValue::Value(nested_item.clone().into_inner().last().unwrap().as_span().as_str().to_string());
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
                                        value = NestedValue::Value(nested_item.clone().into_inner().last().unwrap().as_span().as_str().to_string());
                                    }
                                    _ => {
                                        value = NestedValue::Value(item.as_str().to_string());
                                    }
                                }
                            }
                        },
                        Rule::entry_object => {
                            let mut entries = IndexMap::new();
                            for el in entry_item.clone().into_inner() {
                                let (entry_key, entry_value) = extract_attribute_key_pairs(el).unwrap();
                                entries.insert(entry_key, entry_value);
                            }
                            value = NestedValue::Object(entries);
                        },
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

// Extract content from instruction for ADD and MODIFY command

pub fn extract_content(object: Pair) -> Option<Content> {
    let mut properties: IndexMap<String, NestedValue> = IndexMap::new();
    let mut attributes: IndexMap<String, NestedValue> = IndexMap::new();

    debug!("Into the object: {:?}", object);
    for attr in object.into_inner() {
        debug!("Inside the object: {:?}", attr);
        match attr.as_rule() {
            Rule::attr_key_pairs |
            Rule::attr_entry_code_key_pairs |
            Rule::attr_entry_key_pairs => {
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
            Rule::prop_key_pairs => {
                for prop in attr.into_inner() {
                    debug!("Parsing property {:?}", prop);
                    if let Some((key, value)) = extract_attribute_key_pairs(prop) {
                        debug!("Parsed property: {:?} = {:?}", key, value);
                        // TODO find out how to parse nested objects
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
                return None;
            }
        }
    }

    Some(Content {
        properties: Some(properties),
        attributes: Some(attributes),
    })
}
