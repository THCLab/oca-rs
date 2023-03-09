use crate::state::oca::overlay::Overlay;
use crate::state::oca::DynOverlay;
use std::collections::{HashMap, HashSet};
use isolang::Language;

use super::oca::{OCABundle, overlay};

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

    pub fn validate(self, oca_bundle: &OCABundle) -> Result<(), Vec<Error>> {
        let enforced_langs: HashSet<_> = self.enforced_translations.iter().collect();
        let mut errors: Vec<Error> = vec![];

        /* let oca_bundle: OCABundle = serde_json::from_str(oca_str.as_str())
            .map_err(|e| vec![Error::Custom(e.to_string())])?;
 */
        let capture_base = &oca_bundle.capture_base;
        let sai = capture_base.said.clone();

        if capture_base.said.ne(&capture_base.calculate_said()) {
            errors.push(Error::Custom("capture_base: Malformed SAID".to_string()));
        }

        for o in &oca_bundle.overlays {
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

            let meta_overlays = oca_bundle
                .overlays
                .iter()
                .filter_map(|x| x.as_any().downcast_ref::<overlay::Meta>())
                .collect::<Vec<_>>();

            if !meta_overlays.is_empty() {
                if let Err(meta_errors) =
                    self.validate_meta(&enforced_langs, meta_overlays)
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
                let typed_overlays: Vec<_> = oca_bundle
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
        meta_overlays: Vec<&overlay::Meta>,
    ) -> Result<(), Vec<Error>> {
        let mut errors: Vec<Error> = vec![];
        let translation_langs: HashSet<_> = meta_overlays.iter().map(|o| o.language().unwrap()).collect();

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

        for meta_overlay in meta_overlays {
            if meta_overlay.attr_pairs.get("name").is_none() {
                errors.push(Error::MissingMetaTranslation(
                    *meta_overlay.language().unwrap(),
                    "name".to_string(),
                ));
            }

            if meta_overlay.attr_pairs.get("description").is_none() {
                errors.push(Error::MissingMetaTranslation(
                    *meta_overlay.language().unwrap(),
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
                    *overlay.language().unwrap(),
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
        attribute::{Attribute, AttributeType},
        oca::OCABox,
        encoding::Encoding,
        oca::overlay::meta::Metas,
        oca::overlay::character_encoding::CharacterEncodings,
        oca::overlay::label::Labels,
    };

    #[test]
     fn validate_valid_oca() {
        let validator =
            Validator::new().enforce_translations(vec![Language::Eng, Language::Pol]);

        let mut oca = cascade! {
            OCABox::new();
            ..add_meta(Language::Eng, "name".to_string(), "Driving Licence".to_string());
            ..add_meta(Language::Eng, "description".to_string(), "DL".to_string());
            ..add_meta(Language::Pol, "name".to_string(), "Prawo Jazdy".to_string());
            ..add_meta(Language::Pol, "description".to_string(), "PJ".to_string());
        };

        let attribute = cascade! {
            Attribute::new("name".to_string());
            ..set_attribute_type(AttributeType::Text);
            ..set_encoding(Encoding::Utf8);
            ..set_label(Language::Eng, "Name: ".to_string());
            ..set_label(Language::Pol, "Imię: ".to_string());
        };

        oca.add_attribute(attribute);

        let attribute_2 = cascade! {
            Attribute::new("age".to_string());
            ..set_attribute_type(AttributeType::Numeric);
            ..set_label(Language::Eng, "Age: ".to_string());
            ..set_label(Language::Pol, "Wiek: ".to_string());
        };

        oca.add_attribute(attribute_2);

        let oca_bundle = oca.generate_bundle();

        let result = validator.validate(&oca_bundle);

        if let Err(ref errors) = result {
            println!("{errors:?}");
        }
        assert!(result.is_ok());
    }

    #[test]
    fn validate_oca_with_missing_name_translation() {
         let validator =
            Validator::new().enforce_translations(vec![Language::Eng, Language::Pol]);

        let mut oca = cascade! {
            OCABox::new();
            ..add_meta(Language::Eng, "name".to_string(), "Driving Licence".to_string());
            ..add_meta(Language::Eng, "description".to_string(), "Driving Licence desc".to_string());
            ..add_meta(Language::Pol, "description".to_string(), "Driving Licence desc".to_string());
        };

        let oca_bundle = oca.generate_bundle();

        let result = validator.validate(&oca_bundle);

        assert!(result.is_err());
        if let Err(errors) = result {
            assert_eq!(errors.len(), 1);
        }
    }

    #[test]
    fn validate_oca_with_standards() {
/*         let validator = Validator::new();

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
        } */
    }

    #[test]
    fn validate_oca_with_invalid_saids() {
        let validator = Validator::new();
        let data = r#"
{
    "version": "OCAB10000023_",
    "said": "",
    "capture_base": {
        "type": "spec/capture_base/1.0",
        "said": "ElNWOR0fQbv_J6EL0pJlvCxEpbu4bg1AurHgr_0A7LK",
        "classification": "",
        "attributes": {
            "n1": "Text",
            "n2": "DateTime",
            "n3": "Reference:sai"
        },
        "flagged_attributes": ["n1"]
    },
    "overlays": {
        "character_encoding": {
            "capture_base": "ElNWOR0fQbv_J6EL0pJlvCxEpbu4bg1AurHgr_0A7LKc",
            "said": "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
            "type": "spec/overlays/character_encoding/1.0",
            "default_character_encoding": "utf-8",
            "attribute_character_encoding": {}
        }
    }
}
        "#;
        let oca_bundle = load_oca(&mut data.as_bytes()).unwrap();

        let result = validator.validate(&oca_bundle);

        assert!(result.is_err());
        if let Err(errors) = result {
            assert_eq!(errors.len(), 3);
        }
    }
}
