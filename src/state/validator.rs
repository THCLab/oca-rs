use crate::state::{
    language::Language,
    oca::{DynOverlay, OCATranslation, OCA},
};
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub enum Error {
    Custom(String),
    MissingTranslations(Language),
    MissingMetaTranslation(Language, String),
    UnexpectedTranslations(Language),
    MissingAttributeTranslation(Language, String),
}

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

        if !oca.meta_translations.is_empty() {
            if let Err(meta_errors) = self.validate_meta(&enforced_langs, &oca.meta_translations) {
                errors = errors
                    .into_iter()
                    .chain(meta_errors.into_iter().map(|e| {
                        if let Error::UnexpectedTranslations(lang) = e {
                            Error::Custom(format!(
                                "meta overlay: translations in {:?} language are not enforced",
                                lang
                            ))
                        } else if let Error::MissingTranslations(lang) = e {
                            Error::Custom(format!(
                                "meta overlay: translations in {:?} language are missing",
                                lang
                            ))
                        } else if let Error::MissingMetaTranslation(lang, attr) = e {
                            Error::Custom(format!(
                                "meta overlay: for '{}' translation in {:?} language is missing",
                                attr, lang
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
                        .contains(format!("/{}/", overlay_type).as_str())
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
                                format!("{} overlay: translations in {:?} language are not enforced", overlay_type, lang)
                            )
                        } else if let Error::MissingTranslations(lang) = e {
                            Error::Custom(
                                format!("{} overlay: translations in {:?} language are missing", overlay_type, lang)
                            )
                        } else if let Error::MissingAttributeTranslation(lang, attr_name) = e {
                            Error::Custom(
                                format!("{} overlay: for '{}' attribute missing translations in {:?} language", overlay_type, attr_name, lang)
                            )
                        } else {
                            e
                        }
                    })
                ).collect();
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
            errors.push(Error::UnexpectedTranslations(m.to_string()));
        }

        let missing_translations: HashSet<&_> =
            enforced_langs.difference(&translation_langs).collect();
        for m in missing_translations {
            errors.push(Error::MissingTranslations(m.to_string()));
        }

        let (name_defined, name_undefined): (Vec<_>, Vec<_>) =
            translations.iter().partition(|(_lang, t)| t.name.is_some());
        if !name_defined.is_empty() {
            let name_undefined_langs: HashSet<_> = name_undefined.iter().map(|x| x.0).collect();
            for m in name_undefined_langs {
                errors.push(Error::MissingMetaTranslation(
                    m.to_string(),
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
                    m.to_string(),
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
            errors.push(Error::UnexpectedTranslations(m.to_string()));
        }

        let missing_translations: HashSet<&_> = enforced_langs.difference(&overlay_langs).collect();
        for m in missing_translations {
            errors.push(Error::MissingTranslations(m.to_string()));
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
    use crate::state::{
        attribute::{Attribute, AttributeType},
        encoding::Encoding,
    };
    use maplit::hashmap;

    #[test]
    fn validate_valid_oca() {
        let validator =
            Validator::new().enforce_translations(vec!["En".to_string(), "Pl".to_string()]);

        let oca = OCA::new(Encoding::Utf8)
            .add_name(hashmap! {
                "En".to_string() => "Driving Licence".to_string(),
                "Pl".to_string() => "Prawo Jazdy".to_string(),
            })
            .add_description(hashmap! {
                "En".to_string() => "DL".to_string(),
                "Pl".to_string() => "PJ".to_string(),
            })
            .add_attribute(
                Attribute::new("name".to_string(), AttributeType::Text).add_label(hashmap! {
                    "En".to_string() => "Name: ".to_string(),
                    "Pl".to_string() => "Imię: ".to_string(),
                }),
            )
            .add_attribute(
                Attribute::new("age".to_string(), AttributeType::Number)
                    .add_label(hashmap! {
                        "En".to_string() => "Age: ".to_string(),
                        "Pl".to_string() => "Wiek: ".to_string(),
                    })
                    .add_format("asd".to_string()),
            )
            .finalize();

        let result = validator.validate(&oca);

        assert!(result.is_ok());
    }
}