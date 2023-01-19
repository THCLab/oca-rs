use crate::state::oca::OCABuilder;
use std::io::Read;

pub type GenericError = Box<dyn std::error::Error + Sync + Send>;
pub type GenericResult<T> = Result<T, GenericError>;

pub fn load_oca(source: &mut dyn Read) -> GenericResult<OCABuilder> {
    let oca: OCABuilder = serde_json::from_reader(source)?;

    Ok(oca)
}

#[cfg(test)]
mod tests {
    use super::load_oca;
    use crate::state::attribute::{AttributeBuilder, AttributeType};
    use maplit::hashmap;

    #[test]
    fn loads_json_from_str() {
        let data = r#"
{
  "capture_base": {
    "attributes": {
      "n1": "Text",
      "n2": "DateTime",
      "n3": "Reference:sai"
    },
    "classification": "",
    "digest": "ElNWOR0fQbv_J6EL0pJlvCxEpbu4bg1AurHgr_0A7LKc",
    "flagged_attributes": [
      "n1"
    ],
    "type": "spec/capture_base/1.0"
  },
  "overlays": [
    {
      "attribute_character_encoding": {
        "n2": "iso-8859-1"
      },
      "capture_base": "ElNWOR0fQbv_J6EL0pJlvCxEpbu4bg1AurHgr_0A7LKc",
      "default_character_encoding": "utf-8",
      "digest": "E-fCW5Gcnzn4uTBIUUH4Gdl3xRdlDg25cM3UCk3blubU",
      "type": "spec/overlays/character_encoding/1.0"
    },
    {
      "attribute_units": {
        "n1": "cm"
      },
      "capture_base": "ElNWOR0fQbv_J6EL0pJlvCxEpbu4bg1AurHgr_0A7LKc",
      "digest": "E-gA3mYg2RbZcLXuNrWyMNJqCaclfzMrgQogVUlpFcoY",
      "metric_system": "SI",
      "type": "spec/overlays/unit/1.0"
    },
    {
      "attribute_entry_codes": {
        "n1": [
          "op1",
          "op2"
        ]
      },
      "capture_base": "ElNWOR0fQbv_J6EL0pJlvCxEpbu4bg1AurHgr_0A7LKc",
      "digest": "E4L-BukSBsqZoDDIJvw4_gGjAJs5It4UUfiA200lGup0",
      "type": "spec/overlays/entry_code/1.0"
    },
    {
      "attribute_categories": [],
      "attribute_labels": {
        "n1": "Name: ",
        "n2": "Date: ",
        "n3": "Reference: "
      },
      "capture_base": "ElNWOR0fQbv_J6EL0pJlvCxEpbu4bg1AurHgr_0A7LKc",
      "category_labels": {},
      "digest": "EwXoTd4_ZSZMnRmfQGFkXTfw7uMu9z9bnIah2ZM6hPpQ",
      "language": "En",
      "type": "spec/overlays/label/1.0"
    },
    {
      "attribute_information": {
        "n1": "info en"
      },
      "capture_base": "ElNWOR0fQbv_J6EL0pJlvCxEpbu4bg1AurHgr_0A7LKc",
      "digest": "EUcv8Udxqj7pZfHF1XxrIpb01lfRhpRtWdzxVS706EVI",
      "language": "En",
      "type": "spec/overlays/information/1.0"
    },
    {
      "attribute_entries": {
        "n1": {
          "op1": "Option 1",
          "op2": "Option 2"
        }
      },
      "capture_base": "ElNWOR0fQbv_J6EL0pJlvCxEpbu4bg1AurHgr_0A7LKc",
      "digest": "EcNtZGAs1yPlPSVRO38t13sxr1abF67MnqvjTJQGM3jc",
      "language": "En",
      "type": "spec/overlays/entry/1.0"
    },
    {
      "attribute_categories": [],
      "attribute_labels": {
        "n1": "Imię: ",
        "n2": "Data: ",
        "n3": "Referecja: "
      },
      "capture_base": "ElNWOR0fQbv_J6EL0pJlvCxEpbu4bg1AurHgr_0A7LKc",
      "category_labels": {},
      "digest": "Eme3tfHtbrY0nT8ZNEorzM2Nrkdf3PzwFIP1hnAmXkdg",
      "language": "Pl",
      "type": "spec/overlays/label/1.0"
    },
    {
      "attribute_information": {
        "n1": "info pl"
      },
      "capture_base": "ElNWOR0fQbv_J6EL0pJlvCxEpbu4bg1AurHgr_0A7LKc",
      "digest": "EZvWNCh1yj7FSOdyz67WK1n7pq_xyo-M7RjDiFDR4Q7I",
      "language": "Pl",
      "type": "spec/overlays/information/1.0"
    },
    {
      "attribute_entries": {
        "n1": {
          "op1": "Opcja 1",
          "op2": "Opcja 2"
        }
      },
      "capture_base": "ElNWOR0fQbv_J6EL0pJlvCxEpbu4bg1AurHgr_0A7LKc",
      "digest": "EBXzK5l6KiH40PyJSoDjB4WBMpXh6DwvgpLMbZ2jj-Ws",
      "language": "Pl",
      "type": "spec/overlays/entry/1.0"
    },
    {
      "attribute_conditions": {
        "n2": "${0} == 'op1'"
      },
      "attribute_dependencies": {
        "n2": [
          "n1"
        ]
      },
      "capture_base": "ElNWOR0fQbv_J6EL0pJlvCxEpbu4bg1AurHgr_0A7LKc",
      "digest": "EkyIEvDwMete4Y-adBJZcGihh6K2Orswhkv_unFmXzHM",
      "type": "spec/overlays/conditional/1.0"
    },
    {
      "attribute_formats": {
        "n2": "DD/MM/YYYY"
      },
      "capture_base": "ElNWOR0fQbv_J6EL0pJlvCxEpbu4bg1AurHgr_0A7LKc",
      "digest": "EGiJKfiFCIf8Hdt8QLlnksOB9AH6_gcuc0l3gx0aATnM",
      "type": "spec/overlays/format/1.0"
    },
    {
      "capture_base": "ElNWOR0fQbv_J6EL0pJlvCxEpbu4bg1AurHgr_0A7LKc",
      "description": "DL desc",
      "digest": "Eluyyqh9h7TQWJt980o16ZfHSqSKQD2q0q7QtTUiIBPc",
      "language": "En",
      "name": "Driving Licence",
      "type": "spec/overlays/meta/1.0"
    },
    {
      "capture_base": "ElNWOR0fQbv_J6EL0pJlvCxEpbu4bg1AurHgr_0A7LKc",
      "description": "PJ desc",
      "digest": "EfDTiIz_5kHXWQIxw8JmDEou__3pi94c63cMrIRo7nK4",
      "language": "Pl",
      "name": "Prawo Jazdy",
      "type": "spec/overlays/meta/1.0"
    },
    {
      "capture_base": "ElNWOR0fQbv_J6EL0pJlvCxEpbu4bg1AurHgr_0A7LKc",
      "digest": "EfDTiIz_5kHXWQIxw8JmDEou__3pi94c63cMrIRo7nK4",
      "type": "spec/overlays/standard/1.0",
      "attribute_standards": {
          "n1": "urn:ietf:id:123"
      }
    }
  ]
}
        "#;

        let oca_builder_result = load_oca(&mut data.as_bytes());
        assert!(oca_builder_result.is_ok());
        if let Ok(oca_builder) = oca_builder_result {
            let oca = oca_builder
                .add_attribute(
                    AttributeBuilder::new("new_attr".to_string(), AttributeType::Text)
                        .add_label(hashmap! {
                            "En".to_string() => "New: ".to_string(),
                            "Pl".to_string() => "Nowy: ".to_string(),
                        })
                        .build(),
                )
                .finalize();

            assert_eq!(oca.capture_base.attributes.len(), 4);
        }
    }
}
