use oca_bundle::state::oca::overlay::entry::Entries;
use oca_bundle::state::oca::overlay::entry_code::EntryCodes;
use oca_bundle::state::oca::overlay::information::Information;
use oca_bundle::state::oca::overlay::format::Formats;
use oca_bundle::state::oca::overlay::conformance::Conformances;
use oca_bundle::state::oca::overlay::conditional::Conditionals;
use oca_bundle::state::oca::overlay::cardinality::Cardinalitys;
use oca_bundle::state::oca::overlay::character_encoding::CharacterEncodings;
use oca_bundle::state::oca::overlay::label::Labels;
use oca_bundle::state::oca::overlay::meta::Metas;
use oca_bundle::state::oca::overlay::credential_layout::CredentialLayouts;
use oca_bundle::state::oca::overlay::form_layout::FormLayouts;
use isolang::Language;
use oca_bundle::state::{
    attribute::Attribute as AttributeRaw,
    encoding::Encoding,
    entry_codes::EntryCodes as EntryCodesRaw,
    entries::EntriesElement as EntriesElementRaw,
    oca::{OCABundle as OCABundleRaw, OCABox as OCABoxRaw},
    validator,
};
use oca_ast::ast::NestedAttrType;
use oca_bundle::Encode;
use serde::Serialize;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "OCABundle")]
    pub type OCABundle;
    #[wasm_bindgen(typescript_type = "IMeta")]
    pub type MetaTSType;
    #[wasm_bindgen(typescript_type = "IAttribute")]
    pub type AttributeTSType;
    #[wasm_bindgen(typescript_type = "IAttribute[]")]
    pub type AttributesTSType;
    #[wasm_bindgen(typescript_type = "AST")]
    pub type AST;
    #[wasm_bindgen(typescript_type = "'O' | 'M'")]
    pub type ConformanceOptions;
    #[wasm_bindgen(typescript_type = "{ [language: string]: string }")]
    pub type Translations;
    #[wasm_bindgen(typescript_type = "IEntry")]
    pub type IEntry;
    #[wasm_bindgen(typescript_type = "{ [code: string]: { [language: string]: string } }")]
    pub type EntriesTranslations;
    #[wasm_bindgen(typescript_type = "string[]")]
    pub type EntryCodesMapping;
    #[wasm_bindgen(typescript_type = "string[]")]
    pub type Dependencies;
}

#[wasm_bindgen]
pub struct OCABox {
    raw: OCABoxRaw,
}

impl Default for OCABox {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
impl OCABox {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            raw: OCABoxRaw::new(),
        }
    }

    #[wasm_bindgen(js_name = "load")]
    pub fn load(mut self, oca_bundle: OCABundle) -> Self {
        self.raw = OCABoxRaw::from(
            serde_wasm_bindgen::from_value::<OCABundleRaw>(JsValue::from(
                oca_bundle,
            ))
            .unwrap(),
        );
        self
    }

    #[wasm_bindgen(js_name = "attributes")]
    pub fn attributes(&self) -> AttributesTSType {
        AttributesTSType::from(
            self.raw.attributes
                .values()
                .collect::<Vec<_>>()
                .serialize(&serde_wasm_bindgen::Serializer::json_compatible())
                .unwrap()
        )
    }

    #[wasm_bindgen(js_name = "meta")]
    pub fn meta(&self) -> MetaTSType {
        MetaTSType::from(
            self.raw.meta
                .serialize(&serde_wasm_bindgen::Serializer::json_compatible())
                .unwrap()
        )
    }

    #[wasm_bindgen(js_name = "classification")]
    pub fn classification(&self) -> Option<String> {
        self.raw.classification.clone()
    }

    #[wasm_bindgen(js_name = "toAST")]
    pub fn to_ast(&self) -> AST {
        let oca_bundle = self.raw.clone().generate_bundle();
        AST::from(
            oca_bundle.to_ast()
                .serialize(&serde_wasm_bindgen::Serializer::json_compatible())
                .unwrap()
        )
    }

    #[wasm_bindgen(js_name = "addClassification")]
    pub fn add_classification(mut self, classification: String) -> Self {
        self.raw.add_classification(classification);
        self
    }

    #[wasm_bindgen(js_name = "addFormLayout")]
    pub fn add_form_layout(mut self, layout: String) -> Self {
        self.raw.add_form_layout(layout);
        self
    }

    #[wasm_bindgen(js_name = "addCredentialLayout")]
    pub fn add_credential_layout(mut self, layout: String) -> Self {
        self.raw.add_credential_layout(layout);
        self
    }

    #[wasm_bindgen(js_name = "addMeta")]
    pub fn add_meta(mut self, name: String, values: Translations) -> Self {
        let lang_values: HashMap<String, String> =
            serde_wasm_bindgen::from_value(JsValue::from(values)).unwrap();

        for (lang, value) in lang_values.iter() {
            let language_raw = Language::from_639_3(lang).unwrap();
            self.raw.add_meta(language_raw, name.clone(), value.clone());
        }
        self
    }

    #[wasm_bindgen(js_name = "addAttribute")]
    pub fn add_attribute(mut self, attr: Attribute) -> Self {
        self.raw.add_attribute(attr.raw);
        self
    }

    #[wasm_bindgen(js_name = "generateBundle")]
    pub fn generate_bundle(&self) -> OCABundle {
        let mut raw = self.raw.clone();
        let oca_bundle_json_str =
            String::from_utf8(raw.generate_bundle().encode().unwrap())
                .unwrap();
        OCABundle::from(
            serde_json::from_str::<serde_json::Value>(&oca_bundle_json_str)
                .unwrap()
                .serialize(&serde_wasm_bindgen::Serializer::json_compatible())
                .unwrap(),
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
    pub fn enforce_translations(mut self, languages: JsValue) -> Self {
        let languages_str: Vec<String> = serde_wasm_bindgen::from_value(languages).unwrap();

        let langs: Vec<Language> = languages_str
            .iter()
            .map(|lang| Language::from_639_3(lang).unwrap())
            .collect();

        self.raw = self.raw.enforce_translations(langs);
        self
    }

    pub fn validate(self, oca_bundle: OCABundle) -> JsValue {
        #[derive(Serialize)]
        struct ReturnResult {
            success: bool,
            errors: Vec<String>,
        }
        let return_result: ReturnResult;
        match serde_wasm_bindgen::from_value::<OCABundleRaw>(oca_bundle.into())
        {
            Ok(oca_bundle_raw) => {
                let result = self.raw.validate(&oca_bundle_raw);
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

        return_result
            .serialize(&serde_wasm_bindgen::Serializer::json_compatible())
            .unwrap_or(JsValue::NULL)
    }
}

#[wasm_bindgen]
pub struct Attribute {
    raw: AttributeRaw,
}

#[wasm_bindgen]
pub fn create_nested_attr_type_from_js(value: &JsValue) -> Result<JsValue, JsValue> {
    NestedAttrType::from_js_value(value).map(|attr_type| attr_type.to_js_value())
}

#[wasm_bindgen]
impl Attribute {
    #[wasm_bindgen(constructor)]
    pub fn new(name: String) -> Self {
        Self {
            raw: AttributeRaw::new(name),
        }
    }

    #[wasm_bindgen(js_name = "setAttributeType")]
    pub fn set_attribute_type(mut self, attr_type: JsValue) -> Self {
        let attr_type = NestedAttrType::from_js_value(&attr_type);
        match attr_type {
            Ok(attr_type) => {
                self.raw.set_attribute_type(attr_type);
            }
            Err(err) => {
                panic!("Error setting attribute type: {:?}", err);
            }
        }
        self
    }

    #[wasm_bindgen(js_name = "setFlagged")]
    pub fn set_flagged(mut self) -> Self {
        self.raw.set_flagged();
        self
    }

    #[wasm_bindgen(js_name = "merge")]
    pub fn merge(mut self, attr: Attribute) -> Self {
        self.raw.merge(&attr.raw);
        self
    }

    #[wasm_bindgen(js_name = "setEncoding")]
    pub fn set_encoding(mut self, encoding: Encoding) -> Self {
        self.raw.set_encoding(encoding);
        self
    }

    #[wasm_bindgen(js_name = "setCardinality")]
    pub fn set_cardinality(mut self, cardinality: String) -> Self {
        self.raw.set_cardinality(cardinality);
        self
    }

    #[wasm_bindgen(js_name = "setCondition")]
    pub fn set_condition(mut self, condition: String) -> Self {
        self.raw.set_condition(condition);
        self
    }

    #[wasm_bindgen(js_name = "setConformance")]
    pub fn set_conformance(mut self, conformance: ConformanceOptions) -> Self {
        let conformance_raw: String =
            serde_wasm_bindgen::from_value(conformance.into()).unwrap();
        self.raw.set_conformance(conformance_raw);
        self
    }

    #[wasm_bindgen(js_name = "setFormat")]
    pub fn set_format(mut self, format: String) -> Self {
        self.raw.set_format(format);
        self
    }

    #[wasm_bindgen(js_name = "setLabel")]
    pub fn set_label(mut self, labels: Translations) -> Self {
        let lang_labels: HashMap<String, String> =
            serde_wasm_bindgen::from_value(JsValue::from(labels)).unwrap();

        for (lang, label) in lang_labels.iter() {
            let language_raw = Language::from_639_3(lang).unwrap();
            self.raw.set_label(language_raw, label.clone());
        }
        self
    }

    #[wasm_bindgen(js_name = "setInformation")]
    pub fn set_information(mut self, information: Translations) -> Self {
        let lang_information: HashMap<String, String> =
            serde_wasm_bindgen::from_value(JsValue::from(information)).unwrap();

        for (lang, information) in lang_information.iter() {
            let language_raw = Language::from_639_3(lang).unwrap();
            self.raw.set_information(language_raw, information.clone());
        }
        self
    }

    #[wasm_bindgen(js_name = "setEntries")]
    pub fn set_entries(mut self, entries: EntriesTranslations) -> Self {
        let entry_translations: HashMap<String, HashMap<String, String>> =
            serde_wasm_bindgen::from_value(JsValue::from(entries)).unwrap();

        let mut codes: Vec<String> = vec![];
        let mut lang_entries: HashMap<Language, HashMap<String, String>> = HashMap::new();

        for (entry_code, translations) in entry_translations.iter() {
          codes.push(entry_code.clone());
          for (lang, entry) in translations.iter() {
            let language_raw = Language::from_639_3(lang).unwrap();
            if let Some(entry_tr) = lang_entries.get_mut(&language_raw) {
              entry_tr.insert(entry_code.clone(), entry.clone());
            } else {
              let mut entry_tr: HashMap<String, String> = HashMap::new();
              entry_tr.insert(entry_code.clone(), entry.clone());
              lang_entries.insert(language_raw, entry_tr);
            }
          }
        }

        self.raw.set_entry_codes(EntryCodesRaw::Array(codes));
        for (lang, translations) in lang_entries.iter() {
          self.raw.set_entry(*lang, EntriesElementRaw::Object(translations.clone()));
        }

        self
    }

    /*
    #[wasm_bindgen(js_name = "addUnit")]
    pub fn add_unit(mut self, metric_system: String, unit: String) -> AttributeBuilder {
        self.raw = self.raw.add_unit(metric_system, unit);
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
    */
}

#[wasm_bindgen(typescript_custom_section)]
const ATTRIBUTE_TYPE: &'static str = r#"
type IMeta = {
  [language: string]: {
    [attribute_name: string]: string
  }
}

type IAttribute = {
  name: string
  type: string
  is_flagged: boolean
  labels?: { [language: string]: string }
  category_labels?: { [language: string]: string }
  informations?: { [language: string]: string }
  entry_codes?: string[] | string
  entries?: { [language: string]: string | { [entry_code: string]: string } }
  mapping?: string
  encoding?: string
  format?: string
  units?: { [measurement_system: string]: string }
  entry_codes_mapping?: string[]
  reference_sai?: string
  condition?: string
  dependencies?: string[]
  cardinality?: string
  conformance?: 'O' | 'M'
  standards?: string[]
}
"#;

#[wasm_bindgen(typescript_custom_section)]
const AST_TYPE: &'static str = r#"
type Command = {
  type: 'Add',
  object_kind: string,
  content: {
    attributes?: { [attribute_name: string]: string },
    properties?: { [property_name: string]: string }
  }
}

type EntryCodeCommand = {
  type: 'Add',
  object_kind: 'EntryCode',
  content: {
    attributes?: {
      [attribute_name: string]: string | string[]
    },
    properties?: { [property_name: string]: string }
  }
}

type EntryCommand = {
  type: 'Add',
  object_kind: 'Entry',
  content: {
    attributes?: {
      [attribute_name: string]: string | {
        [entry_code: string]: string
      }
    },
    properties?: { [property_name: string]: string }
  }
}

type AST = {
  version: string,
  commands: (Command | EntryCodeCommand | EntryCommand)[],
  commands_meta?: object
}
"#;

#[wasm_bindgen(typescript_custom_section)]
const OCA_TYPE: &'static str = r#"
type OCABundle = {
  v: string,
  d: string,
  capture_base: CaptureBase,
  overlays: Overlays,
  references?: {
    [capture_base_sai: string]: OCABundle
  }
}

type CaptureBase = {
  type: string,
  d: string,
  classification: string,
  attributes: { [attribute_name: string]: string },
  flagged_attributes: string[]
}

type Overlays = {
  cardinality?: CardinalityOverlay,
  character_encoding?: CharacterEncodingOverlay,
  conditional?: ConditionalOverlay,
  conformance?: ConformanceOverlay,
  entry?: EntryOverlay[],
  entry_code?: EntryCodeOverlay,
  entry_code_mapping?: EntryCodeMappingOverlay,
  format?: FormatOverlay,
  information?: InformationOverlay[],
  label?: LabelOverlay[],
  mapping?: MappingOverlay,
  meta?: MetaOverlay[],
  unit?: UnitOverlay[],
  standard?: StandardOverlay,
  subset?: SubsetOverlay,
  form_layout?: FormLayoutOverlay,
  credential_layout?: CredentialLayoutOverlay
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
  d: string,
  type: string,
  attribute_cardinality: { [attribute_name: string]: string }
}

type CharacterEncodingOverlay = {
  capture_base: string,
  d: string,
  type: string,
  default_character_encoding: string,
  attribute_character_encoding: { [attribute_name: string]: string }
}

type ConditionalOverlay = {
  capture_base: string,
  d: string,
  type: string,
  attribute_conditions: { [attribute_name: string]: string },
  attribute_dependencies: { [attribute_name: string]: string[] }
}

type ConformanceOverlay = {
  capture_base: string,
  d: string,
  type: string,
  attribute_conformance: { [attribute_name: string]: 'O' | 'M' }
}

type CredentialLayoutOverlay = {
  capture_base: string,
  d: string,
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
  d: string,
  type: string,
  language: string,
  attribute_entries: { [attribute_name: string]: { [entry_code: string]: string } }
}

type EntryCodeOverlay = {
  capture_base: string,
  d: string,
  type: string,
  attribute_entry_codes: { [attribute_name: string]: string[] }
}

type EntryCodeMappingOverlay = {
  capture_base: string,
  d: string,
  type: string,
  attribute_entry_codes_mapping: { [attribute_name: string]: string[] }
}

type FormLayoutOverlay = {
  capture_base: string,
  d: string,
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
  d: string,
  type: string,
  attribute_formats: { [attribute_name: string]: string }
}

type InformationOverlay = {
  capture_base: string,
  d: string,
  type: string,
  language: string,
  attribute_information: { [attribute_name: string]: string }
}

type LabelOverlay = {
  capture_base: string,
  d: string,
  type: string,
  language: string,
  attribute_labels: { [attribute_name: string]: string }
  attribute_categories: string[],
  category_labels: { [cat_id: string]: string },
  category_attributes: { [cat_id: string]: string[] }
}

type MappingOverlay = {
  capture_base: string,
  d: string,
  type: string,
  attribute_mapping: { [attribute_name: string]: string }
}

type MetaOverlay = {
  capture_base: string,
  d: string,
  type: string,
  language: string,
  name: string,
  description: string
}

type UnitOverlay = {
  capture_base: string,
  d: string,
  type: string,
  metric_system: string,
  attribute_units: { [attribute_name: string]: string }
}

type StandardOverlay = {
  capture_base: string,
  d: string,
  type: string,
  attribute_standards: { [attribute_name: string]: string }
}

type SubsetOverlay = {
  capture_base: string,
  d: string,
  type: string,
  attributes: string[]
}
"#;

/*
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
*/
