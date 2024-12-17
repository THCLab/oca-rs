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
  "d": "EPOgFUo_zOWuyYJqI7eOoXuWZFIxTaf-MlhcsEnU-B6N",
  "capture_base": {
    "d": "EFOnwfWsPxeHq27ToWKFdFjhawS8j_Ol29ULkvPw4uFg",
    "type": "spec/capture_base/1.0",
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
      "d": "EESpBo5wXPBEkFKtsmLL_Vj_1h9nufxvfF8O4eQWbIdR",
      "capture_base": "EFOnwfWsPxeHq27ToWKFdFjhawS8j_Ol29ULkvPw4uFg",
      "type": "spec/overlays/cardinality/1.0",
      "attribute_cardinality": {
        "age": "2",
        "name": "1"
      }
    },
    "character_encoding": {
      "d": "EHIpSMAXHFoP2UiGbjDaMhlLgU9ggZPAsDsh-M9W1MgT",
      "capture_base": "EFOnwfWsPxeHq27ToWKFdFjhawS8j_Ol29ULkvPw4uFg",
      "type": "spec/overlays/character_encoding/1.0",
      "attribute_character_encoding": {
        "age": "utf-8",
        "name": "utf-8"
      }
    },
    "conditional": {
      "d": "EKds5IULUlcCnNq5o8DSBll3xSdSoI0AsWDpR-2V9-is",
      "capture_base": "EFOnwfWsPxeHq27ToWKFdFjhawS8j_Ol29ULkvPw4uFg",
      "type": "spec/overlays/conditional/1.0",
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
      "d": "EMQIjOAGWkg5CVBhOpm6L_i82dndhSJ6anE1KnHtEYBy",
      "capture_base": "EFOnwfWsPxeHq27ToWKFdFjhawS8j_Ol29ULkvPw4uFg",
      "type": "spec/overlays/conformance/1.0",
      "attribute_conformance": {
        "age": "M",
        "name": "O"
      }
    },
    "entry": [
      {
        "d": "EB4lwWLx0giSMadcal043Uv2f7kAQNqAqp-Cnka03El4",
        "capture_base": "EFOnwfWsPxeHq27ToWKFdFjhawS8j_Ol29ULkvPw4uFg",
        "type": "spec/overlays/entry/1.0",
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
        "d": "EOVbtQZXFZv2lV7eG_1bUvKabGAolJQK_cy7PHB3nJqA",
        "capture_base": "EFOnwfWsPxeHq27ToWKFdFjhawS8j_Ol29ULkvPw4uFg",
        "type": "spec/overlays/entry/1.0",
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
      "d": "EAax_SvkKilHfJ83UoZxCX6219om1_8_nMXrQCp8PSCg",
      "capture_base": "EFOnwfWsPxeHq27ToWKFdFjhawS8j_Ol29ULkvPw4uFg",
      "type": "spec/overlays/entry_code/1.0",
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
      "d": "EDDxJu5ek2OCvxeT1sXRMyA7oH1ML6DoXSNGPA4Te0LK",
      "capture_base": "EFOnwfWsPxeHq27ToWKFdFjhawS8j_Ol29ULkvPw4uFg",
      "type": "spec/overlays/format/1.0",
      "attribute_formats": {
        "age": "^[a-zA-Z]*$",
        "name": "^[a-zA-Z]*$"
      }
    },
    "information": [
      {
        "d": "EEVT-1J1KbjV-B1IUSmSGuNd4buqCwT7MoslwCsGn4z2",
        "capture_base": "EFOnwfWsPxeHq27ToWKFdFjhawS8j_Ol29ULkvPw4uFg",
        "type": "spec/overlays/information/1.0",
        "language": "eng",
        "attribute_information": {
          "age": "age information",
          "name": "name information"
        }
      }
    ],
    "label": [
      {
        "d": "EC-Mwbd8PevoVTarSK1zcqDnlkfK1po-CyyM19da3SYw",
        "capture_base": "EFOnwfWsPxeHq27ToWKFdFjhawS8j_Ol29ULkvPw4uFg",
        "type": "spec/overlays/label/1.0",
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
        "d": "EHyoO8skj6nMW_gSbPcdlhUzbRS1SeefJCj4FDjXQXSZ",
        "capture_base": "EFOnwfWsPxeHq27ToWKFdFjhawS8j_Ol29ULkvPw4uFg",
        "type": "spec/overlays/meta/1.0",
        "language": "eng",
        "description": "Test case OCA",
        "name": "Test"
      }
    ],
    "unit": {
      "d": "EPK3PAQ_ZP1Rd6xUyZlXMFBBXx4MzU1c_3yaC2iLsaqJ",
      "capture_base": "EFOnwfWsPxeHq27ToWKFdFjhawS8j_Ol29ULkvPw4uFg",
      "type": "spec/overlays/unit/1.0",
      "attribute_unit": {
        "age": "kg",
        "name": "kg"
      }
    }
  }
}"#
    } else {
        r#"{
  "d": "ENPETKkN4WDazjnQykX-Bx3RmQ-slqwyoa0cwRYi_PfB",
  "capture_base": {
    "d": "EFOnwfWsPxeHq27ToWKFdFjhawS8j_Ol29ULkvPw4uFg",
    "type": "spec/capture_base/1.0",
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
      "d": "EESpBo5wXPBEkFKtsmLL_Vj_1h9nufxvfF8O4eQWbIdR",
      "capture_base": "EFOnwfWsPxeHq27ToWKFdFjhawS8j_Ol29ULkvPw4uFg",
      "type": "spec/overlays/cardinality/1.0",
      "attribute_cardinality": {
        "age": "2",
        "name": "1"
      }
    },
    "character_encoding": {
      "d": "EHIpSMAXHFoP2UiGbjDaMhlLgU9ggZPAsDsh-M9W1MgT",
      "capture_base": "EFOnwfWsPxeHq27ToWKFdFjhawS8j_Ol29ULkvPw4uFg",
      "type": "spec/overlays/character_encoding/1.0",
      "attribute_character_encoding": {
        "age": "utf-8",
        "name": "utf-8"
      }
    },
    "conditional": {
      "d": "EKds5IULUlcCnNq5o8DSBll3xSdSoI0AsWDpR-2V9-is",
      "capture_base": "EFOnwfWsPxeHq27ToWKFdFjhawS8j_Ol29ULkvPw4uFg",
      "type": "spec/overlays/conditional/1.0",
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
      "d": "EMQIjOAGWkg5CVBhOpm6L_i82dndhSJ6anE1KnHtEYBy",
      "capture_base": "EFOnwfWsPxeHq27ToWKFdFjhawS8j_Ol29ULkvPw4uFg",
      "type": "spec/overlays/conformance/1.0",
      "attribute_conformance": {
        "age": "M",
        "name": "O"
      }
    },
    "entry": [
      {
        "d": "EB4lwWLx0giSMadcal043Uv2f7kAQNqAqp-Cnka03El4",
        "capture_base": "EFOnwfWsPxeHq27ToWKFdFjhawS8j_Ol29ULkvPw4uFg",
        "type": "spec/overlays/entry/1.0",
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
        "d": "EOVbtQZXFZv2lV7eG_1bUvKabGAolJQK_cy7PHB3nJqA",
        "capture_base": "EFOnwfWsPxeHq27ToWKFdFjhawS8j_Ol29ULkvPw4uFg",
        "type": "spec/overlays/entry/1.0",
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
      "d": "EAax_SvkKilHfJ83UoZxCX6219om1_8_nMXrQCp8PSCg",
      "capture_base": "EFOnwfWsPxeHq27ToWKFdFjhawS8j_Ol29ULkvPw4uFg",
      "type": "spec/overlays/entry_code/1.0",
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
        "d": "EEVT-1J1KbjV-B1IUSmSGuNd4buqCwT7MoslwCsGn4z2",
        "capture_base": "EFOnwfWsPxeHq27ToWKFdFjhawS8j_Ol29ULkvPw4uFg",
        "type": "spec/overlays/information/1.0",
        "language": "eng",
        "attribute_information": {
          "age": "age information",
          "name": "name information"
        }
      }
    ],
    "label": [
      {
        "d": "EC-Mwbd8PevoVTarSK1zcqDnlkfK1po-CyyM19da3SYw",
        "capture_base": "EFOnwfWsPxeHq27ToWKFdFjhawS8j_Ol29ULkvPw4uFg",
        "type": "spec/overlays/label/1.0",
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
        "d": "EHyoO8skj6nMW_gSbPcdlhUzbRS1SeefJCj4FDjXQXSZ",
        "capture_base": "EFOnwfWsPxeHq27ToWKFdFjhawS8j_Ol29ULkvPw4uFg",
        "type": "spec/overlays/meta/1.0",
        "language": "eng",
        "description": "Test case OCA",
        "name": "Test"
      }
    ],
    "unit": {
      "d": "EPK3PAQ_ZP1Rd6xUyZlXMFBBXx4MzU1c_3yaC2iLsaqJ",
      "capture_base": "EFOnwfWsPxeHq27ToWKFdFjhawS8j_Ol29ULkvPw4uFg",
      "type": "spec/overlays/unit/1.0",
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
