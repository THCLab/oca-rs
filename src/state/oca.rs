use indexmap::IndexMap;
use said::derivation::SelfAddressing;
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::{BTreeMap, HashMap};

mod capture_base;
mod layout;
pub mod overlay;
use crate::state::{
    attribute::Attribute,
    encoding::Encoding,
    language::Language,
    oca::{capture_base::CaptureBase, overlay::Overlay},
};

pub type DynOverlay = Box<dyn Overlay>;

impl<'de> Deserialize<'de> for DynOverlay {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let de_overlay = serde_value::Value::deserialize(deserializer)?;
        if let serde_value::Value::Map(ref overlay) = de_overlay {
            if let Some(serde_value::Value::String(overlay_type)) =
                overlay.get(&serde_value::Value::String("type".to_string()))
            {
                if overlay_type.contains("/mapping/") {
                    return Ok(Box::new(
                        de_overlay
                            .deserialize_into::<overlay::AttributeMapping>()
                            .unwrap(),
                    ));
                } else if overlay_type.contains("/character_encoding/") {
                    return Ok(Box::new(
                        de_overlay
                            .deserialize_into::<overlay::CharacterEncoding>()
                            .unwrap(),
                    ));
                } else if overlay_type.contains("/cardinality/") {
                    return Ok(Box::new(
                        de_overlay
                            .deserialize_into::<overlay::Cardinality>()
                            .unwrap(),
                    ));
                } else if overlay_type.contains("/conformance/") {
                    return Ok(Box::new(
                        de_overlay
                            .deserialize_into::<overlay::Conformance>()
                            .unwrap(),
                    ));
                } else if overlay_type.contains("/conditional/") {
                    return Ok(Box::new(
                        de_overlay
                            .deserialize_into::<overlay::Conditional>()
                            .unwrap(),
                    ));
                } else if overlay_type.contains("/entry/") {
                    return Ok(Box::new(
                        de_overlay.deserialize_into::<overlay::Entry>().unwrap(),
                    ));
                } else if overlay_type.contains("/entry_code/") {
                    return Ok(Box::new(
                        de_overlay.deserialize_into::<overlay::EntryCode>().unwrap(),
                    ));
                } else if overlay_type.contains("/entry_code_mapping/") {
                    return Ok(Box::new(
                        de_overlay
                            .deserialize_into::<overlay::EntryCodeMapping>()
                            .unwrap(),
                    ));
                } else if overlay_type.contains("/format/") {
                    return Ok(Box::new(
                        de_overlay.deserialize_into::<overlay::Format>().unwrap(),
                    ));
                } else if overlay_type.contains("/information/") {
                    return Ok(Box::new(
                        de_overlay
                            .deserialize_into::<overlay::Information>()
                            .unwrap(),
                    ));
                } else if overlay_type.contains("/label/") {
                    return Ok(Box::new(
                        de_overlay.deserialize_into::<overlay::Label>().unwrap(),
                    ));
                } else if overlay_type.contains("/unit/") {
                    return Ok(Box::new(
                        de_overlay.deserialize_into::<overlay::Unit>().unwrap(),
                    ));
                } else if overlay_type.contains("/meta/") {
                    return Ok(Box::new(
                        de_overlay.deserialize_into::<overlay::Meta>().unwrap(),
                    ));
                } else if overlay_type.contains("/form_layout/") {
                    return Ok(Box::new(
                        de_overlay
                            .deserialize_into::<overlay::FormLayout>()
                            .unwrap(),
                    ));
                } else if overlay_type.contains("/credential_layout/") {
                    return Ok(Box::new(
                        de_overlay
                            .deserialize_into::<overlay::CredentialLayout>()
                            .unwrap(),
                    ));
                } else {
                    return Err(serde::de::Error::custom(format!(
                        "unknown overlay type: {}",
                        overlay_type
                    )));
                }
            } else {
                return Err(serde::de::Error::missing_field("type"));
            }
        }

        Err(serde::de::Error::custom(format!(
            "overlay must be an object, got: {:?}",
            de_overlay
        )))
    }
}

#[derive(Serialize, Deserialize)]
pub struct OCA {
    pub capture_base: CaptureBase,
    pub overlays: Vec<DynOverlay>,
}

struct CatAttributes {
    cat_labels: HashMap<String, String>,
    categorized: IndexMap<String, IndexMap<String, Option<String>>>,
    uncategorized: IndexMap<String, Option<String>>,
    lang: String,
}

impl CatAttributes {
    fn add_to_category(
        &mut self,
        categories: Vec<&str>,
        attribute_name: String,
        attribute_sai: Option<String>,
    ) {
        if categories.is_empty() {
            self.uncategorized.insert(attribute_name, attribute_sai);
            return;
        }
        let mut supercats: Vec<i32> = vec![];
        for (i, category) in categories.iter().enumerate() {
            let supercats_str: Vec<String> = supercats.iter().map(|c| c.to_string()).collect();
            let mut supercat = String::new();
            if !supercats_str.is_empty() {
                supercat = format!("-{}", supercats_str.join("-"))
            }
            let regex =
                regex::Regex::new(format!("^_cat{}(-[0-9]*)_$", supercat).as_str()).unwrap();
            let mut acctual_cat_id = String::new();
            let mut category_exists = false;
            for (cat_id, cat_label) in self.cat_labels.iter() {
                if cat_label == category && regex.is_match(cat_id) {
                    let cat_temp = cat_id.replace('_', "");
                    let mut temp = cat_temp.split('-').collect::<Vec<&str>>();
                    temp.remove(0);
                    supercats = temp.iter().map(|c| c.parse::<i32>().unwrap()).collect();
                    acctual_cat_id = cat_id.to_string();
                    category_exists = true;
                }
            }

            if !category_exists {
                let mut count = 0;
                for cat in self.categorized.keys() {
                    if regex.is_match(cat.as_str()) {
                        count += 1;
                    }
                }
                acctual_cat_id = format!("_cat{}-{}_", supercat, count + 1);
                supercats.push(count + 1);
                self.cat_labels
                    .insert(acctual_cat_id.clone(), category.to_string());
                self.categorized
                    .insert(acctual_cat_id.clone(), IndexMap::new());
            }

            if i + 1 == categories.len() {
                self.categorized
                    .get_mut(acctual_cat_id.as_str())
                    .unwrap()
                    .insert(attribute_name.clone(), attribute_sai.clone());
            }
        }
    }
}

#[derive(Serialize)]
pub struct OCABuilder {
    pub oca: OCA,
    #[serde(skip)]
    pub meta_translations: HashMap<Language, OCATranslation>,
    #[serde(skip)]
    pub form_layout: Option<String>,
    #[serde(skip)]
    pub form_layout_reference: BTreeMap<String, String>,
    #[serde(skip)]
    pub credential_layout: Option<String>,
    #[serde(skip)]
    pub credential_layout_reference: BTreeMap<String, String>,
    #[serde(skip)]
    cat_attributes: CatAttributes,
}

impl<'de> Deserialize<'de> for OCABuilder {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let de_oca = serde_value::Value::deserialize(deserializer)?;
        if let serde_value::Value::Map(ref oca) = de_oca {
            let overlays;
            let mut meta_translations: HashMap<Language, OCATranslation> = HashMap::new();

            let capture_base =
                match oca.get(&serde_value::Value::String("capture_base".to_string())) {
                    Some(de_capture_base) => de_capture_base
                        .clone()
                        .deserialize_into::<CaptureBase>()
                        .unwrap(),
                    None => return Err(serde::de::Error::missing_field("capture_base")),
                };
            match oca.get(&serde_value::Value::String("overlays".to_string())) {
                Some(de_overlays) => {
                    if let serde_value::Value::Seq(de_overlays_value) = de_overlays {
                        let meta_overlay_positions: Vec<bool> = de_overlays_value
                            .iter()
                            .map(|x| {
                                if let serde_value::Value::Map(de_overlay_value) = x {
                                    if let Some(serde_value::Value::String(overlay_type)) =
                                        de_overlay_value
                                            .get(&serde_value::Value::String("type".to_string()))
                                    {
                                        overlay_type.contains("/meta/")
                                    } else {
                                        false
                                    }
                                } else {
                                    false
                                }
                            })
                            .collect();
                        let mut meta_overlays_iter = meta_overlay_positions.iter();
                        let mut meta_overlays: Vec<_> = de_overlays_value.clone();
                        meta_overlays.retain(|_| *meta_overlays_iter.next().unwrap());
                        let mut rest_overlays_iter = meta_overlay_positions.iter();
                        let mut rest_overlays: Vec<_> = de_overlays_value.clone();
                        rest_overlays.retain(|_| !*rest_overlays_iter.next().unwrap());

                        for meta_overlay in meta_overlays {
                            if let serde_value::Value::Map(meta_overlay_value) = meta_overlay {
                                let language;
                                let name;
                                let description;
                                match meta_overlay_value
                                    .get(&serde_value::Value::String("language".to_string()))
                                {
                                    Some(de_language) => {
                                        language = de_language
                                            .clone()
                                            .deserialize_into::<Language>()
                                            .unwrap();
                                    }
                                    None => {
                                        return Err(serde::de::Error::missing_field(
                                            "language in meta overlay",
                                        ))
                                    }
                                }
                                match meta_overlay_value
                                    .get(&serde_value::Value::String("name".to_string()))
                                {
                                    Some(de_name) => {
                                        name =
                                            de_name.clone().deserialize_into::<String>().unwrap();
                                    }
                                    None => {
                                        return Err(serde::de::Error::missing_field(
                                            "name in meta overlay",
                                        ))
                                    }
                                }
                                match meta_overlay_value
                                    .get(&serde_value::Value::String("description".to_string()))
                                {
                                    Some(de_description) => {
                                        description = de_description
                                            .clone()
                                            .deserialize_into::<String>()
                                            .unwrap();
                                    }
                                    None => {
                                        return Err(serde::de::Error::missing_field(
                                            "description in meta overlay",
                                        ))
                                    }
                                }
                                let mut t = OCATranslation::new();
                                if !name.trim().is_empty() {
                                    t.add_name(name);
                                }
                                if !description.trim().is_empty() {
                                    t.add_description(description);
                                }
                                meta_translations.insert(language, t);
                            }
                        }

                        let de_rest_overlays = serde_value::Value::Seq(rest_overlays);
                        overlays = de_rest_overlays
                            .deserialize_into::<Vec<DynOverlay>>()
                            .unwrap();
                    } else {
                        return Err(serde::de::Error::custom("overlays must be an array"));
                    }
                }
                None => return Err(serde::de::Error::missing_field("overlay")),
            }

            Ok(OCABuilder {
                oca: OCA {
                    capture_base,
                    overlays,
                },
                meta_translations,
                form_layout: None,
                form_layout_reference: BTreeMap::new(),
                credential_layout: None,
                credential_layout_reference: BTreeMap::new(),
                cat_attributes: CatAttributes {
                    categorized: IndexMap::new(),
                    uncategorized: IndexMap::new(),
                    cat_labels: HashMap::new(),
                    lang: String::new(),
                },
            })
        } else {
            Err(serde::de::Error::custom(format!(
                "OCA must be an object, got: {:?}",
                de_oca
            )))
        }
    }
}

impl OCABuilder {
    pub fn new(default_encoding: Encoding) -> OCABuilder {
        OCABuilder {
            oca: OCA {
                capture_base: CaptureBase::new(),
                overlays: vec![overlay::CharacterEncoding::new(&default_encoding)],
            },
            meta_translations: HashMap::new(),
            form_layout: None,
            form_layout_reference: BTreeMap::new(),
            credential_layout: None,
            credential_layout_reference: BTreeMap::new(),
            cat_attributes: CatAttributes {
                cat_labels: HashMap::new(),
                categorized: IndexMap::new(),
                uncategorized: IndexMap::new(),
                lang: String::new(),
            },
        }
    }

    pub fn add_classification(mut self, classification: String) -> OCABuilder {
        self.oca.capture_base.classification = classification;
        self
    }

    pub fn add_form_layout(mut self, layout: String) -> OCABuilder {
        self.form_layout = Some(layout);
        self
    }

    pub fn add_credential_layout(mut self, layout: String) -> OCABuilder {
        self.credential_layout = Some(layout);
        self
    }

    pub fn add_name(mut self, names: HashMap<Language, String>) -> OCABuilder {
        for (lang, name) in names.iter() {
            match self.meta_translations.get_mut(lang) {
                Some(t) => {
                    t.add_name(name.clone());
                }
                None => {
                    let mut t = OCATranslation::new();
                    t.add_name(name.clone());
                    self.meta_translations.insert(lang.clone(), t);
                }
            }
        }
        self
    }

    pub fn add_description(mut self, descriptions: HashMap<Language, String>) -> OCABuilder {
        for (lang, description) in descriptions.iter() {
            match self.meta_translations.get_mut(lang) {
                Some(t) => {
                    t.add_description(description.clone());
                }
                None => {
                    let mut t = OCATranslation::new();
                    t.add_description(description.clone());
                    self.meta_translations.insert(lang.clone(), t);
                }
            }
        }
        self
    }

    pub fn add_attribute(mut self, attr: Attribute) -> OCABuilder {
        self.oca.capture_base.add(&attr);

        if attr.mapping.is_some() {
            let mut attribute_mapping_ov = self
                .oca
                .overlays
                .iter_mut()
                .find(|x| x.overlay_type().contains("/mapping/"));
            if attribute_mapping_ov.is_none() {
                self.oca.overlays.push(overlay::AttributeMapping::new());
                attribute_mapping_ov = self.oca.overlays.last_mut();
            }

            if let Some(ov) = attribute_mapping_ov {
                ov.add(&attr)
            }
        }

        if attr.condition.is_some() {
            let mut conditional_ov = self
                .oca
                .overlays
                .iter_mut()
                .find(|x| x.overlay_type().contains("/conditional/"));
            if conditional_ov.is_none() {
                self.oca.overlays.push(overlay::Conditional::new());
                conditional_ov = self.oca.overlays.last_mut();
            }
            if let Some(ov) = conditional_ov {
                ov.add(&attr);
            }
        }

        if attr.cardinality.is_some() {
            let mut cardinality_ov = self
                .oca
                .overlays
                .iter_mut()
                .find(|x| x.overlay_type().contains("/cardinality/"));
            if cardinality_ov.is_none() {
                self.oca.overlays.push(overlay::Cardinality::new());
                cardinality_ov = self.oca.overlays.last_mut();
            }

            if let Some(ov) = cardinality_ov {
                ov.add(&attr)
            }
        }

        if attr.conformance.is_some() {
            let mut conformance_ov = self
                .oca
                .overlays
                .iter_mut()
                .find(|x| x.overlay_type().contains("/conformance/"));
            if conformance_ov.is_none() {
                self.oca.overlays.push(overlay::Conformance::new());
                conformance_ov = self.oca.overlays.last_mut();
            }

            if let Some(ov) = conformance_ov {
                ov.add(&attr)
            }
        }

        if attr.encoding.is_some() {
            let encoding_ov = self
                .oca
                .overlays
                .iter_mut()
                .find(|x| x.overlay_type().contains("/character_encoding/"));
            if let Some(ov) = encoding_ov {
                ov.add(&attr);
            }
        }

        if attr.format.is_some() {
            let mut format_ov = self
                .oca
                .overlays
                .iter_mut()
                .find(|x| x.overlay_type().contains("/format/"));
            if format_ov.is_none() {
                self.oca.overlays.push(overlay::Format::new());
                format_ov = self.oca.overlays.last_mut();
            }

            if let Some(ov) = format_ov {
                ov.add(&attr)
            }
        }

        if attr.unit.is_some() {
            let mut unit_ov = self.oca.overlays.iter_mut().find(|x| {
                if let Some(o_metric_system) = x.metric_system() {
                    return o_metric_system == attr.metric_system.as_ref().unwrap()
                        && x.overlay_type().contains("/unit/");
                }
                false
            });
            if unit_ov.is_none() {
                self.oca.overlays.push(overlay::Unit::new(
                    attr.metric_system.as_ref().unwrap().clone(),
                ));
                unit_ov = self.oca.overlays.last_mut();
            }

            if let Some(ov) = unit_ov {
                ov.add(&attr)
            }
        }

        if attr.entry_codes.is_some() {
            let mut entry_code_ov = self
                .oca
                .overlays
                .iter_mut()
                .find(|x| x.overlay_type().contains("/entry_code/"));
            if entry_code_ov.is_none() {
                self.oca.overlays.push(overlay::EntryCode::new());
                entry_code_ov = self.oca.overlays.last_mut();
            }

            if let Some(ov) = entry_code_ov {
                ov.add(&attr)
            }
        }

        if attr.entry_codes_mapping.is_some() {
            let mut entry_code_mapping_ov = self
                .oca
                .overlays
                .iter_mut()
                .find(|x| x.overlay_type().contains("/entry_code_mapping/"));
            if entry_code_mapping_ov.is_none() {
                self.oca.overlays.push(overlay::EntryCodeMapping::new());
                entry_code_mapping_ov = self.oca.overlays.last_mut();
            }

            if let Some(ov) = entry_code_mapping_ov {
                ov.add(&attr)
            }
        }

        if !attr.translations.is_empty() {
            if self.cat_attributes.lang.is_empty() {
                self.cat_attributes.lang = attr.translations.keys().next().unwrap().clone();
            }
            let attr_tr = attr.translations.get(&self.cat_attributes.lang).unwrap();
            if let Some(value) = &attr_tr.label {
                let mut splitted = value.split('|').collect::<Vec<&str>>();
                splitted.pop();
                self.cat_attributes
                    .add_to_category(splitted, attr.name.clone(), attr.sai.clone());
            }
            for (lang, attr_tr) in attr.translations.iter() {
                let mut label_ov = self.oca.overlays.iter_mut().find(|x| {
                    if let Some(o_lang) = x.language() {
                        return o_lang == lang && x.overlay_type().contains("/label/");
                    }
                    false
                });
                if label_ov.is_none() {
                    self.oca
                        .overlays
                        .push(overlay::Label::new(lang.to_string()));
                    label_ov = self.oca.overlays.last_mut();
                }
                if let Some(ov) = label_ov {
                    ov.add(&attr);
                }

                if attr_tr.information.is_some() {
                    let mut information_ov = self.oca.overlays.iter_mut().find(|x| {
                        if let Some(o_lang) = x.language() {
                            return o_lang == lang && x.overlay_type().contains("/information/");
                        }
                        false
                    });
                    if information_ov.is_none() {
                        self.oca
                            .overlays
                            .push(overlay::Information::new(lang.to_string()));
                        information_ov = self.oca.overlays.last_mut();
                    }
                    if let Some(ov) = information_ov {
                        ov.add(&attr);
                    }
                }

                if attr_tr.entries.is_some() {
                    let mut entry_ov = self.oca.overlays.iter_mut().find(|x| {
                        if let Some(o_lang) = x.language() {
                            return o_lang == lang && x.overlay_type().contains("/entry/");
                        }
                        false
                    });
                    if entry_ov.is_none() {
                        self.oca
                            .overlays
                            .push(overlay::Entry::new(lang.to_string()));
                        entry_ov = self.oca.overlays.last_mut();
                    }
                    if let Some(ov) = entry_ov {
                        ov.add(&attr);
                    }
                }
            }
        }
        self
    }

    pub fn finalize(mut self) -> OCA {
        for (lang, translation) in self.meta_translations.iter() {
            self.oca
                .overlays
                .push(overlay::Meta::new(lang.to_string(), translation));
        }
        let mut form_layout_tmp = None;
        match self.form_layout {
            Some(ref layout) => {
                if !layout.is_empty() {
                    form_layout_tmp = Some(layout.clone());
                }
            }
            None => {
                form_layout_tmp = Some(self.build_default_form_layout());
            }
        }
        if let Some(mut layout) = form_layout_tmp {
            for (i, (name, ref_layout)) in self.form_layout_reference.iter().enumerate() {
                if i == 0 {
                    layout.push_str(
                        r#"
reference_layouts:"#,
                    );
                }
                layout.push_str(
                    format!(
                        r#"
  {}:
    {}"#,
                        name,
                        ref_layout.replace('\n', "\n    ")
                    )
                    .as_str(),
                );
            }

            self.oca.overlays.push(overlay::FormLayout::new(layout));
        }

        let mut credential_layout_tmp = None;
        match self.credential_layout {
            Some(ref layout) => {
                if !layout.is_empty() {
                    credential_layout_tmp = Some(layout.clone());
                }
            }
            None => {
                credential_layout_tmp = Some(self.build_default_credential_layout());
            }
        }
        if let Some(mut layout) = credential_layout_tmp {
            for (i, (name, ref_layout)) in self.credential_layout_reference.iter().enumerate() {
                if i == 0 {
                    layout.push_str(
                        r#"
reference_layouts:"#,
                    );
                }
                layout.push_str(
                    format!(
                        r#"
  {}:
    {}"#,
                        name,
                        ref_layout.replace('\n', "\n    ")
                    )
                    .as_str(),
                );
            }

            self.oca
                .overlays
                .push(overlay::CredentialLayout::new(layout));
        }

        let cs_json = serde_json::to_string(&self.oca.capture_base).unwrap();
        let sai = format!("{}", SelfAddressing::Blake3_256.derive(cs_json.as_bytes()));
        for o in self.oca.overlays.iter_mut() {
            o.sign(&sai);
        }

        self.oca
    }

    pub fn build_default_form_layout(&self) -> String {
        let mut layout = String::from(
            r#"config:
  css:
    style: >-
      form {
        background-color: white;
        font-family: 'Lato', 'sans-serif';
        color: #575756;
        padding: 20px;
        border: 1px solid rgba(0, 0, 0, .1);
        border-radius: 5px;
        box-shadow: 0 5px 8px 0 rgb(0 0 0 / 10%), 0 3px 10px 0 rgb(0 0 0 / 10%);
      }
      ._category h1, h2, h3, h4, h5, h6 {
        font-size: 14px;
        font-weight: 500;
        line-height: 1.15em;
        margin: 0.3em 0;
      }
      ._category h1 { font-size: 22px; }
      ._category h2 { font-size: 18px; }
      ._category {
        border: 0;
        border-bottom: 2px dashed #0000004d;
        margin: 10px 0 20px 0;
      }

      ._control {
        margin: 5px 0;
      }
      ._label {
        display: block;
      }
      ._input {
        padding: 0 0.5em;
      }

      ._input[multiple] {
        height: 6.5em;
        display: grid;
        align-items: center;
      }

      ._reference {
        width: 100%;
        height: 100%;
      }

      ._input, .language {
        margin: 2px 0;
        width: 100%;
        height: 2.5em;
        background-color: white;
        border-radius: 3px;
        border-width: 1px;
      }
      ._information {
        color: #6A6A6A;
        font-size: 14px;
        font-style: italic;
        line-height: 1.5;
        text-align: justify;
      }
      .language {
        width: 100px;
        height: 1.75em;
        float: right;
      }
      #submit {
        margin-top: 10px;
        width: 150px;
        height: 2em;
        background-color: white;
        border-width: 1px;
        border-radius: 3px;
      }
      #submit:hover {
        background-color: lightgray;
      }
elements:
  - type: meta
    config:
      css:
        style: >-
          justify-content: space-between;
    parts:
      - name: language
        config:
          css:
            classes: ['language']
      - name: name
        config:
          css:
            style: >-
              font-size: 24px;
              font-weight: 700;
              margin: 10px 0;
      - name: description
        config:
          css:
            style: >-
              font-size: 16px;
              font-weight: 300;
              line-height: 1.5;"#,
        );
        for (attr_name, attr_sai) in self.cat_attributes.uncategorized.iter() {
            layout.push_str(
                format!(
                    r#"
  - type: attribute
    name: {}
    parts:
      - name: label
      - name: input"#,
                    attr_name
                )
                .as_str(),
            );

            if let Some(sai) = attr_sai {
                layout.push_str(
                    format!(
                        r#"
        layout: {}"#,
                        sai
                    )
                    .as_str(),
                );
            }

            layout.push_str(
                r#"
      - name: information"#,
            );
        }
        for (cat, attributes) in &self.cat_attributes.categorized {
            layout.push_str(
                format!(
                    r#"
  - type: category
    id: {}"#,
                    cat
                )
                .as_str(),
            );
            for (attr_name, attr_sai) in attributes.iter() {
                layout.push_str(
                    format!(
                        r#"
  - type: attribute
    name: {}
    parts:
      - name: label
      - name: input"#,
                        attr_name
                    )
                    .as_str(),
                );

                if let Some(sai) = attr_sai {
                    layout.push_str(
                        format!(
                            r#"
        layout: {}"#,
                            sai
                        )
                        .as_str(),
                    );
                }

                layout.push_str(
                    r#"
      - name: information"#,
                );
            }
        }
        layout
    }

    pub fn add_form_layout_reference(&mut self, name: String, layout: String) {
        self.form_layout_reference.insert(name, layout);
    }

    pub fn build_default_credential_layout(&self) -> String {
        let height = (self.oca.capture_base.attributes.len() * 130)
            + (self.cat_attributes.categorized.len() * 65);
        let mut layout = format!(
            r#"version: beta-1
config:
  css:
    width: 630px
    height: {}px
    style: >-
      .language-select {{
        margin: 2px 0;
        background-color: white;
        border-radius: 3px;
        border-width: 1px;
        width: 100px;
        height: 1.75em;
        float: right;
      }}
      body {{
        background-color: white;
        font-family: 'Lato', 'sans-serif';
        color: rgb(87, 87, 86);
        padding: 20px;
        border: 1px solid rgba(0, 0, 0, .1);
        border-radius: 5px;
        box-shadow: 0 5px 8px 0 rgb(0 0 0 / 10%), 0 3px 10px 0 rgb(0 0 0 / 10%);
      }}
      ._category h1, h2, h3, h4, h5, h6 {{
        font-size: 14px;
        font-weight: 500;
        line-height: 1.15em;
        margin: 0.3em 0;
      }}
      ._category h1 {{ font-size: 22px; }}
      ._category h2 {{ font-size: 18px; }}
      ._category {{
        border: 0;
        border-bottom: 2px dashed rgba(0, 0, 0, .4);
        margin: 10px 0 20px 0;
      }}
      ._attribute {{
        border: 1px dashed rgba(0,0,0,0.3);
        min-height: 2em;
        line-height: 2em;
        border-radius: 3px;
        padding: 0 10px;
        margin: 0.5em 0;
      }}
      ._information {{
        color: rgb(106, 106, 106);
        overflow-wrap: anywhere;
        font-size: 14px;
        font-style: italic;
        line-height: 1.5;
        text-align: justify;
      }}
pages:
  - config:
      name: Credential
    elements:
      - type: row
        elements:
          - type: col
            elements:
              - type: oca-name
                config:
                  css:
                    style: >-
                      font-size: 24px;
                      font-weight: 700;
                      margin-bottom: 10px;
      - type: row
        elements:
          - type: col
            elements:
              - type: oca-description
                config:
                  css:
                    style: >-
                      font-size: 16px;
                      font-weight: 300;
                      line-height: 1.5;
"#,
            height
        );
        for (attr_name, attr_sai) in self.cat_attributes.uncategorized.iter() {
            layout.push_str(
                format!(
                    r#"
      - type: row
        elements:
          - type: col
            elements:
              - type: label
                name: {}"#,
                    attr_name
                )
                .as_str(),
            );

            if let Some(sai) = attr_sai {
                layout.push_str(
                    format!(
                        r#"
      - type: row
        elements:
          - type: col
            elements:
              - type: reference
                name: {}
                layout: {}"#,
                        attr_name, sai
                    )
                    .as_str(),
                );
            } else {
                layout.push_str(
                    format!(
                        r#"
      - type: row
        elements:
          - type: col
            elements:
              - type: attribute
                config:
                  css:
                    classes: ['_attribute']
                name: {}"#,
                        attr_name
                    )
                    .as_str(),
                );
            }

            layout.push_str(
                format!(
                    r#"
      - type: row
        elements:
          - type: col
            elements:
              - type: information
                config:
                  css:
                    classes: ['_information']
                name: {}"#,
                    attr_name
                )
                .as_str(),
            );
        }

        for (cat, attributes) in &self.cat_attributes.categorized {
            layout.push_str(
                format!(
                    r#"
      - type: row
        elements:
          - type: col
            elements:
              - type: category
                config:
                  css:
                    classes: ['_category']
                name: {}"#,
                    cat
                )
                .as_str(),
            );
            for (attr_name, attr_sai) in attributes.iter() {
                layout.push_str(
                    format!(
                        r#"
      - type: row
        elements:
          - type: col
            elements:
              - type: label
                name: {}"#,
                        attr_name
                    )
                    .as_str(),
                );

                if let Some(sai) = attr_sai {
                    layout.push_str(
                        format!(
                            r#"
      - type: row
        elements:
          - type: col
            elements:
              - type: reference
                name: {}
                layout: {}"#,
                            attr_name, sai
                        )
                        .as_str(),
                    );
                } else {
                    layout.push_str(
                        format!(
                            r#"
      - type: row
        elements:
          - type: col
            elements:
              - type: attribute
                config:
                  css:
                    classes: ['_attribute']
                name: {}"#,
                            attr_name
                        )
                        .as_str(),
                    );
                }

                layout.push_str(
                    format!(
                        r#"
      - type: row
        elements:
          - type: col
            elements:
              - type: information
                config:
                  css:
                    classes: ['_information']
                name: {}"#,
                        attr_name
                    )
                    .as_str(),
                );
            }
        }
        layout
    }

    pub fn add_credential_layout_reference(&mut self, name: String, layout: String) {
        self.credential_layout_reference.insert(name, layout);
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq)]
pub struct OCATranslation {
    pub name: Option<String>,
    pub description: Option<String>,
}

impl Default for OCATranslation {
    fn default() -> Self {
        Self::new()
    }
}

impl OCATranslation {
    pub fn new() -> OCATranslation {
        OCATranslation {
            name: None,
            description: None,
        }
    }

    pub fn add_name(&mut self, name: String) -> &mut OCATranslation {
        self.name = Some(name);
        self
    }

    pub fn add_description(&mut self, description: String) -> &mut OCATranslation {
        self.description = Some(description);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{
        attribute::{AttributeBuilder, AttributeType, Entries, Entry},
        encoding::Encoding,
        entry_codes::EntryCodes,
    };
    use maplit::hashmap;

    #[test]
    fn build_oca_without_attributes() {
        let oca = OCABuilder::new(Encoding::Utf8)
            .add_classification("GICS:35102020".to_string())
            .add_name(hashmap! {
                "En".to_string() => "Driving Licence".to_string(),
                "Pl".to_string() => "Prawo Jazdy".to_string(),
            })
            .add_description(hashmap! {
                "En".to_string() => "Driving Licence".to_string(),
                "Pl".to_string() => "Prawo Jazdy".to_string(),
            })
            .finalize();

        // println!("{:#?}", serde_json::to_string(&oca).unwrap());

        assert_eq!(oca.capture_base.attributes.len(), 0);
        assert_eq!(oca.capture_base.classification, "GICS:35102020");
    }

    #[test]
    fn build_oca_with_attributes() {
        let oca_builder = OCABuilder::new(Encoding::Utf8)
            .add_form_layout(
                "
                config:
                    css:
                        style: >-
                            form { background-color: white; }
                elements:
                    - type: meta
                      config:
                          css:
                              style: >-
                                  justify-content: space-between;
                      parts:
                          - name: language
                          - name: name
                          - name: description
                    - type: category
                      id: _cat-1_
                    - type: attribute
                      name: n1
                      parts:
                          - name: label
                          - name: input
                          - name: information
                    - type: attribute
                      name: n2
                      parts:
                          - name: label
                          - name: input
                          - name: information
            "
                .to_string(),
            )
            .add_credential_layout(
                "
version: beta-1
config:
  css:
    width: 200px
    height: 100px
    style: >-
      @import url('https://fonts.googleapis.com/css2?family=Nanum+Gothic:wght@700&display=swap');
pages:
  - config:
      name: Page 0
    elements:
      - type: row
        config:
          css:
            style: >-
              height: 32px;
        elements:
      - type: row
        elements:
          - type: col
            size: 4
            config:
              css:
                style: >-
                  padding-right: 0;
            elements:
              - type: row
                elements:
                  - type: col
                    size: 8
                    elements:
                      - type: row
                        elements:
                          - type: col
                            elements:
                            - type: attribute
                              name: n1
labels:
  passport:
    en: Passport
    fr: Passeport
                                   "
                .to_string(),
            )
            .add_name(hashmap! {
                "En".to_string() => "Driving Licence".to_string(),
                "Pl".to_string() => "Prawo Jazdy".to_string(),
            })
            .add_description(hashmap! {
                "En".to_string() => "DL desc".to_string(),
                "Pl".to_string() => "PJ desc".to_string(),
            });

        let attr1 = AttributeBuilder::new(String::from("n1"), AttributeType::Text)
            .set_flagged()
            .add_label(hashmap! {
                "En".to_string() => "Name: ".to_string(),
                "Pl".to_string() => "Imię: ".to_string(),
            })
            .add_entry_codes(EntryCodes::Array(vec![
                "op1".to_string(),
                "op2".to_string(),
            ]))
            .add_entries(Entries::Object(vec![
                Entry::new(
                    "op1".to_string(),
                    hashmap! {
                        "En".to_string() => "Option 1".to_string(),
                        "Pl".to_string() => "Opcja 1".to_string(),
                    },
                ),
                Entry::new(
                    "op2".to_string(),
                    hashmap! {
                        "En".to_string() => "Option 2".to_string(),
                        "Pl".to_string() => "Opcja 2".to_string(),
                    },
                ),
            ]))
            .add_information(hashmap! {
                "En".to_string() => "info en".to_string(),
                "Pl".to_string() => "info pl".to_string(),
            })
            .add_unit("SI".to_string(), "cm".to_string())
            .build();

        let attr2 = AttributeBuilder::new(String::from("n2"), AttributeType::Date)
            .add_label(hashmap! {
                "En".to_string() => "Date: ".to_string(),
                "Pl".to_string() => "Data: ".to_string(),
            })
            .add_condition("${0} == 'op1'".to_string(), vec!["n1".to_string()])
            .add_encoding(Encoding::Iso8859_1)
            .add_format("DD/MM/YYYY".to_string())
            .build();

        let attr3 = AttributeBuilder::new(String::from("n3"), AttributeType::Sai)
            .add_sai("sai".to_string())
            .add_label(hashmap! {
                "En".to_string() => "Reference: ".to_string(),
                "Pl".to_string() => "Referecja: ".to_string(),
            })
            .build();

        let oca = oca_builder
            .add_attribute(attr1)
            .add_attribute(attr2)
            .add_attribute(attr3)
            .finalize();

        // println!(
        //     "{}",
        //     serde_json::to_string_pretty(&serde_json::to_value(&oca).unwrap()).unwrap()
        // );

        assert_eq!(
            oca.capture_base.attributes.get(&"n3".to_string()),
            Some(&"SAI:sai".to_string())
        );
        assert_eq!(oca.capture_base.attributes.len(), 3);
        assert_eq!(oca.capture_base.flagged_attributes.len(), 1);
    }
}
