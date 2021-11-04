use oca_rust::state::{
    attribute::{Attribute as AttributeRaw, AttributeBuilder as AttributeBuilderRaw, AttributeType, Entry as EntryRaw},
    encoding::Encoding,
    language::Language,
    oca::{OCA as OCARaw, OCABuilder as OCABuilderRaw},
    validator,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "OCA")]
    pub type OCA;
    #[wasm_bindgen(typescript_type = "Attribute")]
    pub type Attribute;
    #[wasm_bindgen(typescript_type = "ITranslations")]
    pub type ITranslations;
    #[wasm_bindgen(typescript_type = "IEntry")]
    pub type IEntry;
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Entry {
    code: String,
    translations: HashMap<String, String>,
}

#[wasm_bindgen]
impl Entry {
    #[wasm_bindgen(constructor)]
    pub fn constructor(code: String, translations: ITranslations) -> Entry {
        let translations_str: HashMap<String, String> =
            serde_wasm_bindgen::from_value(JsValue::from(translations)).unwrap();

        Entry {
            code,
            translations: translations_str
        }
    }

    pub fn plain(self) -> IEntry {
        IEntry::from(
            JsValue::from_serde(&self).unwrap_or(JsValue::NULL)
        )
    }
}

#[wasm_bindgen]
pub struct OCABuilder {
    raw: OCABuilderRaw,
}

#[wasm_bindgen]
impl OCABuilder {
    #[wasm_bindgen(constructor)]
    pub fn new(encoding: Encoding) -> OCABuilder {
        OCABuilder {
            raw: OCABuilderRaw::new(encoding),
        }
    }

    #[wasm_bindgen(js_name = "addClassification")]
    pub fn add_classification(mut self, classification: String) -> OCABuilder {
        self.raw = self.raw.add_classification(classification);
        self
    }

    #[wasm_bindgen(js_name = "addName")]
    pub fn add_name(mut self, names: ITranslations) -> OCABuilder {
        let names_str: HashMap<String, String> =
            serde_wasm_bindgen::from_value(JsValue::from(names)).unwrap();

        let mut names_raw: HashMap<Language, String> = HashMap::new();
        for (lang, name) in names_str.iter() {
            names_raw.insert(lang.to_string(), name.clone());
        }

        self.raw = self.raw.add_name(names_raw);
        self
    }

    #[wasm_bindgen(js_name = "addDescription")]
    pub fn add_description(mut self, descriptions: ITranslations) -> OCABuilder {
        let descriptions_str: HashMap<String, String> =
            serde_wasm_bindgen::from_value(JsValue::from(descriptions)).unwrap();

        let mut descriptions_raw: HashMap<Language, String> = HashMap::new();
        for (lang, description) in descriptions_str.iter() {
            descriptions_raw.insert(lang.to_string(), description.clone());
        }

        self.raw = self.raw.add_description(descriptions_raw);
        self
    }

    #[wasm_bindgen(js_name = "addAttribute")]
    pub fn add_attribute(mut self, attr: Attribute) -> OCABuilder {
        let attr_raw: AttributeRaw = attr.into_serde().unwrap();
        self.raw = self.raw.add_attribute(attr_raw);
        self
    }

    pub fn finalize(self) -> OCA {
        OCA::from(
            JsValue::from_serde(&self.raw.finalize()).unwrap_or(JsValue::NULL)
        )
    }
}

#[wasm_bindgen]
pub struct Validator {
    raw: validator::Validator,
}

impl Default for Validator {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
impl Validator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Validator {
        Validator {
            raw: validator::Validator::new(),
        }
    }

    #[wasm_bindgen(js_name = "enforceTranslations")]
    pub fn enforce_translations(mut self, languages: JsValue) -> Validator {
        let languages_raw: Vec<String> = serde_wasm_bindgen::from_value(languages).unwrap();

        self.raw = self.raw.enforce_translations(languages_raw);
        self
    }

    pub fn validate(self, oca: OCA) -> JsValue {
        #[derive(Serialize)]
        struct ReturnResult {
            success: bool,
            errors: Vec<String>,
        }
        let return_result: ReturnResult;
        let oca_raw: OCARaw = oca.into_serde().unwrap();
        let result = self.raw.validate(&oca_raw);
        match result {
            Ok(()) => {
                return_result = ReturnResult {
                    success: true,
                    errors: vec![],
                }
            }
            Err(err) => {
                let errors: Vec<String> = err
                    .iter()
                    .map(|e| {
                        if let validator::Error::Custom(msg) = e {
                            msg.clone()
                        } else {
                            "undefined error".to_string()
                        }
                    })
                    .collect();
                return_result = ReturnResult {
                    success: false,
                    errors,
                }
            }
        }

        JsValue::from_serde(&return_result).unwrap_or(JsValue::NULL)
    }
}

#[wasm_bindgen]
pub struct AttributeBuilder {
    raw: AttributeBuilderRaw,
}

#[wasm_bindgen]
impl AttributeBuilder {
    #[wasm_bindgen(constructor)]
    pub fn new(name: String, attr_type: AttributeType) -> AttributeBuilder {
        AttributeBuilder {
            raw: AttributeBuilderRaw::new(name, attr_type),
        }
    }

    #[wasm_bindgen(js_name = "setPii")]
    pub fn set_pii(mut self) -> AttributeBuilder {
        self.raw = self.raw.set_pii();
        self
    }

    #[wasm_bindgen(js_name = "addEncoding")]
    pub fn add_encoding(mut self, encoding: Encoding) -> AttributeBuilder {
        self.raw = self.raw.add_encoding(encoding);
        self
    }

    #[wasm_bindgen(js_name = "addFormat")]
    pub fn add_format(mut self, format: String) -> AttributeBuilder {
        self.raw = self.raw.add_format(format);
        self
    }

    #[wasm_bindgen(js_name = "addUnit")]
    pub fn add_unit(mut self, unit: String) -> AttributeBuilder {
        self.raw = self.raw.add_unit(unit);
        self
    }

    #[wasm_bindgen(js_name = "addLabel")]
    pub fn add_label(mut self, labels: ITranslations) -> AttributeBuilder {
        let labels_str: HashMap<String, String> =
            serde_wasm_bindgen::from_value(JsValue::from(labels)).unwrap();

        let mut labels_raw: HashMap<Language, String> = HashMap::new();
        for (lang, label) in labels_str.iter() {
            labels_raw.insert(lang.to_string(), label.clone());
        }

        self.raw = self.raw.add_label(labels_raw);
        self
    }

    #[wasm_bindgen(js_name = "addEntries")]
    pub fn add_entries(mut self, entries: Vec<IEntry>) -> AttributeBuilder {
        let mut entries_raw: Vec<EntryRaw> = vec![];
        for entry in entries.iter() {
            let e: Entry = serde_wasm_bindgen::from_value(JsValue::from(entry)).unwrap();

            let mut entry_tr_raw: HashMap<Language, String> = HashMap::new();
            for (lang, entry_v) in e.translations.iter() {
                entry_tr_raw.insert(lang.to_string(), entry_v.clone());
            }
            entries_raw.push(EntryRaw::new(e.code, entry_tr_raw))
        }

        self.raw = self.raw.add_entries(entries_raw);
        self
    }

    #[wasm_bindgen(js_name = "addInformation")]
    pub fn add_information(mut self, information: ITranslations) -> AttributeBuilder {
        let information_str: HashMap<String, String> =
            serde_wasm_bindgen::from_value(JsValue::from(information)).unwrap();

        let mut information_raw: HashMap<Language, String> = HashMap::new();
        for (lang, info) in information_str.iter() {
            information_raw.insert(lang.to_string(), info.clone());
        }

        self.raw = self.raw.add_information(information_raw);
        self
    }

    pub fn build(self) -> Attribute {
        Attribute::from(
            JsValue::from_serde(&self.raw.build()).unwrap_or(JsValue::NULL)
        )
    }
}

#[wasm_bindgen(typescript_custom_section)]
const OCA_TYPE: &'static str = r#"
type OCA = {
  capture_base: CaptureBase;
  overlays: Overlay[];
}

type CaptureBase = {
  type: string,
  classification: string,
  attributes: { [attr_name: string]: string },
  pii: string[]
}

type Overlay =
  | CharacterEncodingOverlay
  | EntryOverlay
  | EntryCodeOverlay
  | FormatOverlay
  | InformationOverlay
  | LabelOverlay
  | MetaOverlay
  | UnitOverlay

type CharacterEncodingOverlay = {
  capture_base: string,
  type: string,
  default_character_encoding: string,
  attr_character_encoding: { [attr_name: string]: string }
}

type EntryOverlay = {
  capture_base: string,
  type: string,
  language: string,
  attr_entries: { [attr_name: string]: { [entry_code: string]: string } }
}

type EntryCodeOverlay = {
  capture_base: string,
  type: string,
  attr_entry_codes: { [attr_name: string]: string[] }
}

type FormatOverlay = {
  capture_base: string,
  type: string,
  attr_formats: { [attr_name: string]: string }
}

type InformationOverlay = {
  capture_base: string,
  type: string,
  language: string,
  attr_information: { [attr_name: string]: string }
}

type LabelOverlay = {
  capture_base: string,
  type: string,
  language: string,
  attr_labels: { [attr_name: string]: string }
  attr_categories: string[],
  cat_labels: { [cat_id: string]: string },
  cat_attributes: { [cat_id: string]: string[] }
}

type MetaOverlay = {
  capture_base: string,
  type: string,
  language: string,
  name: string,
  description: string
}

type UnitOverlay = {
  capture_base: string,
  type: string,
  attr_units: { [attr_name: string]: string }
}
"#;

#[wasm_bindgen(typescript_custom_section)]
const ATTRIBUTE_TYPE: &'static str = r#"
type AttributeTranslation = {
  label?: string,
  entries?: { [entry_code: string]: string }
  information?: string
}

type Attribute = {
  name: string
  attr_type: string
  is_pii: boolean
  translations: { [language: string]: AttributeTranslation }
  encoding?: string
  format?: string
  unit?: string
  entry_codes?: string[]
}
"#;

#[wasm_bindgen(typescript_custom_section)]
const TYPES: &'static str = r#"
interface ITranslations {
  [language: string]: string
}

interface IEntry {
  code: string,
  translations: ITranslations
}
"#;
