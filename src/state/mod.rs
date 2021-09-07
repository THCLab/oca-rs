use said::derivation::SelfAddressing;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mod capture_base;
mod overlay;
use crate::state::capture_base::CaptureBase;
use crate::state::overlay::Overlay;

#[derive(Serialize)]
pub struct Bundle {
    capture_base: CaptureBase,
    overlays: Vec<Box<dyn Overlay>>,
}

impl Bundle {
    fn new(encoding: Encoding, bundle_tr: HashMap<Language, BundleTranslation>) -> Bundle {
        let mut bundle = Bundle {
            capture_base: CaptureBase::new(),
            overlays: vec![overlay::CharacterEncoding::new(&encoding)],
        };
        for (lang, translation) in bundle_tr.iter() {
            bundle.overlays.push(overlay::Meta::new(lang, translation));
        }
        bundle
    }

    fn add_attribute(&mut self, attr: Attribute) {
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

    fn sign(&mut self) {
        let cs_json = serde_json::to_string(&self.capture_base).unwrap();
        let sai = format!("{}", SelfAddressing::Blake3_256.derive(cs_json.as_bytes()));
        for o in self.overlays.iter_mut() {
            o.sign(&sai);
        }
    }
}

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
    fn new(
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

pub struct BundleTranslation {
    name: String,
    descritpion: String,
}

struct AttributeTranslation {
    label: String,
    entries: Option<HashMap<String, String>>,
    information: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum AttributeType {
    Text,
    Number,
    Date,
    #[serde(rename = "Array[Text]")]
    ArrayText,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Language {
    #[serde(rename = "en_US")]
    EnUs,
    #[serde(rename = "pl_PL")]
    PlPl,
}

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
            BundleTranslation {
                name: String::from("Driving Licence"),
                descritpion: String::from("DL oca schema"),
            },
        );
        bundle_tr.insert(
            Language::PlPl,
            BundleTranslation {
                name: String::from("Prawo Jazdy"),
                descritpion: String::from("PJ oca"),
            },
        );
        let mut bundle = Bundle::new(Encoding::Utf8, bundle_tr);

        let mut attr1_tr = HashMap::new();
        let mut attr1_en_entries = HashMap::new();
        attr1_en_entries.insert("op1".to_string(), "Option 1".to_string());
        attr1_en_entries.insert("op2".to_string(), "Option 2".to_string());
        attr1_tr.insert(
            Language::EnUs,
            AttributeTranslation {
                label: String::from("Name:"),
                entries: Some(attr1_en_entries),
                information: Some("info en".to_string()),
            },
        );
        let mut attr1_pl_entries = HashMap::new();
        attr1_pl_entries.insert("op1".to_string(), "Opcja 1".to_string());
        attr1_pl_entries.insert("op2".to_string(), "Opcja 2".to_string());
        attr1_tr.insert(
            Language::PlPl,
            AttributeTranslation {
                label: String::from("Imię:"),
                entries: Some(attr1_pl_entries),
                information: Some("info pl".to_string()),
            },
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
            AttributeTranslation {
                label: String::from("Date:"),
                entries: None,
                information: None,
            },
        );
        attr2_tr.insert(
            Language::PlPl,
            AttributeTranslation {
                label: String::from("Data:"),
                entries: None,
                information: None,
            },
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
    }
}
