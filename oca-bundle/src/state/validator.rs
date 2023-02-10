use crate::state::{
    oca::{DynOverlay, OCABuilder, OCATranslation, OCA},
};
use std::collections::{HashMap, HashSet};
use isolang::Language;

#[derive(Debug)]
pub enum Error {
    Custom(String),
    MissingTranslations(Language),
    MissingMetaTranslation(Language, String),
    UnexpectedTranslations(Language),
    MissingAttributeTranslation(Language, String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Custom(error) => write!(f, "{error}"),
            Error::MissingTranslations(language) => {
                write!(f, "Missing translation in {language} language")
            }
            Error::MissingMetaTranslation(language, attr) => write!(
                f,
                "Missing meta translation for {attr} in {language} language"
            ),
            Error::UnexpectedTranslations(language) => {
                write!(f, "Unexpected translations in {language} language")
            }
            Error::MissingAttributeTranslation(language, attr) => {
                write!(f, "Missing translation for {attr} in {language} language")
            }
        }
    }
}

impl std::error::Error for Error {}

pub struct Validator {
    enforced_translations: Vec<Language>,
}

impl Default for Validator {
    fn default() -> Self {
        Self::new()
    }
}

impl Validator {
    pub fn new() -> Validator {
        Validator {
            enforced_translations: vec![],
        }
    }

    pub fn enforce_translations(mut self, languages: Vec<Language>) -> Validator {
        self.enforced_translations = self
            .enforced_translations
            .into_iter()
            .chain(languages.into_iter())
            .collect::<Vec<Language>>();
        self
    }

    pub fn validate(self, oca: &OCA) -> Result<(), Vec<Error>> {
        let enforced_langs: HashSet<_> = self.enforced_translations.iter().collect();
        let mut errors: Vec<Error> = vec![];

        let oca_value = serde_json::value::to_value(oca).unwrap();
        let oca_str = serde_json::to_string(&oca_value).unwrap();
        let oca_builder: OCABuilder = serde_json::from_str(oca_str.as_str())
            .map_err(|e| vec![Error::Custom(e.to_string())])?;

        let capture_base = oca_builder.oca.capture_base;
        let sai = capture_base.said.clone();

        if capture_base.said.ne(&capture_base.calculate_said()) {
            errors.push(Error::Custom("capture_base: Malformed SAID".to_string()));
        }

        for o in oca_builder.oca.overlays {
            if o.said().ne(&o.calculate_said()) {
                let msg = match o.language() {
                    Some(lang) => format!("{} ({}): Malformed SAID", o.overlay_type(), lang),
                    None => format!("{}: Malformed SAID", o.overlay_type()),
                };
                errors.push(Error::Custom(msg));
            }

            if o.capture_base().ne(&sai) {
                let msg = match o.language() {
                    Some(lang) => {
                        format!("{} ({}): Mismatch capture_base SAI", o.overlay_type(), lang)
                    }
                    None => format!("{}: Mismatch capture_base SAI", o.overlay_type()),
                };
                errors.push(Error::Custom(msg));
            }
        }

        if !enforced_langs.is_empty() {
            if !oca_builder.meta_translations.is_empty() {
                if let Err(meta_errors) =
                    self.validate_meta(&enforced_langs, &oca_builder.meta_translations)
                {
                    errors = errors
                        .into_iter()
                        .chain(meta_errors.into_iter().map(|e| {
                            if let Error::UnexpectedTranslations(lang) = e {
                                Error::Custom(format!(
                                    "meta overlay: translations in {lang:?} language are not enforced"
                                ))
                            } else if let Error::MissingTranslations(lang) = e {
                                Error::Custom(format!(
                                    "meta overlay: translations in {lang:?} language are missing"
                                ))
                            } else if let Error::MissingMetaTranslation(lang, attr) = e {
                                Error::Custom(format!(
                                    "meta overlay: for '{attr}' translation in {lang:?} language is missing"
                                ))
                            } else {
                                e
                            }
                        }))
                        .collect();
                }
            }

            for overlay_type in &["entry", "information", "label"] {
                let typed_overlays: Vec<_> = oca
                    .overlays
                    .iter()
                    .filter(|x| {
                        x.overlay_type()
                            .contains(format!("/{overlay_type}/").as_str())
                    })
                    .collect();
                if typed_overlays.is_empty() {
                    continue;
                }

                if let Err(translation_errors) =
                    self.validate_translations(&enforced_langs, typed_overlays)
                {
                    errors = errors.into_iter().chain(
                        translation_errors.into_iter().map(|e| {
                            if let Error::UnexpectedTranslations(lang) = e {
                                Error::Custom(
                                    format!("{overlay_type} overlay: translations in {lang:?} language are not enforced")
                                )
                            } else if let Error::MissingTranslations(lang) = e {
                                Error::Custom(
                                    format!("{overlay_type} overlay: translations in {lang:?} language are missing")
                                )
                            } else if let Error::MissingAttributeTranslation(lang, attr_name) = e {
                                Error::Custom(
                                    format!("{overlay_type} overlay: for '{attr_name}' attribute missing translations in {lang:?} language")
                                )
                            } else {
                                e
                            }
                        })
                    ).collect();
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn validate_meta(
        &self,
        enforced_langs: &HashSet<&Language>,
        translations: &HashMap<Language, OCATranslation>,
    ) -> Result<(), Vec<Error>> {
        let mut errors: Vec<Error> = vec![];
        let translation_langs: HashSet<_> = translations.keys().collect();

        let missing_enforcement: HashSet<&_> =
            translation_langs.difference(enforced_langs).collect();
        for m in missing_enforcement {
            errors.push(Error::UnexpectedTranslations(**m));
        }

        let missing_translations: HashSet<&_> =
            enforced_langs.difference(&translation_langs).collect();
        for m in missing_translations {
            errors.push(Error::MissingTranslations(**m));
        }

        let (name_defined, name_undefined): (Vec<_>, Vec<_>) =
            translations.iter().partition(|(_lang, t)| t.name.is_some());
        if !name_defined.is_empty() {
            let name_undefined_langs: HashSet<_> = name_undefined.iter().map(|x| x.0).collect();
            for m in name_undefined_langs {
                errors.push(Error::MissingMetaTranslation(
                    *m,
                    "name".to_string(),
                ));
            }
        }

        let (desc_defined, desc_undefined): (Vec<_>, Vec<_>) = translations
            .iter()
            .partition(|(_lang, t)| t.description.is_some());
        if !desc_defined.is_empty() {
            let desc_undefined_langs: HashSet<_> = desc_undefined.iter().map(|x| x.0).collect();
            for m in desc_undefined_langs {
                errors.push(Error::MissingMetaTranslation(
                    *m,
                    "description".to_string(),
                ));
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn validate_translations(
        &self,
        enforced_langs: &HashSet<&Language>,
        overlays: Vec<&DynOverlay>,
    ) -> Result<(), Vec<Error>> {
        let mut errors: Vec<Error> = vec![];

        let overlay_langs: HashSet<_> = overlays.iter().map(|x| x.language().unwrap()).collect();

        let missing_enforcement: HashSet<&_> = overlay_langs.difference(enforced_langs).collect();
        for m in missing_enforcement {
            errors.push(Error::UnexpectedTranslations(**m)); // why we have && here?
        }

        let missing_translations: HashSet<&_> = enforced_langs.difference(&overlay_langs).collect();
        for m in missing_translations {
            errors.push(Error::MissingTranslations(Language::from(**m))); // why we have && here?
        }

        let all_attributes: HashSet<&String> =
            overlays.iter().flat_map(|o| o.attributes()).collect();
        for overlay in overlays.iter() {
            let attributes: HashSet<_> = overlay.attributes().into_iter().collect();

            let missing_attr_translation: HashSet<&_> =
                all_attributes.difference(&attributes).collect();
            for m in missing_attr_translation {
                errors.push(Error::MissingAttributeTranslation(
                    overlay.language().unwrap().clone(),
                    m.to_string(),
                ));
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::controller::load_oca;
    use crate::state::{
        attribute::{AttributeType},
        encoding::Encoding,
        oca::OCABuilder,
    };
    use maplit::hashmap;
    #[test]
     fn validate_valid_oca() {
        let validator =
            Validator::new().enforce_translations(vec!["En".to_string(), "Pl".to_string()]);

        let oca = OCABuilder::new(Encoding::Utf8)
            .add_name(hashmap! {
                "En".to_string() => "Driving Licence".to_string(),
                "Pl".to_string() => "Prawo Jazdy".to_string(),
            })
            .add_description(hashmap! {
                "En".to_string() => "DL".to_string(),
                "Pl".to_string() => "PJ".to_string(),
            })
            .add_attribute(
                AttributeBuilder::new("name".to_string(), AttributeType::Text)
                    .add_label(hashmap! {
                        "En".to_string() => "Name: ".to_string(),
                        "Pl".to_string() => "Imię: ".to_string(),
                    })
                    .build(),
            )
            .add_attribute(
                AttributeBuilder::new("age".to_string(), AttributeType::Numeric)
                    .add_label(hashmap! {
                        "En".to_string() => "Age: ".to_string(),
                        "Pl".to_string() => "Wiek: ".to_string(),
                    })
                    .add_format("asd".to_string())
                    .build(),
            )
            .finalize();

        let result = validator.validate(&oca);

        assert!(result.is_ok());
    }

    #[test]
    fn validate_oca_with_missing_name_translation() {
        let validator =
            Validator::new().enforce_translations(vec!["En".to_string(), "Pl".to_string()]);

        let oca = OCABuilder::new(Encoding::Utf8)
            .add_name(hashmap! {
                "En".to_string() => "Driving Licence".to_string(),
            })
            .finalize();

        let result = validator.validate(&oca);

        assert!(result.is_err());
        if let Err(errors) = result {
            assert_eq!(errors.len(), 1);
        }
    }

    #[test]
    fn validate_oca_with_standards() {
        let validator = Validator::new();

        let oca = OCABuilder::new(Encoding::Utf8)
            .add_attribute(
                AttributeBuilder::new("test".to_string(), AttributeType::Text)
                    .add_standard("asd".to_string())
                    .build(),
            )
            .finalize();

        let result = validator.validate(&oca);

        assert!(result.is_err());
        if let Err(errors) = result {
            assert_eq!(errors.len(), 1);
        }
    }

    #[test]
    fn validate_oca_with_invalid_saids() {
        let validator = Validator::new();

        let data = r#"
{
    "capture_base": {
        "type": "spec/capture_base/1.0",
        "digest": "ElNWOR0fQbv_J6EL0pJlvCxEpbu4bg1AurHgr_0A7LK",
        "classification": "",
        "attributes": {
            "n1": "Text",
            "n2": "DateTime",
            "n3": "Reference:sai"
        },
        "flagged_attributes": ["n1"]
    },
    "overlays": [
        {
            "capture_base": "ElNWOR0fQbv_J6EL0pJlvCxEpbu4bg1AurHgr_0A7LKc",
            "digest": "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
            "type": "spec/overlays/character_encoding/1.0",
            "default_character_encoding": "utf-8",
            "attribute_character_encoding": {}
        }
    ]
}
        "#;
        let oca = load_oca(&mut data.as_bytes()).unwrap().oca;

        let result = validator.validate(&oca);

        assert!(result.is_err());
        if let Err(errors) = result {
            assert_eq!(errors.len(), 3);
        }
    }
}