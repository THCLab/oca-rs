use oca_bundle::state::{
    attribute::{Attribute, AttributeType},
    encoding::Encoding,
    oca::OCABox,
    oca::overlay::character_encoding::CharacterEncodings,
    oca::overlay::conformance::Conformances,
    oca::overlay::cardinality::Cardinalitys,
};

#[cfg(feature = "format_overlay")]
use oca_bundle::state::oca::overlay::format::Formats;

use cascade::cascade;

#[test]
fn create_oca() {
    let mut oca = cascade! {
        OCABox::new();
        ..add_meta_attribute("name".to_string(), "Test".to_string());
        ..add_meta_attribute("description".to_string(), "Test case OCA".to_string());
    };

    let mut attribute = cascade! {
        Attribute::new("name".to_string());
        ..set_attribute_type(AttributeType::Text);
        ..set_encoding(Encoding::Utf8);
        ..set_cardinality("1".to_string());
    };
    #[cfg(feature = "format_overlay")]
    attribute.set_format("^[a-zA-Z]*$".to_string());

    oca.add_attribute(attribute);

    let attribute_2 = cascade! {
        Attribute::new("age".to_string());
        ..set_attribute_type(AttributeType::Numeric);
        ..set_flagged();
        ..set_encoding(Encoding::Utf8);
        ..set_conformance("M".to_string());
        ..set_cardinality("2".to_string());
    };

    oca.add_attribute(attribute_2);

    let oca_bundle = oca.generate_bundle().unwrap();
    println!("{}", serde_json::to_string_pretty(&oca_bundle).unwrap());
    assert_eq!(oca_bundle.capture_base.attributes.len(), 2);
    assert_eq!(oca_bundle.capture_base.flagged_attributes.len(), 1);

    #[cfg(not(feature = "format_overlay"))]
    assert_eq!(oca_bundle.overlays.len(), 4);
    #[cfg(feature = "format_overlay")]
    assert_eq!(oca_bundle.overlays.len(), 5);
}
