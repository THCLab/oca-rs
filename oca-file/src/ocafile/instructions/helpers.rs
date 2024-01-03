use std::str::FromStr;

use indexmap::IndexMap;
use log::debug;
use oca_ast::ast::{NestedValue, AttributeType, NestedAttrType, Content, RefValue, attributes::NestedAttrTypeFrame};
use recursion::ExpandableExt;
use said::SelfAddressingIdentifier;
use crate::ocafile::{Pair, Rule};

fn extract_attr_type(input: Pair) -> NestedAttrType {
    NestedAttrType::expand_frames(input, |seed| {
        match seed.as_rule() {
            Rule::array_attr_type => {
                NestedAttrTypeFrame::Array(seed.into_inner().next().unwrap())
            },
            Rule::alias => {
                NestedAttrTypeFrame::Reference(oca_ast::ast::RefValue::Name(seed.as_str().to_string()))
            },
            Rule::said => {
                let said = SelfAddressingIdentifier::from_str(seed.as_str()).unwrap();
                NestedAttrTypeFrame::Reference(RefValue::Said(said))
            },

            Rule::base_attr_type => {
                let attr_type = AttributeType::from_str(seed.as_span().as_str()).unwrap();
                NestedAttrTypeFrame::Value(attr_type)
            },
            Rule::object_attr_type => {
                NestedAttrTypeFrame::Object(extract_object(seed))
            },
            r => {
                panic!("Matching attr type didn't work. Unhandled Rule type: {:?}", r);
            }
        }
    })
}

fn extract_object(input_pair: Pair) -> IndexMap<String, Pair> {
    let mut object_fields = input_pair.into_inner();
    let mut idmap = IndexMap::new();
    while let Some(field) = object_fields.next() {
        let key = field.as_span().as_str().to_owned();
        let value = object_fields.next().unwrap();
        idmap.insert(key, value);
    };
    idmap
}

pub fn extract_attribute(attr_pair: Pair) -> Option<(String, NestedAttrType)> {
    let mut attr_name = String::new();
    let mut attr_type = NestedAttrType::Value(AttributeType::Text);

    debug!("Extracting the attribute type from: {:?}", attr_pair);
    for item in attr_pair.into_inner() {
        match item.as_rule() {
            Rule::attr_key => {
                debug!("Extracting attribute key {:?}", attr_name);
                attr_name = item.as_str().to_string();
            },
            Rule::_attr_type => {
                debug!("Attribute type to parse: {:?}", item);
                let mut inner = item.into_inner();
                let inner_pair = inner.next().unwrap();
                attr_type = extract_attr_type(inner_pair);
            }
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
    let properties: Option<IndexMap<String, NestedValue>> = extract_properites_key_pairs(object.clone());
    let attributes: Option<IndexMap<String, NestedValue>> = extract_attributes_key_paris(object.clone());


    Content {
        properties,
        attributes,
    }
}

pub(crate) fn extract_flagged_attrs(_: pest::iterators::Pair<'_, Rule>) -> IndexMap<String, NestedAttrType> {
    todo!()
}
