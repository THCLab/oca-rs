use std::str::FromStr;

use indexmap::IndexMap;
use log::debug;
use oca_bundle::state::attribute::AttributeType;
use ocaast::ast::{Content, NestedValue};
use crate::ocafile::{Pair, Rule};

/// Extract attributes key pairs for ADD and MODIFY command

pub fn extract_attribute_key_pairs(attr_pair: Pair) -> Option<(String, String)> {
    let mut key = String::new();
    let mut value = String::new();

    debug!("Extract the attribute: {:?}", attr_pair);
    for item in attr_pair.into_inner() {
        match item.as_rule() {
            Rule::attr_key => {
                key = item.as_str().to_string();
            }
            Rule::attr_type => match AttributeType::from_str(item.as_span().as_str()) {
                Ok(attr_type) => {
                    debug!("Attribute type: {:?}", attr_type);
                    // value = attr_type.to_string();
                    value = serde_json::to_string(&attr_type).unwrap();
                }
                Err(e) => {
                    panic!("Invalid attribute type {:?}", e);
                }
            },
            Rule::key_value => {
                value = item.as_str().to_string();
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
            Rule::attr_key_pairs => {
                for attr in attr.into_inner() {
                    debug!("Parsing attribute {:?}", attr);
                    if let Some((key, value)) = extract_attribute_key_pairs(attr) {
                        debug!("Parsed attribute: {:?} = {:?}", key, value);
                        // TODO find out how to parse nested objects
                        attributes.insert(key, NestedValue::Value(value));
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
                        properties.insert(key, NestedValue::Value(value));
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
