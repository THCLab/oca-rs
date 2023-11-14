use isolang::Language;
use oca_bundle::state::{
    attribute::{Attribute, AttributeType},
    entry_codes::EntryCodes as EntryCodesValue,
    entries::EntriesElement,
    encoding::Encoding,
    oca::OCABox,
    oca::overlay::meta::Metas,
    oca::overlay::character_encoding::CharacterEncodings,
    oca::overlay::conditional::Conditionals,
    oca::overlay::conformance::Conformances,
    oca::overlay::cardinality::Cardinalitys,
    oca::overlay::entry_code::EntryCodes,
    oca::overlay::entry::Entries,
    oca::overlay::label::Labels,
    oca::overlay::information::Information,
    oca::overlay::unit::{Unit, AttributeUnit, MeasurementSystem, MeasurementUnit, MetricUnit},
    /* oca::overlay::form_layout::FormLayouts,
    oca::overlay::credential_layout::CredentialLayouts; */
};

#[cfg(feature = "format_overlay")]
use oca_bundle::state::oca::overlay::format::Formats;

use cascade::cascade;
use maplit::hashmap;

#[test]
fn create_oca() {
    /* let form_layout = r#"
elements:
    - type: "test"
    "#;
    let credential_layout = r#"
version: "1.0"
pages:
    - config:
        name: "test"
      elements:
        - type: "test"
    "#; */
    let mut oca = cascade! {
        OCABox::new();
        ..add_meta(Language::Eng, "name".to_string(), "Test".to_string());
        ..add_meta(Language::Eng, "description".to_string(), "Test case OCA".to_string());
        /* ..add_form_layout(form_layout.to_string());
        ..add_credential_layout(credential_layout.to_string()); */
    };

    let mut attribute = cascade! {
        Attribute::new("name".to_string());
        ..set_attribute_type(AttributeType::Text);
        ..set_flagged();
        ..set_encoding(Encoding::Utf8);
        ..set_cardinality("1".to_string());
        ..set_conformance("O".to_string());
        ..set_label(isolang::Language::Eng, "Name".to_string());
        ..set_information(isolang::Language::Eng, "name information".to_string());
        ..set_entry_codes(EntryCodesValue::Array(vec!["a".to_string(), "b".to_string()]));
        ..set_entry(isolang::Language::Eng, EntriesElement::Object(hashmap! {
            "a".to_string() => "Option A".to_string(),
            "b".to_string() => "Option B".to_string(),
        }));
        ..set_unit(AttributeUnit { measurement_system: MeasurementSystem::Metric, unit: MeasurementUnit::Metric(MetricUnit::Kilogram) });
    };
    #[cfg(feature = "format_overlay")]
    attribute.set_format("^[a-zA-Z]*$".to_string());

    oca.add_attribute(attribute);

    let mut attribute_2 = cascade! {
        Attribute::new("age".to_string());
        ..set_attribute_type(AttributeType::Numeric);
        ..set_flagged();
        ..set_encoding(Encoding::Utf8);
        ..set_cardinality("2".to_string());
        ..set_conformance("M".to_string());
        ..set_condition("${name} ~= nil and ${name} ~= ''".to_string());
        ..set_label(isolang::Language::Eng, "Age".to_string());
        ..set_information(isolang::Language::Eng, "age information".to_string());
        ..set_entry_codes(EntryCodesValue::Array(vec!["a".to_string(), "b".to_string()]));
        ..set_entry(isolang::Language::Eng, EntriesElement::Object(hashmap! {
            "a".to_string() => "Option A".to_string(),
            "b".to_string() => "Option B".to_string(),
        }));
        ..set_unit(AttributeUnit { measurement_system: MeasurementSystem::Metric, unit: MeasurementUnit::Metric(MetricUnit::Kilogram) });
    };
    #[cfg(feature = "format_overlay")]
    attribute_2.set_format("^[a-zA-Z]*$".to_string());

    oca.add_attribute(attribute_2);

    let oca_bundle = oca.generate_bundle();
    assert_eq!(oca_bundle.said, oca.generate_bundle().said);

    assert_eq!(oca_bundle.capture_base.attributes.len(), 2);
    assert_eq!(oca_bundle.capture_base.flagged_attributes.len(), 2);

    #[cfg(not(feature = "format_overlay"))]
    assert_eq!(oca_bundle.overlays.len(), 10);
    #[cfg(feature = "format_overlay")]
    assert_eq!(oca_bundle.overlays.len(), 11);

    let serialized_bundle = serde_json::to_string_pretty(&oca_bundle).unwrap();

    let expected = if cfg!(feature = "format_overlay") {
r#"{
  "d": "EP3_9m8B2BqPXrF2i0_01zsvjsuMr8SQOuua32KT1RO-",
  "capture_base": {
    "d": "EEDq_Ml2WZox89ROgdZXOWUf2Q3Dsv9xB198uJs5ZjZF",
    "type": "spec/capture_base/1.0",
    "classification": "",
    "attributes": {
      "age": "Numeric",
      "name": "Text"
    },
    "flagged_attributes": [
      "age",
      "name"
    ]
  },
  "overlays": {
    "character_encoding": {
      "d": "ECD2IVxrwPzTnGGZ9a21hC117X_txPZNNA3B1fp5AWUv",
      "type": "spec/overlays/character_encoding/1.0",
      "capture_base": "EEDq_Ml2WZox89ROgdZXOWUf2Q3Dsv9xB198uJs5ZjZF",
      "attribute_character_encoding": {
        "age": "utf-8",
        "name": "utf-8"
      }
    },
    "format": {
      "d": "EHCh4CCiNEjHnRKXt9Kmj1NhGv488Gdnh_HIQIAW8Joe",
      "type": "spec/overlays/format/1.0",
      "capture_base": "EEDq_Ml2WZox89ROgdZXOWUf2Q3Dsv9xB198uJs5ZjZF",
      "attribute_formats": {
        "age": "^[a-zA-Z]*$",
        "name": "^[a-zA-Z]*$"
      }
    },
    "meta": [
      {
        "d": "EAHZWCJd_z91C4Q2gtdgTOM3ht1BmAfcCv_vAliD9gmI",
        "language": "eng",
        "type": "spec/overlays/meta/1.0",
        "capture_base": "EEDq_Ml2WZox89ROgdZXOWUf2Q3Dsv9xB198uJs5ZjZF",
        "description": "Test case OCA",
        "name": "Test"
      }
    ],
    "label": [
      {
        "d": "EPFExRy1gpkJjkpUUI1HGdZk9OXp_vZG6kHw8dEJ5wwB",
        "language": "eng",
        "type": "spec/overlays/label/1.0",
        "capture_base": "EEDq_Ml2WZox89ROgdZXOWUf2Q3Dsv9xB198uJs5ZjZF",
        "attribute_labels": {
          "age": "Age",
          "name": "Name"
        },
        "attribute_categories": [],
        "category_labels": {}
      }
    ],
    "information": [
      {
        "d": "EBcQK4EVV3T7jFhMQV1pWv27pvAHnO8fKamnVd9nXJ4P",
        "language": "eng",
        "type": "spec/overlays/information/1.0",
        "capture_base": "EEDq_Ml2WZox89ROgdZXOWUf2Q3Dsv9xB198uJs5ZjZF",
        "attribute_information": {
          "age": "age information",
          "name": "name information"
        }
      }
    ],
    "conditional": {
      "d": "EAOz_gKx2AdkCySOoYwa_HUejInHuJjJvS_DMTJt1oQW",
      "type": "spec/overlays/conditional/1.0",
      "capture_base": "EEDq_Ml2WZox89ROgdZXOWUf2Q3Dsv9xB198uJs5ZjZF",
      "attribute_conditions": {
        "age": "${0} ~= nil and ${0} ~= ''"
      },
      "attribute_dependencies": {
        "age": [
          "name"
        ]
      }
    },
    "conformance": {
      "d": "EMhtk2tC1Molaq4n_LeEVuthTfrWzID91uXebjLxRSXj",
      "type": "spec/overlays/conformance/1.0",
      "capture_base": "EEDq_Ml2WZox89ROgdZXOWUf2Q3Dsv9xB198uJs5ZjZF",
      "attribute_conformance": {
        "age": "M",
        "name": "O"
      }
    },
    "entry_code": {
      "d": "EAk0iUaBlTgFseZtS7Um4-HoVSPCsVlAV1t8k-XVJ664",
      "type": "spec/overlays/entry_code/1.0",
      "capture_base": "EEDq_Ml2WZox89ROgdZXOWUf2Q3Dsv9xB198uJs5ZjZF",
      "attribute_entry_codes": {
        "age": [
          "a",
          "b"
        ],
        "name": [
          "a",
          "b"
        ]
      }
    },
    "entry": [
      {
        "d": "EN1D4e4aFBECgnNrhmfxSO2xje8STURokBJ4mQ59Pj_a",
        "language": "eng",
        "type": "spec/overlays/entry/1.0",
        "capture_base": "EEDq_Ml2WZox89ROgdZXOWUf2Q3Dsv9xB198uJs5ZjZF",
        "attribute_entries": {
          "age": {
            "a": "Option A",
            "b": "Option B"
          },
          "name": {
            "a": "Option A",
            "b": "Option B"
          }
        }
      }
    ],
    "cardinality": {
      "d": "EHO515sS5rv1sAiytFtATiAFHydLsQOLSZaTNCL5hKl8",
      "type": "spec/overlays/cardinality/1.0",
      "capture_base": "EEDq_Ml2WZox89ROgdZXOWUf2Q3Dsv9xB198uJs5ZjZF",
      "attribute_cardinality": {
        "age": "2",
        "name": "1"
      }
    },
    "unit": {
      "d": "EGeb2Uu_EKrMMaACGWSgHey5Ur8LJT6RlNatIeaPDZ31",
      "type": "spec/overlays/unit/1.0",
      "capture_base": "EEDq_Ml2WZox89ROgdZXOWUf2Q3Dsv9xB198uJs5ZjZF",
      "measurement_system": "metric",
      "attribute_units": {
        "age": "kilogram",
        "name": "kilogram"
      }
    }
  }
}"#
    } else {

r#"{
  "d": "EP0kXbXniRBITVqKmHihZZ5F7Ryb8_17qDfSUyoAoaCX",
  "capture_base": {
    "d": "EEDq_Ml2WZox89ROgdZXOWUf2Q3Dsv9xB198uJs5ZjZF",
    "type": "spec/capture_base/1.0",
    "classification": "",
    "attributes": {
      "age": "Numeric",
      "name": "Text"
    },
    "flagged_attributes": [
      "age",
      "name"
    ]
  },
  "overlays": {
    "character_encoding": {
      "d": "ECD2IVxrwPzTnGGZ9a21hC117X_txPZNNA3B1fp5AWUv",
      "type": "spec/overlays/character_encoding/1.0",
      "capture_base": "EEDq_Ml2WZox89ROgdZXOWUf2Q3Dsv9xB198uJs5ZjZF",
      "attribute_character_encoding": {
        "age": "utf-8",
        "name": "utf-8"
      }
    },
    "meta": [
      {
        "d": "EAHZWCJd_z91C4Q2gtdgTOM3ht1BmAfcCv_vAliD9gmI",
        "language": "eng",
        "type": "spec/overlays/meta/1.0",
        "capture_base": "EEDq_Ml2WZox89ROgdZXOWUf2Q3Dsv9xB198uJs5ZjZF",
        "description": "Test case OCA",
        "name": "Test"
      }
    ],
    "label": [
      {
        "d": "EPFExRy1gpkJjkpUUI1HGdZk9OXp_vZG6kHw8dEJ5wwB",
        "language": "eng",
        "type": "spec/overlays/label/1.0",
        "capture_base": "EEDq_Ml2WZox89ROgdZXOWUf2Q3Dsv9xB198uJs5ZjZF",
        "attribute_labels": {
          "age": "Age",
          "name": "Name"
        },
        "attribute_categories": [],
        "category_labels": {}
      }
    ],
    "information": [
      {
        "d": "EBcQK4EVV3T7jFhMQV1pWv27pvAHnO8fKamnVd9nXJ4P",
        "language": "eng",
        "type": "spec/overlays/information/1.0",
        "capture_base": "EEDq_Ml2WZox89ROgdZXOWUf2Q3Dsv9xB198uJs5ZjZF",
        "attribute_information": {
          "age": "age information",
          "name": "name information"
        }
      }
    ],
    "conditional": {
      "d": "EAOz_gKx2AdkCySOoYwa_HUejInHuJjJvS_DMTJt1oQW",
      "type": "spec/overlays/conditional/1.0",
      "capture_base": "EEDq_Ml2WZox89ROgdZXOWUf2Q3Dsv9xB198uJs5ZjZF",
      "attribute_conditions": {
        "age": "${0} ~= nil and ${0} ~= ''"
      },
      "attribute_dependencies": {
        "age": [
          "name"
        ]
      }
    },
    "conformance": {
      "d": "EMhtk2tC1Molaq4n_LeEVuthTfrWzID91uXebjLxRSXj",
      "type": "spec/overlays/conformance/1.0",
      "capture_base": "EEDq_Ml2WZox89ROgdZXOWUf2Q3Dsv9xB198uJs5ZjZF",
      "attribute_conformance": {
        "age": "M",
        "name": "O"
      }
    },
    "entry_code": {
      "d": "EAk0iUaBlTgFseZtS7Um4-HoVSPCsVlAV1t8k-XVJ664",
      "type": "spec/overlays/entry_code/1.0",
      "capture_base": "EEDq_Ml2WZox89ROgdZXOWUf2Q3Dsv9xB198uJs5ZjZF",
      "attribute_entry_codes": {
        "age": [
          "a",
          "b"
        ],
        "name": [
          "a",
          "b"
        ]
      }
    },
    "entry": [
      {
        "d": "EN1D4e4aFBECgnNrhmfxSO2xje8STURokBJ4mQ59Pj_a",
        "language": "eng",
        "type": "spec/overlays/entry/1.0",
        "capture_base": "EEDq_Ml2WZox89ROgdZXOWUf2Q3Dsv9xB198uJs5ZjZF",
        "attribute_entries": {
          "age": {
            "a": "Option A",
            "b": "Option B"
          },
          "name": {
            "a": "Option A",
            "b": "Option B"
          }
        }
      }
    ],
    "cardinality": {
      "d": "EHO515sS5rv1sAiytFtATiAFHydLsQOLSZaTNCL5hKl8",
      "type": "spec/overlays/cardinality/1.0",
      "capture_base": "EEDq_Ml2WZox89ROgdZXOWUf2Q3Dsv9xB198uJs5ZjZF",
      "attribute_cardinality": {
        "age": "2",
        "name": "1"
      }
    },
    "unit": {
      "d": "EGeb2Uu_EKrMMaACGWSgHey5Ur8LJT6RlNatIeaPDZ31",
      "type": "spec/overlays/unit/1.0",
      "capture_base": "EEDq_Ml2WZox89ROgdZXOWUf2Q3Dsv9xB198uJs5ZjZF",
      "measurement_system": "metric",
      "attribute_units": {
        "age": "kilogram",
        "name": "kilogram"
      }
    }
  }
}"#
    };

    assert_eq!(serialized_bundle, expected);
}
