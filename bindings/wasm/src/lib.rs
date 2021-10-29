use oca_rust::state::{
    attribute::{Attribute as AttributeRaw, AttributeType, Entry as EntryRaw},
    encoding::Encoding,
    language::Language,
    oca::OCA as OCARaw,
    validator,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "IOCA")]
    pub type IOCA;
    #[wasm_bindgen(typescript_type = "IAttribute")]
    pub type IAttribute;
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

    pub fn build(self) -> IEntry {
        IEntry::from(
            JsValue::from_serde(&self).unwrap_or(JsValue::NULL)
        )
    }
}


#[wasm_bindgen]
pub struct OCA {
    raw: OCARaw,
}

#[wasm_bindgen]
impl OCA {
    #[wasm_bindgen(constructor)]
    pub fn new(encoding: Encoding) -> OCA {
        OCA {
            raw: OCARaw::new(encoding),
        }
    }

    #[wasm_bindgen(js_name = "addName")]
    pub fn add_name(mut self, names: ITranslations) -> OCA {
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
    pub fn add_description(mut self, descriptions: ITranslations) -> OCA {
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
    pub fn add_attribute(mut self, attr: IAttribute) -> OCA {
        let attr_raw: AttributeRaw = attr.into_serde().unwrap();
        self.raw = self.raw.add_attribute(attr_raw);
        self
    }

    pub fn finalize(self) -> IOCA {
        IOCA::from(
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

    pub fn validate(self, oca: IOCA) -> JsValue {
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
pub struct Attribute {
    raw: AttributeRaw,
}

#[wasm_bindgen]
impl Attribute {
    #[wasm_bindgen(constructor)]
    pub fn new(name: String, attr_type: AttributeType) -> Attribute {
        Attribute {
            raw: AttributeRaw::new(name, attr_type),
        }
    }

    #[wasm_bindgen(js_name = "setPii")]
    pub fn set_pii(mut self) -> Attribute {
        self.raw = self.raw.set_pii();
        self
    }

    #[wasm_bindgen(js_name = "addEncoding")]
    pub fn add_encoding(mut self, encoding: Encoding) -> Attribute {
        self.raw = self.raw.add_encoding(encoding);
        self
    }

    #[wasm_bindgen(js_name = "addFormat")]
    pub fn add_format(mut self, format: String) -> Attribute {
        self.raw = self.raw.add_format(format);
        self
    }

    #[wasm_bindgen(js_name = "addUnit")]
    pub fn add_unit(mut self, unit: String) -> Attribute {
        self.raw = self.raw.add_unit(unit);
        self
    }

    #[wasm_bindgen(js_name = "addLabel")]
    pub fn add_label(mut self, labels: ITranslations) -> Attribute {
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
    pub fn add_entries(mut self, entries: Vec<IEntry>) -> Attribute {
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
    pub fn add_information(mut self, information: ITranslations) -> Attribute {
        let information_str: HashMap<String, String> =
            serde_wasm_bindgen::from_value(JsValue::from(information)).unwrap();

        let mut information_raw: HashMap<Language, String> = HashMap::new();
        for (lang, info) in information_str.iter() {
            information_raw.insert(lang.to_string(), info.clone());
        }

        self.raw = self.raw.add_information(information_raw);
        self
    }

    pub fn build(self) -> IAttribute {
        IAttribute::from(
            JsValue::from_serde(&self.raw).unwrap_or(JsValue::NULL)
        )
    }
}

#[wasm_bindgen(typescript_custom_section)]
const OCA_TYPE: &'static str = r#"
interface ICaptureBase {
  schema_type: string,
  classification: string,
  attributes: { [attr_name: string]: string },
  pii: string[]
}

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

type Overlay = CharacterEncodingOverlay
  | EntryOverlay
  | EntryCodeOverlay
  | FormatOverlay
  | InformationOverlay
  | LabelOverlay
  | MetaOverlay
  | UnitOverlay

interface IOCA {
  capture_base: ICaptureBase;
  overlays: Overlay[];
}
"#;

#[wasm_bindgen(typescript_custom_section)]
const ATTRIBUTE_TYPE: &'static str = r#"
interface IAttributeTranslation {
  label?: string,
  entries?: { [entry_code: string]: string }
  information?: string
}

interface IAttribute {
  name: string
  attr_type: string
  is_pii: boolean
  translations: { [language: string]: IAttributeTranslation }
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
