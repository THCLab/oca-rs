use said::derivation::SelfAddressing;
use serde::{Deserialize, Serialize, Serializer};
use std::collections::{BTreeMap, HashMap};

#[derive(Serialize, Deserialize, Debug)]
pub struct Bundle {
    pub capture_base: CaptureBase,
    pub overlays: Vec<Overlay>,
}

impl Bundle {
    fn new(encoding: Encoding, bundle_tr: HashMap<Language, BundleTranslation>) -> Bundle {
        let mut bundle = Bundle {
            capture_base: CaptureBase {
                schema_type: String::from("spec/capture_base/1.0"),
                classification: String::from("classification"),
                attributes: HashMap::new(),
                pii: Vec::new(),
            },
            overlays: Vec::new(),
        };
        bundle.overlays.push(Overlay::CharacterEncoding {
            capture_base: String::new(),
            overlay_type: String::from("spec/overlays/character_encoding/1.0"),
            default_character_encoding: encoding,
            attr_character_encoding: HashMap::new(),
        });
        for (lang, m) in bundle_tr.iter() {
            bundle.overlays.push(Overlay::Meta {
                capture_base: String::new(),
                overlay_type: String::from("spec/overalys/meta/1.0"),
                language: *lang,
                name: String::from(&m.name),
                descritpion: String::from(&m.descritpion),
            })
        }
        bundle
    }

    fn add_attribute(&mut self, attr: Attribute) {
        let name = attr.name;
        self.capture_base
            .attributes
            .insert(name.clone(), attr.attr_type);
        if attr.is_pii {
            self.capture_base.pii.push(name.clone());
        }

        if let Some(encoding) = attr.encoding {
            if let Some(Overlay::CharacterEncoding {
                attr_character_encoding,
                ..
            }) = self
                .overlays
                .iter_mut()
                .find(|x| matches!(x, Overlay::CharacterEncoding { .. }))
            {
                attr_character_encoding.insert(name.clone(), encoding);
            }
        }

        if let Some(format) = attr.format {
            let mut format_ov = self
                .overlays
                .iter_mut()
                .find(|x| matches!(x, Overlay::Format { .. }));
            if format_ov.is_none() {
                self.overlays.push(Overlay::Format {
                    capture_base: String::new(),
                    overlay_type: String::from("spec/overlays/formating/1.0"),
                    attr_formats: HashMap::new(),
                });
                format_ov = self.overlays.last_mut();
            }

            if let Some(Overlay::Format { attr_formats, .. }) = format_ov {
                attr_formats.insert(name.clone(), format);
            }
        }

        if let Some(unit) = attr.unit {
            let mut unit_ov = self
                .overlays
                .iter_mut()
                .find(|x| matches!(x, Overlay::Unit { .. }));
            if unit_ov.is_none() {
                self.overlays.push(Overlay::Unit {
                    capture_base: String::new(),
                    overlay_type: String::from("spec/overlays/unit/1.0"),
                    attr_units: HashMap::new(),
                });
                unit_ov = self.overlays.last_mut();
            }

            if let Some(Overlay::Unit { attr_units, .. }) = unit_ov {
                attr_units.insert(name.clone(), unit);
            }
        }

        if let Some(entry_codes) = attr.entry_codes {
            let mut entry_code_ov = self
                .overlays
                .iter_mut()
                .find(|x| matches!(x, Overlay::EntryCode { .. }));
            if entry_code_ov.is_none() {
                self.overlays.push(Overlay::EntryCode {
                    capture_base: String::new(),
                    overlay_type: String::from("spec/overlays/entry_code/1.0"),
                    attr_entry_codes: HashMap::new(),
                });
                entry_code_ov = self.overlays.last_mut();
            }
            if let Some(Overlay::EntryCode {
                attr_entry_codes, ..
            }) = entry_code_ov
            {
                attr_entry_codes.insert(name.clone(), entry_codes);
            }
        }

        for (lang, attr_tr) in attr.translations.iter() {
            let label = String::from(&attr_tr.label);
            let mut label_ov = self.overlays.iter_mut().find(|x| match x {
                Overlay::Label {
                    language: o_lang, ..
                } => o_lang == lang,
                _ => false,
            });
            if label_ov.is_none() {
                let mut cat_labels = HashMap::new();
                cat_labels.insert(String::from("_cat-1_"), String::from("Category 1"));
                let mut cat_attributes = HashMap::new();
                cat_attributes.insert(String::from("_cat-1_"), vec![]);
                self.overlays.push(Overlay::Label {
                    capture_base: String::new(),
                    overlay_type: String::from("spec/overlays/label/1.0"),
                    language: *lang,
                    attr_labels: HashMap::new(),
                    attr_categories: vec![String::from("_cat-1_")],
                    cat_labels,
                    cat_attributes,
                });
                label_ov = self.overlays.last_mut();
            }
            if let Some(Overlay::Label {
                attr_labels,
                cat_attributes,
                ..
            }) = label_ov
            {
                attr_labels.insert(name.clone(), String::from(&label));
                cat_attributes
                    .get_mut("_cat-1_")
                    .unwrap()
                    .push(name.clone());
            }

            if let Some(information) = &attr_tr.information {
                let mut information_ov = self.overlays.iter_mut().find(|x| match x {
                    Overlay::Information {
                        language: o_lang, ..
                    } => o_lang == lang,
                    _ => false,
                });
                if information_ov.is_none() {
                    self.overlays.push(Overlay::Information {
                        capture_base: String::new(),
                        overlay_type: String::from("spec/overlays/information/1.0"),
                        language: *lang,
                        attr_information: HashMap::new(),
                    });
                    information_ov = self.overlays.last_mut();
                }
                if let Some(Overlay::Information {
                        attr_information, ..
                    }) = information_ov
                {
                    attr_information.insert(name.clone(), String::from(information));
                }
            }

            if let Some(entries) = &attr_tr.entries {
                let mut entry_ov = self.overlays.iter_mut().find(|x| match x {
                    Overlay::Entry {
                        language: o_lang, ..
                    } => o_lang == lang,
                    _ => false,
                });
                if entry_ov.is_none() {
                    self.overlays.push(Overlay::Entry {
                        capture_base: String::new(),
                        overlay_type: String::from("spec/overlays/entry/1.0"),
                        language: *lang,
                        attr_entries: HashMap::new(),
                    });
                    entry_ov = self.overlays.last_mut();
                }
                if let Some(Overlay::Entry { attr_entries, .. }) = entry_ov {
                    attr_entries.insert(name.clone(), entries.clone());
                }
            }
        }
    }

    fn sign(&mut self) {
        let cs_json = serde_json::to_string(&self.capture_base).unwrap();
        let sai = format!("{}", SelfAddressing::Blake3_256.derive(cs_json.as_bytes()));
        for o in self.overlays.iter_mut() {
            match o {
                Overlay::Meta { capture_base, .. } => {
                    capture_base.clear();
                    capture_base.push_str(&sai);
                }
                Overlay::Label { capture_base, .. } => {
                    capture_base.clear();
                    capture_base.push_str(&sai);
                }
                Overlay::Information { capture_base, .. } => {
                    capture_base.clear();
                    capture_base.push_str(&sai);
                }
                Overlay::Format { capture_base, .. } => {
                    capture_base.clear();
                    capture_base.push_str(&sai);
                }
                Overlay::Unit { capture_base, .. } => {
                    capture_base.clear();
                    capture_base.push_str(&sai);
                }
                Overlay::CharacterEncoding { capture_base, .. } => {
                    capture_base.clear();
                    capture_base.push_str(&sai);
                }
                Overlay::EntryCode { capture_base, .. } => {
                    capture_base.clear();
                    capture_base.push_str(&sai);
                }
                Overlay::Entry { capture_base, .. } => {
                    capture_base.clear();
                    capture_base.push_str(&sai);
                }
            }
        }
    }
}

struct Attribute {
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

struct AttributeTranslation {
    label: String,
    entries: Option<HashMap<String, String>>,
    information: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CaptureBase {
    #[serde(rename = "type")]
    schema_type: String,
    classification: String,
    #[serde(serialize_with = "ordered_attributes")]
    attributes: HashMap<String, AttributeType>,
    pii: Vec<String>,
}

fn ordered_attributes<S>(
    value: &HashMap<String, AttributeType>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let ordered: BTreeMap<_, _> = value.iter().collect();
    ordered.serialize(serializer)
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum AttributeType {
    Text,
    Number,
    Date,
    #[serde(rename = "Array[Text]")]
    ArrayText,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Overlay {
    Meta {
        capture_base: String,
        #[serde(rename = "type")]
        overlay_type: String,
        language: Language,
        name: String,
        descritpion: String,
    },
    CharacterEncoding {
        capture_base: String,
        #[serde(rename = "type")]
        overlay_type: String,
        default_character_encoding: Encoding,
        attr_character_encoding: HashMap<String, Encoding>,
    },
    Label {
        capture_base: String,
        #[serde(rename = "type")]
        overlay_type: String,
        language: Language,
        attr_labels: HashMap<String, String>,
        attr_categories: Vec<String>,
        cat_labels: HashMap<String, String>,
        cat_attributes: HashMap<String, Vec<String>>,
    },
    Information {
        capture_base: String,
        #[serde(rename = "type")]
        overlay_type: String,
        language: Language,
        attr_information: HashMap<String, String>,
    },
    EntryCode {
        capture_base: String,
        #[serde(rename = "type")]
        overlay_type: String,
        attr_entry_codes: HashMap<String, Vec<String>>,
    },
    Entry {
        capture_base: String,
        #[serde(rename = "type")]
        overlay_type: String,
        language: Language,
        attr_entries: HashMap<String, HashMap<String, String>>,
    },
    Format {
        capture_base: String,
        #[serde(rename = "type")]
        overlay_type: String,
        attr_formats: HashMap<String, String>,
    },
    Unit {
        capture_base: String,
        #[serde(rename = "type")]
        overlay_type: String,
        attr_units: HashMap<String, String>,
    },
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

struct BundleTranslation {
    name: String,
    descritpion: String,
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
