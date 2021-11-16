use said::derivation::SelfAddressing;
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;

mod capture_base;
mod overlay;
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
                if overlay_type.contains("/character_encoding/") {
                    return Ok(Box::new(
                        de_overlay
                            .deserialize_into::<overlay::CharacterEncoding>()
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

#[derive(Serialize)]
pub struct OCABuilder {
    pub oca: OCA,
    #[serde(skip)]
    pub meta_translations: HashMap<Language, OCATranslation>,
}

impl<'de> Deserialize<'de> for OCABuilder {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let de_oca = serde_value::Value::deserialize(deserializer)?;
        if let serde_value::Value::Map(ref oca) = de_oca {
            let capture_base;
            let overlays;
            let mut meta_translations: HashMap<Language, OCATranslation> = HashMap::new();

            match oca.get(&serde_value::Value::String("capture_base".to_string())) {
                Some(de_capture_base) => {
                    capture_base = de_capture_base
                        .clone()
                        .deserialize_into::<CaptureBase>()
                        .unwrap();
                }
                None => return Err(serde::de::Error::missing_field("capture_base")),
            }
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
        }
    }

    pub fn add_classification(mut self, classification: String) -> OCABuilder {
        self.oca.capture_base.classification = classification;
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
            let mut unit_ov = self
                .oca
                .overlays
                .iter_mut()
                .find(|x| x.overlay_type().contains("/unit/"));
            if unit_ov.is_none() {
                self.oca.overlays.push(overlay::Unit::new());
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
        self
    }

    pub fn finalize(mut self) -> OCA {
        for (lang, translation) in self.meta_translations.iter() {
            self.oca
                .overlays
                .push(overlay::Meta::new(lang.to_string(), translation));
        }

        let cs_json = serde_json::to_string(&self.oca.capture_base).unwrap();
        let sai = format!("{}", SelfAddressing::Blake3_256.derive(cs_json.as_bytes()));
        for o in self.oca.overlays.iter_mut() {
            o.sign(&sai);
        }
        self.oca
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
        attribute::{AttributeBuilder, AttributeType, Entry},
        encoding::Encoding,
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
            .add_name(hashmap! {
                "En".to_string() => "Driving Licence".to_string(),
                "Pl".to_string() => "Prawo Jazdy".to_string(),
            })
            .add_description(hashmap! {
                "En".to_string() => "DL desc".to_string(),
                "Pl".to_string() => "PJ desc".to_string(),
            });

        let attr1 = AttributeBuilder::new(String::from("n1"), AttributeType::Text)
            .set_pii()
            .add_label(hashmap! {
                "En".to_string() => "Name: ".to_string(),
                "Pl".to_string() => "Imię: ".to_string(),
            })
            .add_entries(vec![
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
            ])
            .add_information(hashmap! {
                "En".to_string() => "info en".to_string(),
                "Pl".to_string() => "info pl".to_string(),
            })
            .add_unit("days".to_string())
            .build();

        let attr2 = AttributeBuilder::new(String::from("n2"), AttributeType::Date)
            .add_label(hashmap! {
                "En".to_string() => "Date: ".to_string(),
                "Pl".to_string() => "Data: ".to_string(),
            })
            .add_encoding(Encoding::Iso8859_1)
            .add_format("DD/MM/YYYY".to_string())
            .build();

        let oca = oca_builder
            .add_attribute(attr1)
            .add_attribute(attr2)
            .finalize();

        // println!(
        //     "{}",
        //     serde_json::to_string_pretty(&serde_json::to_value(&oca).unwrap()).unwrap()
        // );

        assert_eq!(oca.capture_base.attributes.len(), 2);
        assert_eq!(oca.capture_base.pii.len(), 1);
    }
}
