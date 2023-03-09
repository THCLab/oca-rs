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
use crate::state::{attribute::Attribute};
use said::derivation::SelfAddressing;
use std::any::Any;
use isolang::Language;
erased_serde::serialize_trait_object!(Overlay);

use dyn_clonable::*;

#[clonable]
pub trait Overlay: erased_serde::Serialize + Clone {
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

            impl serde::Serialize for [<$name Overlay>] {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
                {
                    use std::collections::BTreeMap;
                    let mut state = serializer.serialize_struct(stringify!([<$name Overlay>]), 4)?;
                    state.serialize_field("said", &self.said)?;
                    state.serialize_field("type", &self.overlay_type)?;
                    state.serialize_field("capture_base", &self.capture_base)?;
                    let sorted_attr: BTreeMap<_, _> = self.$field1.iter().collect();
                    state.serialize_field(stringify!($field1), &sorted_attr)?;
                    state.end()
                }
            }

            #[derive(serde::Deserialize, Debug, Clone)]
            pub struct [<$name Overlay>] {
                capture_base: String,
                said: String,
                #[serde(rename = "type")]
                overlay_type: String,
                pub $field1: std::collections::HashMap<String, $field2_type>
            }

            impl crate::state::oca::overlay::Overlay for [<$name Overlay>] {
                fn as_any(&self) -> &dyn std::any::Any {
                    self
                }
                fn overlay_type(&self) -> &String {
                    &self.overlay_type
                }
                fn capture_base(&self) -> &String {
                    &self.capture_base
                }
                fn capture_base_mut(&mut self) -> &mut String {
                    &mut self.capture_base
                }
                fn said(&self) -> &String {
                    &self.said
                }
                fn said_mut(&mut self) -> &mut String {
                    &mut self.said
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
                        capture_base: String::new(),
                        said: String::from("############################################"),
                        overlay_type: format!("spec/overlays/{}/1.0", stringify!([<$name:snake:lower>])),
                        $field1: std::collections::HashMap::new(),

                    }
                }
            }
        }
    }
}
pub(crate) use overlay;
