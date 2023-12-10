use std::str::FromStr;

use indexmap::IndexMap;
use log::debug;
use oca_ast::ast::{NestedValue, AttributeType, NestedAttrType, Content};
use crate::ocafile::{Pair, Rule};

pub fn extract_attribute_type(attr_pair: Pair) -> Option<(String, NestedAttrType)> {
    let mut attr_name = String::new();
    let mut attr_type = NestedAttrType::Value(AttributeType::Text);

    debug!("Extracting the attribute type from: {:?}", attr_pair);
    for item in attr_pair.into_inner() {
        match item.as_rule() {
            Rule::attr_key => {
                attr_name = item.as_str().to_string();
                debug!("Extracting attribute key {:?}", attr_name);
            },
            Rule::_attr_type => {
                debug!("Attribute type to parse: {:?}", item);
                if let Some(attr_type_rule) = item.clone().into_inner().next() {
                    match attr_type_rule.as_rule() {
                        Rule::reference => {
                            debug!("Matching referance {:?}", attr_type_rule);
                            attr_type = NestedAttrType::Reference(oca_ast::ast::RefValue::Name(attr_type_rule.as_str().to_string()));
                        },
                        Rule::said => {
                            debug!("Matching said reference: {:?}", attr_type_rule);
                            attr_type = NestedAttrType::Reference(oca_ast::ast::RefValue::Said(attr_type_rule.as_str().to_string()))
                        }
                        Rule::base_attr_type => {
                            debug!("Matching basic attribute type from rule: {}", attr_type_rule);
                            match AttributeType::from_str(attr_type_rule.as_span().as_str()) {
                                Ok(base_attr_type) => {
                                    debug!("Attribute type: {:?}", base_attr_type);
                                    attr_type = NestedAttrType::Value(base_attr_type);
                                }
                                Err(e) => {
                                    panic!("Invalid attribute type {:?}", e);
                                }
                            }
                        }
                        Rule::array_attr_type => {
                            debug!("Matching array attribute type from rule: {:?}", attr_type_rule);
                            if let Some(value) = attr_type_rule.clone().into_inner().next() {
                                match AttributeType::from_str(value.as_span().as_str()) {
                                    Ok(base_attr_type) => {
                                        attr_type = NestedAttrType::Array(Box::new(NestedAttrType::Value(base_attr_type)));
                                    }
                                    Err(e) => {
                                        panic!("Invalid attribute type {:?}", e);
                                    }
                                }
                            }

                        }
                        Rule::ref_array => {
                            debug!("Matching reference array type from rule: {:?}", attr_type_rule);
                            if let Some(value) = attr_type_rule.clone().into_inner().next() {
                                match value.as_rule() {
                                    Rule::reference => {
                                        attr_type = NestedAttrType::Array(Box::new(NestedAttrType::Reference(oca_ast::ast::RefValue::Name(value.as_str().to_string()))));
                                    },
                                    Rule::said => {
                                        attr_type = NestedAttrType::Array(Box::new(NestedAttrType::Reference(oca_ast::ast::RefValue::Said(value.as_str().to_string()))));
                                    },
                                    _ => {
                                        panic!("Invalid reference array value in {:?}", value.as_rule());
                                    }
                                }
                            }
                        }
                        _ => {
                            panic!("Matching referance didn't worked");
                        }
                    }
                }
            },
            _ => {
                panic!("Invalid attribute in {:?}", item.as_rule());
            }
        }
    }
    Some((attr_name, attr_type))
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


pub fn extract_attributes_key_paris(object: Pair) -> Option<IndexMap<String, NestedValue>> {
    let mut attributes: IndexMap<String, NestedValue> = IndexMap::new();

    debug!("Extracting content of the attributes: {:?}", object);
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
            _ => {
                debug!(
                    "Unexpected token: Invalid attribute in instruction {:?}",
                    attr.as_rule()
                );
                return None;
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
    let mut properties: Option<IndexMap<String, NestedValue>> = Some(IndexMap::new());
    let mut attributes: Option<IndexMap<String, NestedValue>> = Some(IndexMap::new());


    properties = extract_properites_key_pairs(object.clone());
    attributes = extract_attributes_key_paris(object.clone());


    Content {
        properties,
        attributes,
    }
}

pub(crate) fn extract_flagged_attrs(object: pest::iterators::Pair<'_, Rule>) -> IndexMap<String, NestedAttrType> {
    todo!()
}
