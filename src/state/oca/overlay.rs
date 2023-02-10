pub mod attribute_mapping;
pub mod cardinality;
pub mod character_encoding;
pub mod conditional;
pub mod conformance;
pub mod credential_layout;
pub mod entry;
pub mod entry_code;
pub mod entry_code_mapping;
pub mod form_layout;
pub mod format;
pub mod information;
pub mod label;
pub mod meta;
pub mod standard;
pub mod subset;
pub mod unit;

pub use self::attribute_mapping::AttributeMappingOverlay as AttributeMapping;
pub use self::cardinality::CardinalityOverlay as Cardinality;
pub use self::character_encoding::CharacterEncodingOverlay as CharacterEncoding;
pub use self::conditional::ConditionalOverlay as Conditional;
pub use self::conformance::ConformanceOverlay as Conformance;
pub use self::credential_layout::CredentialLayoutOverlay as CredentialLayout;
pub use self::entry::EntryOverlay as Entry;
pub use self::entry_code::EntryCodeOverlay as EntryCode;
pub use self::entry_code_mapping::EntryCodeMappingOverlay as EntryCodeMapping;
pub use self::form_layout::FormLayoutOverlay as FormLayout;
pub use self::format::FormatOverlay as Format;
pub use self::information::InformationOverlay as Information;
pub use self::label::LabelOverlay as Label;
pub use self::meta::MetaOverlay as Meta;
pub use self::standard::StandardOverlay as Standard;
pub use self::subset::SubsetOverlay as Subset;
use self::unit::MeasurementSystem;
pub use self::unit::UnitOverlay as Unit;
use crate::state::{attribute::Attribute};
use said::derivation::SelfAddressing;
use std::any::Any;
use isolang::Language;
erased_serde::serialize_trait_object!(Overlay);

pub trait Overlay: erased_serde::Serialize {
    fn as_any(&self) -> &dyn Any;
    fn capture_base(&self) -> &String;
    fn capture_base_mut(&mut self) -> &mut String;
    fn said(&self) -> &String;
    fn said_mut(&mut self) -> &mut String;
    fn overlay_type(&self) -> &String;
    fn language(&self) -> Option<&Language> {
        None
    }

    fn attributes(&self) -> Vec<&String>;

    fn add(&mut self, attribute: &Attribute);

    fn calculate_said(&self) -> String {
        let mut buf = vec![];
        {
            let json_serializer = &mut serde_json::Serializer::new(&mut buf);
            let mut erased_serializer: Box<dyn erased_serde::Serializer> =
                Box::new(<dyn erased_serde::Serializer>::erase(json_serializer));
            self.erased_serialize(erased_serializer.as_mut()).unwrap();
        }
        let self_json = std::str::from_utf8(buf.as_slice()).unwrap().to_string();

        format!(
            "{}",
            SelfAddressing::Blake3_256.derive(
                self_json
                    .replace(self.said(), "############################################")
                    .as_bytes()
            )
        )
    }

    fn sign(&mut self, capture_base_sai: &str) {
        self.capture_base_mut().clear();
        self.capture_base_mut().push_str(capture_base_sai);
        self.said_mut().clear();
        self.said_mut()
            .push_str("############################################");

        let mut buf = vec![];
        {
            let json_serializer = &mut serde_json::Serializer::new(&mut buf);
            let mut erased_serializer: Box<dyn erased_serde::Serializer> =
                Box::new(<dyn erased_serde::Serializer>::erase(json_serializer));
            self.erased_serialize(erased_serializer.as_mut()).unwrap();
        }
        let self_json = std::str::from_utf8(buf.as_slice()).unwrap().to_string();
        self.said_mut().clear();
        self.said_mut().push_str(&format!(
            "{}",
            SelfAddressing::Blake3_256.derive(self_json.as_bytes())
        ))
    }
}
