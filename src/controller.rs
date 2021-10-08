use crate::state::oca::OCA;
use std::io::Read;

pub type GenericError = Box<dyn std::error::Error + Sync + Send>;
pub type GenericResult<T> = Result<T, GenericError>;

pub fn load_oca(source: &mut dyn Read) -> GenericResult<OCA> {
    let oca: OCA = serde_json::from_reader(source)?;

    Ok(oca)
}

#[cfg(test)]
mod tests {
    use super::load_oca;
    use crate::state::{
        attribute::{Attribute, AttributeType},
        language::Language,
    };
    use maplit::hashmap;

    #[test]
    fn loads_json_from_str() {
        let data = r#"
{
  "capture_base": {
    "type": "spec/capture_base/1.0",
    "classification": "classification",
    "attributes": { "n1": "Text", "n2": "Date" },
    "pii": [ "n1" ]
  },
  "overlays": [
    {
      "capture_base": "EP20A21C9EHJu8fOOrjV4ywgn6bGFQbQ3cfWFwWL57io",
      "type": "spec/overalys/character_encoding/1.0",
      "default_character_encoding": "utf-8",
      "attr_character_encoding": { "n2": "iso-8859-1" }
    },
    {
      "capture_base": "EP20A21C9EHJu8fOOrjV4ywgn6bGFQbQ3cfWFwWL57io",
      "type": "spec/overalys/unit/1.0",
      "attr_units": { "n1": "days" }
    },
    {
      "capture_base": "EP20A21C9EHJu8fOOrjV4ywgn6bGFQbQ3cfWFwWL57io",
      "type": "spec/overalys/entry_code/1.0",
      "attr_entry_codes": { "n1": [ "op1", "op2" ] }
    },
    {
      "capture_base": "EP20A21C9EHJu8fOOrjV4ywgn6bGFQbQ3cfWFwWL57io",
      "type": "spec/overalys/label/1.0",
      "language": "pl_PL",
      "attr_labels": { "n1": "Imię: ", "n2": "Data: " },
      "attr_categories": [ "_cat-1_" ],
      "cat_labels": { "_cat-1_": "Category 1" },
      "cat_attributes": { "_cat-1_": [ "n1", "n2" ] }
    },
    {
      "capture_base": "EP20A21C9EHJu8fOOrjV4ywgn6bGFQbQ3cfWFwWL57io",
      "type": "spec/overalys/information/1.0",
      "language": "pl_PL",
      "attr_information": { "n1": "info pl" }
    },
    {
      "capture_base": "EP20A21C9EHJu8fOOrjV4ywgn6bGFQbQ3cfWFwWL57io",
      "type": "spec/overalys/entry/1.0",
      "language": "pl_PL",
      "attr_entries": { "n1": { "op2": "Opcja 2", "op1": "Opcja 1" } }
    },
    {
      "capture_base": "EP20A21C9EHJu8fOOrjV4ywgn6bGFQbQ3cfWFwWL57io",
      "type": "spec/overalys/label/1.0",
      "language": "en_EN",
      "attr_labels": { "n2": "Date: ", "n1": "Name: " },
      "attr_categories": [ "_cat-1_" ],
      "cat_labels": { "_cat-1_": "Category 1" },
      "cat_attributes": { "_cat-1_": [ "n1", "n2" ] }
    },
    {
      "capture_base": "EP20A21C9EHJu8fOOrjV4ywgn6bGFQbQ3cfWFwWL57io",
      "type": "spec/overalys/information/1.0",
      "language": "en_EN",
      "attr_information": { "n1": "info en" }
    },
    {
      "capture_base": "EP20A21C9EHJu8fOOrjV4ywgn6bGFQbQ3cfWFwWL57io",
      "type": "spec/overalys/entry/1.0",
      "language": "en_EN",
      "attr_entries": { "n1": { "op1": "Option 1", "op2": "Option 2" } }
    },
    {
      "capture_base": "EP20A21C9EHJu8fOOrjV4ywgn6bGFQbQ3cfWFwWL57io",
      "type": "spec/overalys/format/1.0",
      "attr_formats": { "n2": "DD/MM/YYYY" }
    },
    {
      "capture_base": "EP20A21C9EHJu8fOOrjV4ywgn6bGFQbQ3cfWFwWL57io",
      "type": "spec/overalys/meta/1.0",
      "language": "en_EN",
      "name": "Driving Licence",
      "description": "DL desc"
    },
    {
      "capture_base": "EP20A21C9EHJu8fOOrjV4ywgn6bGFQbQ3cfWFwWL57io",
      "type": "spec/overalys/meta/1.0",
      "language": "pl_PL",
      "name": "Prawo Jazdy",
      "description": "PJ desc"
    }
  ]
}
        "#;

        let mut oca = load_oca(&mut data.as_bytes()).unwrap();
        oca = oca
            .add_attribute(
                Attribute::new("new_attr".to_string(), AttributeType::Text).add_label(hashmap! {
                    Language::En => "New: ".to_string(),
                    Language::Pl => "Nowy: ".to_string(),
                }),
            )
            .finalize();

        assert_eq!(oca.capture_base.attributes.len(), 3);
    }
}
