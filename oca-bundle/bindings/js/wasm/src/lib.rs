use oca_bundle::state::{
    attribute::{
        Attribute as AttributeRaw, AttributeBuilder as AttributeBuilderRaw, AttributeType,
        Entries as EntriesRaw, Entry as EntryRaw,
    },
    encoding::Encoding,
    entry_codes::EntryCodes as EntryCodesRaw,
    language::Language,
    oca::{OCABuilder as OCABuilderRaw, OCA as OCARaw},
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
    #[wasm_bindgen(typescript_type = "'O' | 'M'")]
    pub type ConformanceOptions;
    #[wasm_bindgen(typescript_type = "ITranslations")]
    pub type ITranslations;
    #[wasm_bindgen(typescript_type = "IEntry")]
    pub type IEntry;
    #[wasm_bindgen(typescript_type = "{ [language: string]: string } | IEntry[]")]
    pub type Entries;
    #[wasm_bindgen(typescript_type = "string | string[]")]
    pub type EntryCodes;
    #[wasm_bindgen(typescript_type = "string[]")]
    pub type EntryCodesMapping;
    #[wasm_bindgen(typescript_type = "string[]")]
    pub type Dependencies;
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
            translations: translations_str,
        }
    }

    pub fn plain(self) -> IEntry {
        IEntry::from(JsValue::from_serde(&self).unwrap_or(JsValue::NULL))
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

    #[wasm_bindgen(js_name = "addDefaultFormLayout")]
    pub fn add_default_form_layout(mut self) -> OCABuilder {
        self.raw = self.raw.add_default_form_layout();
        self
    }

    #[wasm_bindgen(js_name = "addFormLayout")]
    pub fn add_form_layout(mut self, layout: String) -> OCABuilder {
        self.raw = self.raw.add_form_layout(layout);
        self
    }

    #[wasm_bindgen(js_name = "addDefaultCredentialLayout")]
    pub fn add_default_credential_layout(mut self) -> OCABuilder {
        self.raw = self.raw.add_default_credential_layout();
        self
    }

    #[wasm_bindgen(js_name = "addCredentialLayout")]
    pub fn add_credential_layout(mut self, layout: String) -> OCABuilder {
        self.raw = self.raw.add_credential_layout(layout);
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

    #[wasm_bindgen(js_name = "addMeta")]
    pub fn add_meta(mut self, name: String, values: ITranslations) -> OCABuilder {
        let values_str: HashMap<String, String> =
            serde_wasm_bindgen::from_value(JsValue::from(values)).unwrap();

        let mut values_raw: HashMap<Language, String> = HashMap::new();
        for (lang, value) in values_str.iter() {
            values_raw.insert(lang.to_string(), value.clone());
        }

        self.raw = self.raw.add_meta(name, values_raw);
        self
    }

    #[wasm_bindgen(js_name = "addAttribute")]
    pub fn add_attribute(mut self, attr: Attribute) -> Result<OCABuilder, String> {
        let attr_raw: AttributeRaw = attr.into_serde().map_err(|e| {
            e.to_string()
                .split(" at line")
                .collect::<Vec<&str>>()
                .get(0)
                .unwrap()
                .to_string()
        })?;
        self.raw = self.raw.add_attribute(attr_raw);
        Ok(self)
    }

    pub fn finalize(self) -> OCA {
        OCA::from(JsValue::from_serde(&self.raw.finalize()).unwrap_or(JsValue::NULL))
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
        match oca.into_serde::<OCARaw>() {
            Ok(oca_raw) => {
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
            }
            Err(err) => {
                return_result = ReturnResult {
                    success: false,
                    errors: vec![err.to_string()],
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
    pub fn new(name: String, attribute_type: AttributeType) -> AttributeBuilder {
        AttributeBuilder {
            raw: AttributeBuilderRaw::new(name, attribute_type),
        }
    }

    #[wasm_bindgen(js_name = "setFlagged")]
    pub fn set_flagged(mut self) -> AttributeBuilder {
        self.raw = self.raw.set_flagged();
        self
    }

    #[wasm_bindgen(js_name = "addSai")]
    pub fn add_sai(mut self, sai: String) -> AttributeBuilder {
        self.raw = self.raw.add_sai(sai);
        self
    }

    #[wasm_bindgen(js_name = "addCondition")]
    pub fn add_condition(
        mut self,
        condition: String,
        dependencies: Dependencies,
    ) -> AttributeBuilder {
        let dependencies_value = JsValue::from(dependencies);
        self.raw = self.raw.add_condition(
            condition,
            serde_wasm_bindgen::from_value(dependencies_value).unwrap(),
        );
        self
    }

    #[wasm_bindgen(js_name = "addCardinality")]
    pub fn add_cardinality(mut self, cardinality: String) -> AttributeBuilder {
        self.raw = self.raw.add_cardinality(cardinality);
        self
    }

    #[wasm_bindgen(js_name = "addConformance")]
    pub fn add_conformance(mut self, conformance: ConformanceOptions) -> AttributeBuilder {
        let conformance_value = JsValue::from(conformance);
        self.raw = self
            .raw
            .add_conformance(serde_wasm_bindgen::from_value(conformance_value).unwrap());
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

    #[wasm_bindgen(js_name = "addStandard")]
    pub fn add_standard(mut self, standard: String) -> AttributeBuilder {
        self.raw = self.raw.add_standard(standard);
        self
    }

    #[wasm_bindgen(js_name = "addMapping")]
    pub fn add_mapping(mut self, mapping: String) -> AttributeBuilder {
        self.raw = self.raw.add_mapping(mapping);
        self
    }

    #[wasm_bindgen(js_name = "addUnit")]
    pub fn add_unit(mut self, metric_system: String, unit: String) -> AttributeBuilder {
        self.raw = self.raw.add_unit(metric_system, unit);
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

    #[wasm_bindgen(js_name = "addEntryCodes")]
    pub fn add_entry_codes(mut self, entry_codes: EntryCodes) -> AttributeBuilder {
        let entry_codes_value = JsValue::from(entry_codes);
        let entry_codes_raw: EntryCodesRaw = match entry_codes_value.is_string() {
            true => EntryCodesRaw::Sai(serde_wasm_bindgen::from_value(entry_codes_value).unwrap()),
            false => {
                EntryCodesRaw::Array(serde_wasm_bindgen::from_value(entry_codes_value).unwrap())
            }
        };

        self.raw = self.raw.add_entry_codes(entry_codes_raw);
        self
    }

    #[wasm_bindgen(js_name = "addEntryCodesMapping")]
    pub fn add_entry_codes_mapping(mut self, mappings: EntryCodesMapping) -> AttributeBuilder {
        let mappings_value = JsValue::from(mappings);

        self.raw = self
            .raw
            .add_entry_codes_mapping(serde_wasm_bindgen::from_value(mappings_value).unwrap());
        self
    }

    #[wasm_bindgen(js_name = "addEntries")]
    pub fn add_entries(mut self, entries: Entries) -> AttributeBuilder {
        let entries_value = JsValue::from(entries);
        let entries_raw = match js_sys::Array::is_array(&entries_value) {
            true => {
                let mut entries_raw_vec: Vec<EntryRaw> = vec![];
                let entries_vec: Vec<Entry> =
                    serde_wasm_bindgen::from_value(entries_value).unwrap();
                for entry in entries_vec.iter() {
                    let mut entry_tr_raw: HashMap<Language, String> = HashMap::new();
                    for (lang, entry_v) in entry.translations.iter() {
                        entry_tr_raw.insert(lang.to_string(), entry_v.clone());
                    }
                    entries_raw_vec.push(EntryRaw::new(entry.code.to_string(), entry_tr_raw))
                }
                EntriesRaw::Object(entries_raw_vec)
            }
            false => {
                let entries_sai: HashMap<Language, String> =
                    serde_wasm_bindgen::from_value(entries_value).unwrap();
                EntriesRaw::Sai(entries_sai)
            }
        };

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
        Attribute::from(JsValue::from_serde(&self.raw.build()).unwrap_or(JsValue::NULL))
    }
}

#[wasm_bindgen(typescript_custom_section)]
const OCA_TYPE: &'static str = r#"
type OCA = {
  capture_base: CaptureBase,
  overlays: Overlay[],
  references?: {
    [capture_base_sai: string]: OCA
  }
}

type CaptureBase = {
  type: string,
  digest: string,
  classification: string,
  attributes: { [attribute_name: string]: string },
  flagged_attributes: string[]
}

type Overlay =
  | CardinalityOverlay
  | CharacterEncodingOverlay
  | ConditionalOverlay
  | ConformanceOverlay
  | EntryOverlay
  | EntryCodeOverlay
  | EntryCodeMappingOverlay
  | FormatOverlay
  | InformationOverlay
  | LabelOverlay
  | MappingOverlay
  | MetaOverlay
  | UnitOverlay
  | StandardOverlay
  | SubsetOverlay
  | FormLayoutOverlay
  | CredentialLayoutOverlay

type CardinalityOverlay = {
  capture_base: string,
  digest: string,
  type: string,
  attribute_cardinality: { [attribute_name: string]: string }
}

type CharacterEncodingOverlay = {
  capture_base: string,
  digest: string,
  type: string,
  default_character_encoding: string,
  attribute_character_encoding: { [attribute_name: string]: string }
}

type ConditionalOverlay = {
  capture_base: string,
  digest: string,
  type: string,
  attribute_conditions: { [attribute_name: string]: string },
  attribute_dependencies: { [attribute_name: string]: string[] }
}

type ConformanceOverlay = {
  capture_base: string,
  digest: string,
  type: string,
  attribute_conformance: { [attribute_name: string]: 'O' | 'M' }
}

type CredentialLayoutOverlay = {
  capture_base: string,
  digest: string,
  type: string,
  layout: {
    version: string,
    config?: {
      css?: {
        width?: string,
        height?: string,
        style?: string
      }
    },
    pages: {
      config?: {
        css?: {
          style?: string,
          classes?: string[],
          background_image?: string
        },
        name: string
      },
      elements: {
        type: string,
        size?: string,
        name?: string,
        layout?: string,
        content?: string,
        config?: {
          css?: {
            style?: string
            classes?: string[]
          }
        },
        elements?: CredentialLayoutOverlay['layout']['pages'][0]['elements']
      }[]
    }[],
    labels?: {
      [label: string]: {
        [language: string]: string
      }
    },
    reference_layouts?: {
      [reference_layout: string]: CredentialLayoutOverlay['layout']
    }
  }
}

type EntryOverlay = {
  capture_base: string,
  digest: string,
  type: string,
  language: string,
  attribute_entries: { [attribute_name: string]: { [entry_code: string]: string } }
}

type EntryCodeOverlay = {
  capture_base: string,
  digest: string,
  type: string,
  attribute_entry_codes: { [attribute_name: string]: string[] }
}

type EntryCodeMappingOverlay = {
  capture_base: string,
  digest: string,
  type: string,
  attribute_entry_codes_mapping: { [attribute_name: string]: string[] }
}

type FormLayoutOverlay = {
  capture_base: string,
  digest: string,
  type: string,
  layout: {
    config?: {
      css?: {
        style?: string
      }
    },
    elements: {
      type: string,
      config?: {
        css?: {
          style?: string,
          classes?: string[]
        }
      },
      id?: string,
      name?: string,
      parts?: {
        name: string,
        layout?: string,
        config?: {
          css?: {
            style?: string,
            classes?: string[]
          }
        }
      }[]
    }[],
    reference_layouts?: {
      [reference_layout: string]: FormLayoutOverlay['layout']
    }
  }
}

type FormatOverlay = {
  capture_base: string,
  digest: string,
  type: string,
  attribute_formats: { [attribute_name: string]: string }
}

type InformationOverlay = {
  capture_base: string,
  digest: string,
  type: string,
  language: string,
  attribute_information: { [attribute_name: string]: string }
}

type LabelOverlay = {
  capture_base: string,
  digest: string,
  type: string,
  language: string,
  attribute_labels: { [attribute_name: string]: string }
  attribute_categories: string[],
  category_labels: { [cat_id: string]: string },
  category_attributes: { [cat_id: string]: string[] }
}

type MappingOverlay = {
  capture_base: string,
  digest: string,
  type: string,
  attribute_mapping: { [attribute_name: string]: string }
}

type MetaOverlay = {
  capture_base: string,
  digest: string,
  type: string,
  language: string,
  name: string,
  description: string
}

type UnitOverlay = {
  capture_base: string,
  digest: string,
  type: string,
  metric_system: string,
  attribute_units: { [attribute_name: string]: string }
}

type StandardOverlay = {
  capture_base: string,
  digest: string,
  type: string,
  attribute_standards: { [attribute_name: string]: string }
}

type SubsetOverlay = {
  capture_base: string,
  digest: string,
  type: string,
  attributes: string[]
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
  attribute_type: string
  sai?: string
  is_flagged: boolean
  translations: { [language: string]: AttributeTranslation }
  encoding?: string
  format?: string
  standard?: string
  metric_system?: string
  unit?: string
  entry_codes?: string[]
  entry_codes_mapping?: string[]
  condition?: string
  dependencies?: string[]
  mapping?: string
  cardinality?: string
  conformance?: 'O' | 'M'
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
