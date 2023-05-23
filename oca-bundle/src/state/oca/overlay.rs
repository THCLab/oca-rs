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
#[cfg(feature = "format_overlay")]
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
#[cfg(feature = "format_overlay")]
pub use self::format::FormatOverlay as Format;
pub use self::information::InformationOverlay as Information;
pub use self::label::LabelOverlay as Label;
pub use self::meta::MetaOverlay as Meta;
pub use self::standard::StandardOverlay as Standard;
pub use self::subset::SubsetOverlay as Subset;
use self::unit::MeasurementSystem;
pub use self::unit::UnitOverlay as Unit;
use crate::state::attribute::Attribute;
use said::{sad::SAD, sad::SerializationFormats, derivation::HashFunctionCode};
use std::any::Any;
use isolang::Language;
erased_serde::serialize_trait_object!(Overlay);

use dyn_clonable::*;

#[clonable]
pub trait Overlay: erased_serde::Serialize + Clone + SAD {
    fn as_any(&self) -> &dyn Any;
    fn capture_base(&self) -> &Option<said::SelfAddressingIdentifier>;
    fn set_capture_base(&mut self, said: &said::SelfAddressingIdentifier);
    fn said(&self) -> &Option<said::SelfAddressingIdentifier>;
    fn overlay_type(&self) -> &String;
    fn language(&self) -> Option<&Language> {
        None
    }

    fn attributes(&self) -> Vec<&String>;

    fn add(&mut self, attribute: &Attribute);

    fn fill_said(&mut self) {
        self.compute_digest();//HashFunctionCode::Blake3_256, SerializationFormats::JSON);
    }

    fn sign(&mut self, capture_base_sai: &said::SelfAddressingIdentifier) {
        self.set_capture_base(capture_base_sai);
        self.fill_said();
    }
}

macro_rules! overlay {
    ($name:ident, $field1:ident, $field2:ident: $field2_type:ty) => {
        paste::paste! {
            pub trait [<$name s>] {
                fn [<set_ $field2>](&mut self, $field2: $field2_type);
            }

            impl [<$name s>] for crate::state::attribute::Attribute {
                fn [<set_ $field2>](&mut self, $field2: $field2_type) {
                    self.$field2 = Some($field2);
                }
            }

            pub fn serialize_attributes<S>(attributes: &std::collections::HashMap<String, $field2_type>, s: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                use std::collections::BTreeMap;

                let mut ser = s.serialize_map(Some(attributes.len()))?;
                let sorted_attributes: BTreeMap<_, _> = attributes.iter().collect();
                for (k, v) in sorted_attributes {
                    ser.serialize_entry(k, v)?;
                }
                ser.end()
            }

            #[derive(serde::Deserialize, serde::Serialize, SAD, Debug, Clone)]
            pub struct [<$name Overlay>] {
                #[said]
                #[serde(rename = "d")]
                said: Option<said::SelfAddressingIdentifier>,
                #[serde(rename = "type")]
                overlay_type: String,
                capture_base: Option<said::SelfAddressingIdentifier>,
                #[serde(serialize_with = "serialize_attributes")]
                pub $field1: std::collections::HashMap<String, $field2_type>
            }

            impl crate::state::oca::overlay::Overlay for [<$name Overlay>] {
                fn as_any(&self) -> &dyn std::any::Any {
                    self
                }
                fn overlay_type(&self) -> &String {
                    &self.overlay_type
                }
                fn capture_base(&self) -> &Option<said::SelfAddressingIdentifier> {
                    &self.capture_base
                }
                fn set_capture_base(&mut self, said: &said::SelfAddressingIdentifier) {
                    self.capture_base = Some(said.clone());
                }
                fn said(&self) -> &Option<said::SelfAddressingIdentifier> {
                    &self.said
                }
                fn attributes(&self) -> Vec<&String> {
                    self.$field1.keys().collect::<Vec<&String>>()
                }

                fn add(&mut self, attribute: &crate::state::attribute::Attribute) {
                    if attribute.$field2.is_some() {
                        self.$field1.insert(attribute.name.clone(), attribute.$field2.clone().unwrap());
                    }
                }
            }

            impl Default for [<$name Overlay>] {
                fn default() -> Self {
                    Self::new()
                }
            }

            impl [<$name Overlay>] {
                pub fn new() -> Self {
                    Self {
                        capture_base: None,
                        said: None,
                        overlay_type: format!("spec/overlays/{}/1.0", stringify!([<$name:snake:lower>])),
                        $field1: std::collections::HashMap::new(),

                    }
                }
            }
        }
    }
}
pub(crate) use overlay;
