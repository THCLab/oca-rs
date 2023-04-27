use std::str::FromStr;
pub mod bundle;
use ocaast::ast;
use oca_bundle::state::{oca::OCABox, attribute::{Attribute, AttributeType}};

pub fn add(new: &str, to: &str) -> (Vec<u8>, Vec<u8>) {
    let mut digests: Vec<u8> = Vec::new();
    digests.push(to.len().try_into().unwrap());
    digests.extend(to.as_bytes());
    (new.as_bytes().to_vec(), digests)
}

pub fn add_step(base: &str, op: ast::Command) -> (Vec<u8>, Vec<u8>) {
    let mut digests: Vec<u8> = Vec::new();

    let mut oca = OCABox::new();

    match op.kind {
      ast::CommandType::Add => {
        match op.object_kind {
          ast::ObjectKind::CaptureBase => {
            if let Some(ref content) = op.content {
              if let Some(ref attributes) = content.attributes {
                for (attr_name, attr_type_value) in attributes {
                  let mut attribute = Attribute::new(attr_name.clone());
                  if let ast::NestedValue::Value(attr_value) = attr_type_value {
                    let attribute_type = AttributeType::from_str(attr_value.as_str());
                    if attribute_type.is_ok() {
                      attribute.set_attribute_type(attribute_type.unwrap());
                    }
                    oca.add_attribute(attribute);
                  }
                }
              }
              if let Some(ref properties) = content.properties {
                for (prop_name, prop_value) in properties {
                  if prop_name.eq("classification") {
                    if let ast::NestedValue::Value(value) = prop_value {
                      oca.add_classification(value.clone());
                    }
                  }
                }
              }
            }
          },
          _ => ()
        }
      },
      _ => ()
    }

    println!("{}", serde_json::to_string(&oca.generate_bundle()).unwrap());

    let serialized = serde_json::to_string(&op).unwrap();
    digests.extend::<Vec<u8>>(serialized.try_into().unwrap());
    (base.as_bytes().to_vec(), digests)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indexmap::IndexMap;

    #[test]
    fn test_add_step() {
      let mut attributes = IndexMap::new();
      attributes.insert("test".to_string(), ast::NestedValue::Value("Text".to_string()));

      let mut properties = IndexMap::new();
      properties.insert("classification".to_string(), ast::NestedValue::Value("class".to_string()));

      let command = ast::Command {
        kind: ast::CommandType::Add,
        object_kind: ast::ObjectKind::CaptureBase,
        content: Some(ast::Content {
            attributes: Some(attributes),
            properties: Some(properties),
        }),
      };
      let (base, op) = add_step("hello", command);
      println!("{}", String::from_utf8(op.clone()).unwrap());
      assert_eq!(base, "hello".as_bytes().to_vec());
      assert_eq!(op, vec![3, 5, 119, 111, 114, 108, 100]);
    }
}
