pub mod cardinality;
pub mod character_encoding;
pub mod conditional;
pub mod credential_layout;
pub mod entry;
pub mod entry_code;
pub mod form_layout;
pub mod format;
pub mod information;
pub mod label;
pub mod meta;
pub mod unit;

pub use self::cardinality::CardinalityOverlay as Cardinality;
pub use self::character_encoding::CharacterEncodingOverlay as CharacterEncoding;
pub use self::conditional::ConditionalOverlay as Conditional;
pub use self::credential_layout::CredentialLayoutOverlay as CredentialLayout;
pub use self::entry::EntryOverlay as Entry;
pub use self::entry_code::EntryCodeOverlay as EntryCode;
pub use self::form_layout::FormLayoutOverlay as FormLayout;
pub use self::format::FormatOverlay as Format;
pub use self::information::InformationOverlay as Information;
pub use self::label::LabelOverlay as Label;
pub use self::meta::MetaOverlay as Meta;
pub use self::unit::UnitOverlay as Unit;
use crate::state::{attribute::Attribute, language::Language};

erased_serde::serialize_trait_object!(Overlay);

pub trait Overlay: erased_serde::Serialize {
    fn capture_base(&mut self) -> &mut String;
    fn overlay_type(&self) -> &String;
    fn language(&self) -> Option<&Language> {
        None
    }
    fn attributes(&self) -> Vec<&String>;

    fn add(&mut self, attribute: &Attribute);

    fn sign(&mut self, capture_base_sai: &str) {
        self.capture_base().clear();
        self.capture_base().push_str(capture_base_sai);
    }
}
