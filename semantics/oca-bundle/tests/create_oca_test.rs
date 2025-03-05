use isolang::Language;
use oca_bundle_semantics::state::{
    attribute::{Attribute, AttributeType},
    encoding::Encoding,
    entries::EntriesElement,
    entry_codes::EntryCodes as EntryCodesValue,
    oca::overlay::cardinality::Cardinalitys,
    oca::overlay::character_encoding::CharacterEncodings,
    oca::overlay::conditional::Conditionals,
    oca::overlay::conformance::Conformances,
    oca::overlay::entry::Entries,
    oca::overlay::entry_code::EntryCodes,
    oca::overlay::information::Information,
    oca::overlay::label::Labels,
    oca::overlay::meta::Metas,
    oca::overlay::unit::Units,
    oca::OCABox,
};

#[cfg(feature = "format_overlay")]
use oca_bundle_semantics::state::oca::overlay::format::Formats;

use cascade::cascade;
use maplit::hashmap;

#[test]
fn create_oca() {
    let mut oca = cascade! {
        OCABox::new();
        ..add_meta(Language::Eng, "name".to_string(), "Test".to_string());
        ..add_meta(Language::Eng, "description".to_string(), "Test case OCA".to_string());
    };

    let mut attribute = cascade! {
        Attribute::new("name".to_string());
        ..set_attribute_type(oca_ast_semantics::ast::NestedAttrType::Value(AttributeType::Text));
        ..set_flagged();
        ..set_encoding(Encoding::Utf8);
        ..set_cardinality("1".to_string());
        ..set_conformance("O".to_string());
        ..set_label(isolang::Language::Eng, "Name".to_string());
        ..set_information(isolang::Language::Eng, "name information".to_string());
        ..set_entry_codes(EntryCodesValue::Array(vec!["a".to_string(), "b".to_string()]));
        ..set_entry(isolang::Language::Pol, EntriesElement::Object(hashmap! {
            "a".to_string() => "Opcja A".to_string(),
            "b".to_string() => "Opcja B".to_string(),
        }));
        ..set_entry(isolang::Language::Eng, EntriesElement::Object(hashmap! {
            "a".to_string() => "Option A".to_string(),
            "b".to_string() => "Option B".to_string(),
        }));
        ..set_unit("kg".to_string());
    };
    #[cfg(feature = "format_overlay")]
    attribute.set_format("^[a-zA-Z]*$".to_string());

    oca.add_attribute(attribute);

    let mut attribute_2 = cascade! {
        Attribute::new("age".to_string());
        ..set_attribute_type(oca_ast_semantics::ast::NestedAttrType::Value(AttributeType::Numeric));
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
        ..set_unit("kg".to_string());
    };
    #[cfg(feature = "format_overlay")]
    attribute_2.set_format("^[a-zA-Z]*$".to_string());

    oca.add_attribute(attribute_2);

    let oca_bundle = oca.generate_bundle();
    assert_eq!(oca_bundle.said, oca.generate_bundle().said);
    println!("{:#?}", oca_bundle);

    assert_eq!(oca_bundle.capture_base.attributes.len(), 2);
    assert_eq!(oca_bundle.capture_base.flagged_attributes.len(), 2);

    #[cfg(not(feature = "format_overlay"))]
    assert_eq!(oca_bundle.overlays.len(), 11);
    #[cfg(feature = "format_overlay")]
    assert_eq!(oca_bundle.overlays.len(), 12);

    let serialized_bundle = serde_json::to_string_pretty(&oca_bundle).unwrap();
    println!("{}", serialized_bundle);

    let expected = if cfg!(feature = "format_overlay") {
        r#"{
  "d": "EKt3_1YAWFo2tf2khPqtV6gaI9tvCo8jgdh1uMGQSnB9",
  "capture_base": {
    "d": "EGbBiC4HEJnZhafuhu4wf_lS7zwCg1l1ZIrGoe8JbB4s",
    "type": "spec/capture_base/1.1",
    "attributes": {
      "age": "Numeric",
      "name": "Text"
    },
    "classification": "",
    "flagged_attributes": [
      "age",
      "name"
    ]
  },
  "overlays": {
    "cardinality": {
      "d": "EJ7yumk8wtXkMZMDqCHKZVcEJ8ymjSlEYzCUj7NLUmwU",
      "capture_base": "EGbBiC4HEJnZhafuhu4wf_lS7zwCg1l1ZIrGoe8JbB4s",
      "type": "spec/overlays/cardinality/1.1",
      "attribute_cardinality": {
        "age": "2",
        "name": "1"
      }
    },
    "character_encoding": {
      "d": "EIjeafGrlGLqhVbX-YDh7H6XSV3C7g5qAJotjAo5OCNi",
      "capture_base": "EGbBiC4HEJnZhafuhu4wf_lS7zwCg1l1ZIrGoe8JbB4s",
      "type": "spec/overlays/character_encoding/1.1",
      "attribute_character_encoding": {
        "age": "utf-8",
        "name": "utf-8"
      }
    },
    "conditional": {
      "d": "EP8mHNqOLSfuKpTkzfLI4Oc3JoXVgI9XA3HnAG-_E6e2",
      "capture_base": "EGbBiC4HEJnZhafuhu4wf_lS7zwCg1l1ZIrGoe8JbB4s",
      "type": "spec/overlays/conditional/1.1",
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
      "d": "EOWj3aeNmzg4XIoAcKdUMnlTui5vZgOKK5jIwNGWaWXz",
      "capture_base": "EGbBiC4HEJnZhafuhu4wf_lS7zwCg1l1ZIrGoe8JbB4s",
      "type": "spec/overlays/conformance/1.1",
      "attribute_conformance": {
        "age": "M",
        "name": "O"
      }
    },
    "entry": [
      {
        "d": "EBgZ2JsfGabAVKIZ0Xfal6sLNp84rysKKnAJB9C0Fan5",
        "capture_base": "EGbBiC4HEJnZhafuhu4wf_lS7zwCg1l1ZIrGoe8JbB4s",
        "type": "spec/overlays/entry/1.1",
        "language": "eng",
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
      },
      {
        "d": "ELRBbVw3Awq0UL-RBGWvpHh5bD8B0O7rDj8sV3ujy2dv",
        "capture_base": "EGbBiC4HEJnZhafuhu4wf_lS7zwCg1l1ZIrGoe8JbB4s",
        "type": "spec/overlays/entry/1.1",
        "language": "pol",
        "attribute_entries": {
          "name": {
            "a": "Opcja A",
            "b": "Opcja B"
          }
        }
      }
    ],
    "entry_code": {
      "d": "EJ2w0sektxjHzhaXpIk0i9Pbhang9stT_AbXdOSmHrlD",
      "capture_base": "EGbBiC4HEJnZhafuhu4wf_lS7zwCg1l1ZIrGoe8JbB4s",
      "type": "spec/overlays/entry_code/1.1",
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
    "format": {
      "d": "EDuiqqJ_zdVLcAbb3MThjgYIQ0xxPun9Vm3-kRVM7Mc8",
      "capture_base": "EGbBiC4HEJnZhafuhu4wf_lS7zwCg1l1ZIrGoe8JbB4s",
      "type": "spec/overlays/format/1.1",
      "attribute_formats": {
        "age": "^[a-zA-Z]*$",
        "name": "^[a-zA-Z]*$"
      }
    },
    "information": [
      {
        "d": "EJFEliHs1yi_hcpgwwpHRizYfL8ESsPfLBRcnl2WKDbl",
        "capture_base": "EGbBiC4HEJnZhafuhu4wf_lS7zwCg1l1ZIrGoe8JbB4s",
        "type": "spec/overlays/information/1.1",
        "language": "eng",
        "attribute_information": {
          "age": "age information",
          "name": "name information"
        }
      }
    ],
    "label": [
      {
        "d": "EIbDK-C9BN5SDCuqJ23fEsAFFoPABgtxVUp6TiNgrk6o",
        "capture_base": "EGbBiC4HEJnZhafuhu4wf_lS7zwCg1l1ZIrGoe8JbB4s",
        "type": "spec/overlays/label/1.1",
        "language": "eng",
        "attribute_categories": [],
        "attribute_labels": {
          "age": "Age",
          "name": "Name"
        },
        "category_labels": {}
      }
    ],
    "meta": [
      {
        "d": "ECXWiZUmx8s8OsgfGI6dpMD4RnjPCuk-JVkwuK2aFAVl",
        "capture_base": "EGbBiC4HEJnZhafuhu4wf_lS7zwCg1l1ZIrGoe8JbB4s",
        "type": "spec/overlays/meta/1.1",
        "language": "eng",
        "description": "Test case OCA",
        "name": "Test"
      }
    ],
    "unit": {
      "d": "EErcFaMa5fJ3YjtnG6VWSIUtRVyHAWCHSZfwtjz0sO2M",
      "capture_base": "EGbBiC4HEJnZhafuhu4wf_lS7zwCg1l1ZIrGoe8JbB4s",
      "type": "spec/overlays/unit/1.1",
      "attribute_unit": {
        "age": "kg",
        "name": "kg"
      }
    }
  }
}"#
    } else {
        r#"{
  "d": "EM0w7t4KWd7GGiteUCm6frLXJ_k67Ir6z0kcfVcM9D70",
  "capture_base": {
    "d": "EGbBiC4HEJnZhafuhu4wf_lS7zwCg1l1ZIrGoe8JbB4s",
    "type": "spec/capture_base/1.1",
    "attributes": {
      "age": "Numeric",
      "name": "Text"
    },
    "classification": "",
    "flagged_attributes": [
      "age",
      "name"
    ]
  },
  "overlays": {
    "cardinality": {
      "d": "EJ7yumk8wtXkMZMDqCHKZVcEJ8ymjSlEYzCUj7NLUmwU",
      "capture_base": "EGbBiC4HEJnZhafuhu4wf_lS7zwCg1l1ZIrGoe8JbB4s",
      "type": "spec/overlays/cardinality/1.1",
      "attribute_cardinality": {
        "age": "2",
        "name": "1"
      }
    },
    "character_encoding": {
      "d": "EIjeafGrlGLqhVbX-YDh7H6XSV3C7g5qAJotjAo5OCNi",
      "capture_base": "EGbBiC4HEJnZhafuhu4wf_lS7zwCg1l1ZIrGoe8JbB4s",
      "type": "spec/overlays/character_encoding/1.1",
      "attribute_character_encoding": {
        "age": "utf-8",
        "name": "utf-8"
      }
    },
    "conditional": {
      "d": "EP8mHNqOLSfuKpTkzfLI4Oc3JoXVgI9XA3HnAG-_E6e2",
      "capture_base": "EGbBiC4HEJnZhafuhu4wf_lS7zwCg1l1ZIrGoe8JbB4s",
      "type": "spec/overlays/conditional/1.1",
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
      "d": "EOWj3aeNmzg4XIoAcKdUMnlTui5vZgOKK5jIwNGWaWXz",
      "capture_base": "EGbBiC4HEJnZhafuhu4wf_lS7zwCg1l1ZIrGoe8JbB4s",
      "type": "spec/overlays/conformance/1.1",
      "attribute_conformance": {
        "age": "M",
        "name": "O"
      }
    },
    "entry": [
      {
        "d": "EBgZ2JsfGabAVKIZ0Xfal6sLNp84rysKKnAJB9C0Fan5",
        "capture_base": "EGbBiC4HEJnZhafuhu4wf_lS7zwCg1l1ZIrGoe8JbB4s",
        "type": "spec/overlays/entry/1.1",
        "language": "eng",
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
      },
      {
        "d": "ELRBbVw3Awq0UL-RBGWvpHh5bD8B0O7rDj8sV3ujy2dv",
        "capture_base": "EGbBiC4HEJnZhafuhu4wf_lS7zwCg1l1ZIrGoe8JbB4s",
        "type": "spec/overlays/entry/1.1",
        "language": "pol",
        "attribute_entries": {
          "name": {
            "a": "Opcja A",
            "b": "Opcja B"
          }
        }
      }
    ],
    "entry_code": {
      "d": "EJ2w0sektxjHzhaXpIk0i9Pbhang9stT_AbXdOSmHrlD",
      "capture_base": "EGbBiC4HEJnZhafuhu4wf_lS7zwCg1l1ZIrGoe8JbB4s",
      "type": "spec/overlays/entry_code/1.1",
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
    "information": [
      {
        "d": "EJFEliHs1yi_hcpgwwpHRizYfL8ESsPfLBRcnl2WKDbl",
        "capture_base": "EGbBiC4HEJnZhafuhu4wf_lS7zwCg1l1ZIrGoe8JbB4s",
        "type": "spec/overlays/information/1.1",
        "language": "eng",
        "attribute_information": {
          "age": "age information",
          "name": "name information"
        }
      }
    ],
    "label": [
      {
        "d": "EIbDK-C9BN5SDCuqJ23fEsAFFoPABgtxVUp6TiNgrk6o",
        "capture_base": "EGbBiC4HEJnZhafuhu4wf_lS7zwCg1l1ZIrGoe8JbB4s",
        "type": "spec/overlays/label/1.1",
        "language": "eng",
        "attribute_categories": [],
        "attribute_labels": {
          "age": "Age",
          "name": "Name"
        },
        "category_labels": {}
      }
    ],
    "meta": [
      {
        "d": "ECXWiZUmx8s8OsgfGI6dpMD4RnjPCuk-JVkwuK2aFAVl",
        "capture_base": "EGbBiC4HEJnZhafuhu4wf_lS7zwCg1l1ZIrGoe8JbB4s",
        "type": "spec/overlays/meta/1.1",
        "language": "eng",
        "description": "Test case OCA",
        "name": "Test"
      }
    ],
    "unit": {
      "d": "EErcFaMa5fJ3YjtnG6VWSIUtRVyHAWCHSZfwtjz0sO2M",
      "capture_base": "EGbBiC4HEJnZhafuhu4wf_lS7zwCg1l1ZIrGoe8JbB4s",
      "type": "spec/overlays/unit/1.1",
      "attribute_unit": {
        "age": "kg",
        "name": "kg"
      }
    }
  }
}"#
    };

    assert_eq!(serialized_bundle, expected);
}
