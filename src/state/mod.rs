use said::derivation::SelfAddressing;
use serde::{Deserialize, Serialize, Serializer};
use std::collections::{BTreeMap, HashMap};

#[derive(Serialize)]
pub struct Bundle {
    capture_base: CaptureBase,
    overlays: Vec<Box<dyn Overlay>>,
}

erased_serde::serialize_trait_object!(Overlay);

impl Bundle {
    fn new(encoding: Encoding, bundle_tr: HashMap<Language, BundleTranslation>) -> Bundle {
        let mut bundle = Bundle {
            capture_base: CaptureBase {
                schema_type: String::from("spec/capture_base/1.0"),
                classification: String::from("classification"),
                attributes: HashMap::new(),
                pii: Vec::new(),
            },
            overlays: vec![CharacterEncodingOverlay::new(&encoding)],
        };
        for (lang, translation) in bundle_tr.iter() {
            bundle.overlays.push(MetaOverlay::new(lang, translation));
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
                self.overlays.push(FormatOverlay::new());
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
                self.overlays.push(UnitOverlay::new());
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
                self.overlays.push(EntryCodeOverlay::new());
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
                self.overlays.push(LabelOverlay::new(lang));
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
                    self.overlays.push(InformationOverlay::new(lang));
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
                    self.overlays.push(EntryOverlay::new(lang));
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

#[derive(Serialize, Deserialize, Debug)]
pub struct CaptureBase {
    #[serde(rename = "type")]
    schema_type: String,
    classification: String,
    #[serde(serialize_with = "ordered_attributes")]
    attributes: HashMap<String, AttributeType>,
    pii: Vec<String>,
}

impl CaptureBase {
    fn add(&mut self, attribute: &Attribute) {
        self.attributes
            .insert(attribute.name.clone(), attribute.attr_type);
        if attribute.is_pii {
            self.pii.push(attribute.name.clone());
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum AttributeType {
    Text,
    Number,
    Date,
    #[serde(rename = "Array[Text]")]
    ArrayText,
}

trait Overlay: erased_serde::Serialize {
    fn capture_base(&mut self) -> &mut String;
    fn overlay_type(&self) -> &String;
    fn language(&self) -> Option<&Language> {
        None
    }

    fn add(&mut self, attribute: &Attribute);

    fn sign(&mut self, capture_base_sai: &str) {
        self.capture_base().clear();
        self.capture_base().push_str(capture_base_sai);
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct MetaOverlay {
    capture_base: String,
    #[serde(rename = "type")]
    overlay_type: String,
    language: Language,
    name: String,
    descritpion: String,
}

impl Overlay for MetaOverlay {
    fn capture_base(&mut self) -> &mut String {
        &mut self.capture_base
    }
    fn overlay_type(&self) -> &String {
        &self.overlay_type
    }
    fn language(&self) -> Option<&Language> {
        Some(&self.language)
    }

    fn add(&mut self, _attribute: &Attribute) {}
}
impl MetaOverlay {
    fn new(lang: &Language, bundle_tr: &BundleTranslation) -> Box<MetaOverlay> {
        Box::new(MetaOverlay {
            capture_base: String::new(),
            overlay_type: "spec/overalys/meta/1.0".to_string(),
            language: *lang,
            name: bundle_tr.name.clone(),
            descritpion: bundle_tr.descritpion.clone(),
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct CharacterEncodingOverlay {
    capture_base: String,
    #[serde(rename = "type")]
    overlay_type: String,
    default_character_encoding: Encoding,
    attr_character_encoding: HashMap<String, Encoding>,
}

impl Overlay for CharacterEncodingOverlay {
    fn capture_base(&mut self) -> &mut String {
        &mut self.capture_base
    }
    fn overlay_type(&self) -> &String {
        &self.overlay_type
    }

    fn add(&mut self, attribute: &Attribute) {
        self.attr_character_encoding
            .insert(attribute.name.clone(), attribute.encoding.unwrap());
    }
}
impl CharacterEncodingOverlay {
    fn new(encoding: &Encoding) -> Box<CharacterEncodingOverlay> {
        Box::new(CharacterEncodingOverlay {
            capture_base: String::new(),
            overlay_type: "spec/overalys/character_encoding/1.0".to_string(),
            default_character_encoding: *encoding,
            attr_character_encoding: HashMap::new(),
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct FormatOverlay {
    capture_base: String,
    #[serde(rename = "type")]
    overlay_type: String,
    attr_formats: HashMap<String, String>,
}

impl Overlay for FormatOverlay {
    fn capture_base(&mut self) -> &mut String {
        &mut self.capture_base
    }
    fn overlay_type(&self) -> &String {
        &self.overlay_type
    }

    fn add(&mut self, attribute: &Attribute) {
        if attribute.format.is_some() {
            self.attr_formats.insert(
                attribute.name.clone(),
                attribute.format.as_ref().unwrap().clone(),
            );
        }
    }
}
impl FormatOverlay {
    fn new() -> Box<FormatOverlay> {
        Box::new(FormatOverlay {
            capture_base: String::new(),
            overlay_type: "spec/overalys/format/1.0".to_string(),
            attr_formats: HashMap::new(),
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct UnitOverlay {
    capture_base: String,
    #[serde(rename = "type")]
    overlay_type: String,
    attr_units: HashMap<String, String>,
}

impl Overlay for UnitOverlay {
    fn capture_base(&mut self) -> &mut String {
        &mut self.capture_base
    }
    fn overlay_type(&self) -> &String {
        &self.overlay_type
    }

    fn add(&mut self, attribute: &Attribute) {
        if attribute.unit.is_some() {
            self.attr_units.insert(
                attribute.name.clone(),
                attribute.unit.as_ref().unwrap().clone(),
            );
        }
    }
}
impl UnitOverlay {
    fn new() -> Box<UnitOverlay> {
        Box::new(UnitOverlay {
            capture_base: String::new(),
            overlay_type: "spec/overalys/unit/1.0".to_string(),
            attr_units: HashMap::new(),
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct EntryCodeOverlay {
    capture_base: String,
    #[serde(rename = "type")]
    overlay_type: String,
    attr_entry_codes: HashMap<String, Vec<String>>,
}

impl Overlay for EntryCodeOverlay {
    fn capture_base(&mut self) -> &mut String {
        &mut self.capture_base
    }
    fn overlay_type(&self) -> &String {
        &self.overlay_type
    }

    fn add(&mut self, attribute: &Attribute) {
        if attribute.entry_codes.is_some() {
            self.attr_entry_codes.insert(
                attribute.name.clone(),
                attribute.entry_codes.as_ref().unwrap().clone(),
            );
        }
    }
}
impl EntryCodeOverlay {
    fn new() -> Box<EntryCodeOverlay> {
        Box::new(EntryCodeOverlay {
            capture_base: String::new(),
            overlay_type: "spec/overalys/entry_code/1.0".to_string(),
            attr_entry_codes: HashMap::new(),
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct LabelOverlay {
    capture_base: String,
    #[serde(rename = "type")]
    overlay_type: String,
    language: Language,
    attr_labels: HashMap<String, String>,
    attr_categories: Vec<String>,
    cat_labels: HashMap<String, String>,
    cat_attributes: HashMap<String, Vec<String>>,
}

impl Overlay for LabelOverlay {
    fn capture_base(&mut self) -> &mut String {
        &mut self.capture_base
    }
    fn overlay_type(&self) -> &String {
        &self.overlay_type
    }
    fn language(&self) -> Option<&Language> {
        Some(&self.language)
    }

    fn add(&mut self, attribute: &Attribute) {
        if let Some(tr) = attribute.translations.get(&self.language) {
            self.attr_labels
                .insert(attribute.name.clone(), tr.label.clone());
            self.cat_attributes
                .get_mut("_cat-1_")
                .unwrap()
                .push(attribute.name.clone());
        }
    }
}
impl LabelOverlay {
    fn new(lang: &Language) -> Box<LabelOverlay> {
        let mut cat_labels = HashMap::new();
        cat_labels.insert(String::from("_cat-1_"), String::from("Category 1"));
        let mut cat_attributes = HashMap::new();
        cat_attributes.insert(String::from("_cat-1_"), vec![]);
        Box::new(LabelOverlay {
            capture_base: String::new(),
            overlay_type: "spec/overalys/label/1.0".to_string(),
            language: *lang,
            attr_labels: HashMap::new(),
            attr_categories: vec![String::from("_cat-1_")],
            cat_labels,
            cat_attributes,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct InformationOverlay {
    capture_base: String,
    #[serde(rename = "type")]
    overlay_type: String,
    language: Language,
    attr_information: HashMap<String, String>,
}

impl Overlay for InformationOverlay {
    fn capture_base(&mut self) -> &mut String {
        &mut self.capture_base
    }
    fn overlay_type(&self) -> &String {
        &self.overlay_type
    }
    fn language(&self) -> Option<&Language> {
        Some(&self.language)
    }

    fn add(&mut self, attribute: &Attribute) {
        if let Some(tr) = attribute.translations.get(&self.language) {
            if let Some(info) = &tr.information {
                self.attr_information
                    .insert(attribute.name.clone(), info.clone());
            }
        }
    }
}
impl InformationOverlay {
    fn new(lang: &Language) -> Box<InformationOverlay> {
        Box::new(InformationOverlay {
            capture_base: String::new(),
            overlay_type: "spec/overalys/information/1.0".to_string(),
            language: *lang,
            attr_information: HashMap::new(),
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct EntryOverlay {
    capture_base: String,
    #[serde(rename = "type")]
    overlay_type: String,
    language: Language,
    attr_entries: HashMap<String, HashMap<String, String>>,
}

impl Overlay for EntryOverlay {
    fn capture_base(&mut self) -> &mut String {
        &mut self.capture_base
    }
    fn overlay_type(&self) -> &String {
        &self.overlay_type
    }
    fn language(&self) -> Option<&Language> {
        Some(&self.language)
    }

    fn add(&mut self, attribute: &Attribute) {
        if let Some(tr) = attribute.translations.get(&self.language) {
            if let Some(entries) = &tr.entries {
                self.attr_entries
                    .insert(attribute.name.clone(), entries.clone());
            }
        }
    }
}
impl EntryOverlay {
    fn new(lang: &Language) -> Box<EntryOverlay> {
        Box::new(EntryOverlay {
            capture_base: String::new(),
            overlay_type: "spec/overalys/entry/1.0".to_string(),
            language: *lang,
            attr_entries: HashMap::new(),
        })
    }
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
