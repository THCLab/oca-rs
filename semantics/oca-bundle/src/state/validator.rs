use crate::state::oca::overlay::Overlay;
use crate::state::oca::DynOverlay;
use indexmap::IndexMap;
use isolang::Language;
use oca_ast_semantics::ast::{AttributeType, NestedAttrType, OverlayType};
use std::{
    collections::{HashMap, HashSet},
    error::Error as StdError,
};

use super::oca::{overlay, OCABundle};
use piccolo::{Closure, Lua, Thread};

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

pub enum SemanticValidationStatus {
    Valid,
    Invalid(Vec<Error>),
}

pub fn validate(oca_bundle: &OCABundle) -> Result<SemanticValidationStatus, String> {
    let validator = Validator::new();
    match validator.validate(oca_bundle) {
        Ok(_) => Ok(SemanticValidationStatus::Valid),
        Err(errors) => Ok(SemanticValidationStatus::Invalid(errors)),
    }
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
            .chain(languages)
            .collect::<Vec<Language>>();
        self
    }

    pub fn validate(self, oca_bundle: &OCABundle) -> Result<(), Vec<Error>> {
        let enforced_langs: HashSet<_> = self.enforced_translations.iter().collect();
        let mut errors: Vec<Error> = vec![];

        /* let oca_bundle: OCABundle = serde_json::from_str(oca_str.as_str())
                   .map_err(|e| vec![Error::Custom(e.to_string())])?;
        */
        let mut recalculated_oca_bundle = oca_bundle.clone();
        recalculated_oca_bundle.fill_said();

        if oca_bundle.said.ne(&recalculated_oca_bundle.said) {
            errors.push(Error::Custom("OCA Bundle: Malformed SAID".to_string()));
        }

        let capture_base = &oca_bundle.capture_base;

        let mut recalculated_capture_base = capture_base.clone();
        recalculated_capture_base.sign();

        if capture_base.said.ne(&recalculated_capture_base.said) {
            errors.push(Error::Custom("capture_base: Malformed SAID".to_string()));
        }

        for o in &oca_bundle.overlays {
            let mut recalculated_overlay = o.clone();
            recalculated_overlay.fill_said();
            if o.said().ne(recalculated_overlay.said()) {
                let msg = match o.language() {
                    Some(lang) => format!("{} ({}): Malformed SAID", o.overlay_type(), lang),
                    None => format!("{}: Malformed SAID", o.overlay_type()),
                };
                errors.push(Error::Custom(msg));
            }

            if o.capture_base().ne(&capture_base.said) {
                let msg = match o.language() {
                    Some(lang) => {
                        format!("{} ({}): Mismatch capture_base SAI", o.overlay_type(), lang)
                    }
                    None => format!("{}: Mismatch capture_base SAI", o.overlay_type()),
                };
                errors.push(Error::Custom(msg));
            }
        }

        let conditional_overlay = oca_bundle
            .overlays
            .iter()
            .find_map(|x| x.as_any().downcast_ref::<overlay::Conditional>());

        if let Some(conditional_overlay) = conditional_overlay {
            self.validate_conditional(
                oca_bundle.capture_base.attributes.clone(),
                conditional_overlay,
            )?;
        }

        if !enforced_langs.is_empty() {
            let meta_overlays = oca_bundle
                .overlays
                .iter()
                .filter_map(|x| x.as_any().downcast_ref::<overlay::Meta>())
                .collect::<Vec<_>>();

            if !meta_overlays.is_empty() {
                if let Err(meta_errors) = self.validate_meta(&enforced_langs, meta_overlays) {
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

            for overlay_type in &[
                OverlayType::Entry,
                OverlayType::Information,
                OverlayType::Label,
            ] {
                let typed_overlays: Vec<_> = oca_bundle
                    .overlays
                    .iter()
                    .filter(|x| x.overlay_type().eq(overlay_type))
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

    fn validate_conditional(
        &self,
        attr_types: IndexMap<String, NestedAttrType>,
        overlay: &overlay::Conditional,
    ) -> Result<(), Vec<Error>> {
        let mut errors: Vec<Error> = vec![];

        let conditions = overlay.attribute_conditions.clone();
        let dependencies = overlay.attribute_dependencies.clone();
        let re = regex::Regex::new(r"\$\{(\d+)\}").unwrap();
        for &attr in overlay.attributes().iter() {
            let condition = conditions.get(attr).unwrap(); // todo
            let condition_dependencies = dependencies.get(attr).unwrap(); // todo
            if condition_dependencies.contains(attr) {
                errors.push(Error::Custom(format!(
                    "Attribute '{attr}' cannot be a dependency of itself"
                )));
                continue;
            }

            let mut attr_mocks: HashMap<String, String> = HashMap::new();
            condition_dependencies.iter().for_each(|dep| {
                let dep_type = attr_types.get(dep).unwrap(); // todo
                let value = match dep_type {
                    NestedAttrType::Null => "null".to_string(),
                    NestedAttrType::Value(base_type) => match base_type {
                        AttributeType::Text => "'test'".to_string(),
                        AttributeType::Numeric => "0".to_string(),
                        AttributeType::DateTime => "'2020-01-01'".to_string(),
                        AttributeType::Binary => "test".to_string(),
                        AttributeType::Boolean => "true".to_string(),
                    },
                    // TODO validate nested objects
                    NestedAttrType::Array(boxed_type) => match **boxed_type {
                        NestedAttrType::Value(base_type) => match base_type {
                            AttributeType::Text => "['test']".to_string(),
                            AttributeType::Numeric => "[0]".to_string(),
                            AttributeType::DateTime => "['2020-01-01']".to_string(),
                            AttributeType::Binary => "[test]".to_string(),
                            AttributeType::Boolean => "[true]".to_string(),
                        },
                        _ => panic!("Invalid or not supported array type"),
                    },
                    NestedAttrType::Reference(ref_value) => ref_value.to_string(),
                };
                attr_mocks.insert(dep.to_string(), value);
            });

            let script = re
                .replace_all(condition, |caps: &regex::Captures| {
                    attr_mocks
                        .get(&condition_dependencies[caps[1].parse::<usize>().unwrap()].clone())
                        .unwrap()
                        .to_string()
                })
                .to_string();

            let mut lua = Lua::new();
            let thread_result = lua.try_run(|ctx| {
                let closure = Closure::load(ctx, format!("return {script}").as_bytes())?;
                let thread = Thread::new(&ctx);
                thread.start(ctx, closure.into(), ())?;
                Ok(ctx.state.registry.stash(&ctx, thread))
            });

            match thread_result {
                Ok(thread) => {
                    if let Err(e) = lua.run_thread::<bool>(&thread) {
                        errors.push(Error::Custom(format!(
                            "Attribute '{attr}' has invalid condition: {}",
                            e.source().unwrap()
                        )));
                    }
                }
                Err(e) => {
                    errors.push(Error::Custom(format!(
                        "Attribute '{attr}' has invalid condition: {}",
                        e.source().unwrap()
                    )));
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
        let translation_langs: HashSet<_> = meta_overlays
            .iter()
            .map(|o| o.language().unwrap())
            .collect();

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

        let attributes = meta_overlays
            .iter()
            .flat_map(|o| o.attr_pairs.keys())
            .collect::<HashSet<_>>();

        for meta_overlay in meta_overlays {
            attributes.iter().for_each(|attr| {
                if !meta_overlay.attr_pairs.contains_key(*attr) {
                    errors.push(Error::MissingMetaTranslation(
                        *meta_overlay.language().unwrap(),
                        attr.to_string(),
                    ));
                }
            });
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
            errors.push(Error::MissingTranslations(**m)); // why we have && here?
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
        encoding::Encoding,
        oca::overlay::character_encoding::CharacterEncodings,
        oca::overlay::conditional::Conditionals,
        oca::overlay::label::Labels,
        oca::overlay::meta::Metas,
        oca::OCABox,
    };

    #[test]
    fn validate_valid_oca() {
        let validator = Validator::new().enforce_translations(vec![Language::Eng, Language::Pol]);

        let mut oca = cascade! {
            OCABox::new();
            ..add_meta(Language::Eng, "name".to_string(), "Driving Licence".to_string());
            ..add_meta(Language::Eng, "description".to_string(), "DL".to_string());
            ..add_meta(Language::Pol, "name".to_string(), "Prawo Jazdy".to_string());
            ..add_meta(Language::Pol, "description".to_string(), "PJ".to_string());
        };

        let attribute = cascade! {
            Attribute::new("name".to_string());
            ..set_attribute_type(NestedAttrType::Value(AttributeType::Text));
            ..set_encoding(Encoding::Utf8);
            ..set_label(Language::Eng, "Name: ".to_string());
            ..set_label(Language::Pol, "Imię: ".to_string());
        };

        oca.add_attribute(attribute);

        let attribute_2 = cascade! {
            Attribute::new("age".to_string());
            ..set_attribute_type(NestedAttrType::Value(AttributeType::Numeric));
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
        let validator = Validator::new().enforce_translations(vec![Language::Eng, Language::Pol]);

        let mut oca = cascade! {
            OCABox::new();
            ..add_meta(Language::Eng, "name".to_string(), "Driving Licence".to_string());
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
    "said": "EBQMQm_tXSC8tnNICl7paGUeGg0SyF1tceHhTUutn1PN",
    "capture_base": {
        "type": "spec/capture_base/1.0",
        "said": "EBQMQm_tXSC8tnNICl7paGUeGg0SyF1tceHhTUutn1PN",
        "classification": "",
        "attributes": {
            "n1": "Text",
            "n2": "DateTime",
            "n3": "refs:EBQMQm_tXSC8tnNICl7paGUeGg0SyF1tceHhTUutn1aP"
        },
        "flagged_attributes": ["n1"]
    },
    "overlays": {
        "character_encoding": {
            "capture_base": "EDRt2wL8yVWVSJdF8aMFtU9VQ6aWzXZTgWj3WqsIKLqm",
            "said": "EBQMQm_tXSC8tnNICl7paGUeGg0SyF1tceHhTUutn1PN",
            "type": "spec/overlays/character_encoding/1.0",
            "default_character_encoding": "utf-8",
            "attribute_character_encoding": {}
        }
    }
}
        "#;
        let oca_bundle = load_oca(&mut data.as_bytes());
        match oca_bundle {
            Ok(oca_bundle) => {
                let result = validator.validate(&oca_bundle);
                assert!(result.is_err());
                if let Err(errors) = result {
                    println!("{:?}", errors);
                    assert_eq!(errors.len(), 4);
                }
            }
            Err(e) => {
                println!("{:?}", e);
                panic!("Failed to load OCA bundle");
            }
        }
    }

    #[test]
    fn validate_oca_with_conditional() {
        let validator = Validator::new();

        let mut oca = OCABox::new();

        let attribute_age = cascade! {
            Attribute::new("age".to_string());
            ..set_attribute_type(NestedAttrType::Value(AttributeType::Numeric));
            ..set_encoding(Encoding::Utf8);
        };

        oca.add_attribute(attribute_age);

        let attribute_name = cascade! {
            Attribute::new("name".to_string());
            ..set_attribute_type(NestedAttrType::Value(AttributeType::Text));
            ..set_condition(
                "${age} > 18 and ${age} < 30".to_string()
            );
        };

        oca.add_attribute(attribute_name);

        let oca_bundle = oca.generate_bundle();
        let result = validator.validate(&oca_bundle);
        assert!(result.is_ok());

        /* println!("{:?}", result);
        assert!(result.is_err());
        if let Err(errors) = result {
            assert_eq!(errors.len(), 1);
        } */
    }
}
