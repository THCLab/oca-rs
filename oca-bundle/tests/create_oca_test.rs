use isolang::Language;
use oca_bundle::state::{
    attribute::{Attribute, AttributeType},
    entry_codes::EntryCodes as EntryCodesValue,
    entries::EntriesElement,
    encoding::Encoding,
    oca::OCABox,
    oca::overlay::meta::Metas,
    oca::overlay::character_encoding::CharacterEncodings,
    oca::overlay::conformance::Conformances,
    oca::overlay::cardinality::Cardinalitys,
    oca::overlay::entry_code::EntryCodes,
    oca::overlay::entry::Entries,
    oca::overlay::label::Labels,
    oca::overlay::information::Information,
    oca::overlay::unit::{Unit, AttributeUnit, MeasurementSystem, MeasurementUnit, MetricUnit},
    oca::overlay::form_layout::FormLayouts,
    oca::overlay::credential_layout::CredentialLayouts,
};

#[cfg(feature = "format_overlay")]
use oca_bundle::state::oca::overlay::format::Formats;

use cascade::cascade;
use maplit::hashmap;

#[test]
fn create_oca() {
    let form_layout = r#"
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
    "#;
    let mut oca = cascade! {
        OCABox::new();
        ..add_meta(Language::Eng, "name".to_string(), "Test".to_string());
        ..add_meta(Language::Eng, "description".to_string(), "Test case OCA".to_string());
        ..add_form_layout(form_layout.to_string());
        ..add_credential_layout(credential_layout.to_string());
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
        ..set_cardinality("1".to_string());
        ..set_conformance("M".to_string());
        ..set_cardinality("2".to_string());
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
    // println!("{}", serde_json::to_string_pretty(&oca_bundle).unwrap());
    assert_eq!(oca_bundle.capture_base.attributes.len(), 2);
    assert_eq!(oca_bundle.capture_base.flagged_attributes.len(), 2);

    #[cfg(not(feature = "format_overlay"))]
    assert_eq!(oca_bundle.overlays.len(), 11);
    #[cfg(feature = "format_overlay")]
    assert_eq!(oca_bundle.overlays.len(), 12);

    assert_eq!(oca_bundle.said, oca.generate_bundle().said);
}
