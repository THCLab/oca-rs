use core::str::FromStr;

use said::derivation::SelfAddressing;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

mod capture_base;
mod overlay;
use crate::state::capture_base::CaptureBase;
use crate::state::overlay::Overlay;

#[derive(Serialize)]
pub struct Bundle {
    pub capture_base: CaptureBase,
    pub overlays: Vec<Box<dyn Overlay>>,
}

impl Bundle {
    pub fn new(
        default_encoding: Encoding,
        bundle_tr: HashMap<Language, BundleTranslation>,
    ) -> Bundle {
        let mut bundle = Bundle {
            capture_base: CaptureBase::new(),
            overlays: vec![overlay::CharacterEncoding::new(&default_encoding)],
        };
        for (lang, translation) in bundle_tr.iter() {
            bundle.overlays.push(overlay::Meta::new(lang, translation));
        }
        bundle
    }

    pub fn add_attribute(&mut self, attr: Attribute) {
        self.capture_base.add(&attr);

        if attr.encoding.is_some() {
            let encoding_ov = self
                .overlays
                .iter_mut()
                .find(|x| x.overlay_type().contains("/character_encoding/"));
            if let Some(ov) = encoding_ov {
                ov.add(&attr);
            }
        }

        if attr.format.is_some() {
            let mut format_ov = self
                .overlays
                .iter_mut()
                .find(|x| x.overlay_type().contains("/format/"));
            if format_ov.is_none() {
                self.overlays.push(overlay::Format::new());
                format_ov = self.overlays.last_mut();
            }

            if let Some(ov) = format_ov {
                ov.add(&attr)
            }
        }

        if attr.unit.is_some() {
            let mut unit_ov = self
                .overlays
                .iter_mut()
                .find(|x| x.overlay_type().contains("/unit/"));
            if unit_ov.is_none() {
                self.overlays.push(overlay::Unit::new());
                unit_ov = self.overlays.last_mut();
            }

            if let Some(ov) = unit_ov {
                ov.add(&attr)
            }
        }

        if attr.entry_codes.is_some() {
            let mut entry_code_ov = self
                .overlays
                .iter_mut()
                .find(|x| x.overlay_type().contains("/entry_code/"));
            if entry_code_ov.is_none() {
                self.overlays.push(overlay::EntryCode::new());
                entry_code_ov = self.overlays.last_mut();
            }

            if let Some(ov) = entry_code_ov {
                ov.add(&attr)
            }
        }

        for (lang, attr_tr) in attr.translations.iter() {
            let mut label_ov = self.overlays.iter_mut().find(|x| {
                if let Some(o_lang) = x.language() {
                    return o_lang == lang && x.overlay_type().contains("/label/");
                }
                false
            });
            if label_ov.is_none() {
                self.overlays.push(overlay::Label::new(lang));
                label_ov = self.overlays.last_mut();
            }
            if let Some(ov) = label_ov {
                ov.add(&attr);
            }

            if attr_tr.information.is_some() {
                let mut information_ov = self.overlays.iter_mut().find(|x| {
                    if let Some(o_lang) = x.language() {
                        return o_lang == lang && x.overlay_type().contains("/character_encoding/");
                    }
                    false
                });
                if information_ov.is_none() {
                    self.overlays.push(overlay::Information::new(lang));
                    information_ov = self.overlays.last_mut();
                }
                if let Some(ov) = information_ov {
                    ov.add(&attr);
                }
            }

            if attr_tr.entries.is_some() {
                let mut entry_ov = self.overlays.iter_mut().find(|x| {
                    if let Some(o_lang) = x.language() {
                        return o_lang == lang && x.overlay_type().contains("/entry/");
                    }
                    false
                });
                if entry_ov.is_none() {
                    self.overlays.push(overlay::Entry::new(lang));
                    entry_ov = self.overlays.last_mut();
                }
                if let Some(ov) = entry_ov {
                    ov.add(&attr);
                }
            }
        }
    }

    pub fn sign(&mut self) {
        let cs_json = serde_json::to_string(&self.capture_base).unwrap();
        let sai = format!("{}", SelfAddressing::Blake3_256.derive(cs_json.as_bytes()));
        for o in self.overlays.iter_mut() {
            o.sign(&sai);
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Attribute {
    name: String,
    attr_type: AttributeType,
    is_pii: bool,
    translations: HashMap<Language, AttributeTranslation>,
    encoding: Option<Encoding>,
    format: Option<String>,
    unit: Option<String>,
    entry_codes: Option<Vec<String>>,
}

impl Attribute {
    pub fn new(
        name: String,
        attr_type: AttributeType,
        is_pii: bool,
        translations: HashMap<Language, AttributeTranslation>,
        encoding: Option<Encoding>,
        format: Option<String>,
        unit: Option<String>,
        entry_codes: Option<Vec<String>>,
    ) -> Attribute {
        Attribute {
            name,
            attr_type,
            is_pii,
            translations,
            encoding,
            format,
            unit,
            entry_codes,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BundleTranslation {
    name: String,
    descritpion: String,
}

impl BundleTranslation {
    pub fn new(name: String, descritpion: String) -> BundleTranslation {
        BundleTranslation { name, descritpion }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AttributeTranslation {
    label: String,
    entries: Option<HashMap<String, String>>,
    information: Option<String>,
}

impl AttributeTranslation {
    pub fn new(
        label: String,
        entries: Option<HashMap<String, String>>,
        information: Option<String>,
    ) -> AttributeTranslation {
        AttributeTranslation {
            label,
            entries,
            information,
        }
    }
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum AttributeType {
    Text,
    Number,
    Date,
    #[serde(rename = "Array[Text]")]
    ArrayText,
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Language {
    #[serde(rename = "en_EN")]
    En,
    #[serde(rename = "en_US")]
    EnUs,
    #[serde(rename = "pl_PL")]
    Pl,
}

impl FromStr for Language {
    type Err = ();

    fn from_str(input: &str) -> Result<Language, Self::Err> {
        match input {
            "0" => Ok(Language::En),
            "En" => Ok(Language::En),
            "1" => Ok(Language::EnUs),
            "EnUs" => Ok(Language::EnUs),
            "2" => Ok(Language::Pl),
            "Pl" => Ok(Language::Pl),
            _ => Err(()),
        }
    }
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Encoding {
    #[serde(rename = "utf-8")]
    Utf8,
    #[serde(rename = "iso-8859-1")]
    Iso8859_1,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut bundle_tr: HashMap<Language, BundleTranslation> = HashMap::new();
        bundle_tr.insert(
            Language::EnUs,
            BundleTranslation::new(
                String::from("Driving Licence"),
                String::from("DL oca schema"),
            ),
        );
        bundle_tr.insert(
            Language::Pl,
            BundleTranslation::new(String::from("Prawo Jazdy"), String::from("PJ oca")),
        );
        let mut bundle = Bundle::new(Encoding::Utf8, bundle_tr);

        let mut attr1_tr = HashMap::new();
        let mut attr1_en_entries = HashMap::new();
        attr1_en_entries.insert("op1".to_string(), "Option 1".to_string());
        attr1_en_entries.insert("op2".to_string(), "Option 2".to_string());
        attr1_tr.insert(
            Language::EnUs,
            AttributeTranslation::new(
                String::from("Name:"),
                Some(attr1_en_entries),
                Some("info en".to_string()),
            ),
        );
        let mut attr1_pl_entries = HashMap::new();
        attr1_pl_entries.insert("op1".to_string(), "Opcja 1".to_string());
        attr1_pl_entries.insert("op2".to_string(), "Opcja 2".to_string());
        attr1_tr.insert(
            Language::Pl,
            AttributeTranslation::new(
                String::from("Imię:"),
                Some(attr1_pl_entries),
                Some("info pl".to_string()),
            ),
        );
        let attr1 = Attribute::new(
            String::from("n1"),
            AttributeType::Text,
            true,
            attr1_tr,
            None,
            None,
            Some(String::from("days")),
            Some(vec!["op1".to_string(), "op2".to_string()]),
        );
        bundle.add_attribute(attr1);

        let mut attr2_tr = HashMap::new();
        attr2_tr.insert(
            Language::EnUs,
            AttributeTranslation::new(String::from("Date:"), None, None),
        );
        attr2_tr.insert(
            Language::Pl,
            AttributeTranslation::new(String::from("Data:"), None, None),
        );
        let attr2 = Attribute::new(
            String::from("n2"),
            AttributeType::Date,
            false,
            attr2_tr,
            Some(Encoding::Iso8859_1),
            Some(String::from("DD/MM/YYYY")),
            None,
            None,
        );
        bundle.add_attribute(attr2);
        bundle.sign();

        println!("{:#?}", serde_json::to_string(&bundle).unwrap());

        assert_eq!(bundle.capture_base.attributes.len(), 2);
        assert_eq!(bundle.capture_base.pii.len(), 1);
    }
}
