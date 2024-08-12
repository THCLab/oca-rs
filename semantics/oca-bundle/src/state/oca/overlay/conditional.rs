use crate::state::{attribute::Attribute, oca::Overlay};
use oca_ast::ast::OverlayType;
use piccolo::{Closure, Lua, Thread};
use said::derivation::HashFunctionCode;
use said::{sad::SerializationFormats, sad::SAD};
use serde::{Deserialize, Serialize};
use std::{any::Any, collections::BTreeMap, error::Error as StdError, fmt::Display};

pub trait Conditionals {
    fn set_condition(&mut self, condition: String);
    fn check_condition(
        &self,
        dependency_values: BTreeMap<String, Box<dyn Display + 'static>>,
    ) -> Result<bool, Vec<Error>>;
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Error {
    Custom(String),
}

impl Conditionals for Attribute {
    fn set_condition(&mut self, condition: String) {
        let re = regex::Regex::new(r"\$\{([^}]*)\}").unwrap();
        let mut dependencies = Vec::new();

        let cond = re
            .replace_all(&condition, |caps: &regex::Captures| {
                let i = dependencies
                    .iter()
                    .position(|d| d == &caps[1])
                    .unwrap_or_else(|| {
                        dependencies.push(caps[1].to_string());
                        dependencies.len() - 1
                    });
                format!("${{{}}}", i)
            })
            .to_string();

        self.condition = Some(cond);
        self.dependencies = Some(dependencies);
    }

    fn check_condition(
        &self,
        dependency_values: BTreeMap<String, Box<dyn Display + 'static>>,
    ) -> Result<bool, Vec<Error>> {
        let mut errors: Vec<Error> = vec![];

        let condition = &self.condition.clone().ok_or(vec![Error::Custom(
            "Attribute has no condition".to_string(),
        )])?;
        let condition_dependencies = &self.dependencies.clone().unwrap();
        let re = regex::Regex::new(r"\$\{(\d+)\}").unwrap();
        let attr = &self.name;
        if condition_dependencies.contains(attr) {
            errors.push(Error::Custom(format!(
                "Attribute '{attr}' cannot be a dependency of itself"
            )));
        }
        condition_dependencies.iter().for_each(|d| {
            if !dependency_values.contains_key(d) {
                errors.push(Error::Custom(format!("Missing dependency '{d}' value",)));
            }
        });

        if !errors.is_empty() {
            return Err(errors);
        }

        let script = re
            .replace_all(condition, |caps: &regex::Captures| {
                dependency_values
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

        let mut result = None;
        match thread_result {
            Ok(thread) => match lua.run_thread::<bool>(&thread) {
                Ok(r) => result = Some(r),
                Err(e) => {
                    errors.push(Error::Custom(format!(
                        "Attribute '{attr}' has invalid condition: {}",
                        e.source().unwrap()
                    )));
                }
            },
            Err(e) => {
                errors.push(Error::Custom(format!(
                    "Attribute '{attr}' has invalid condition: {}",
                    e.source().unwrap()
                )));
            }
        };
        if errors.is_empty() {
            match result.is_some() {
                true => Ok(result.unwrap()),
                false => Err(vec![Error::Custom(format!(
                    "Attribute '{attr}' has invalid condition",
                ))]),
            }
        } else {
            Err(errors)
        }
    }
}

#[derive(SAD, Serialize, Deserialize, Debug, Clone)]
pub struct ConditionalOverlay {
    #[said]
    #[serde(rename = "d")]
    said: Option<said::SelfAddressingIdentifier>,
    #[serde(rename = "type")]
    overlay_type: OverlayType,
    capture_base: Option<said::SelfAddressingIdentifier>,
    pub attribute_conditions: BTreeMap<String, String>,
    pub attribute_dependencies: BTreeMap<String, Vec<String>>,
}

impl Overlay for ConditionalOverlay {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn capture_base(&self) -> &Option<said::SelfAddressingIdentifier> {
        &self.capture_base
    }
    fn set_capture_base(&mut self, said: &said::SelfAddressingIdentifier) {
        self.capture_base = Some(said.clone());
    }
    fn said(&self) -> &Option<said::SelfAddressingIdentifier> {
        &self.said
    }
    fn overlay_type(&self) -> &OverlayType {
        &self.overlay_type
    }
    fn attributes(&self) -> Vec<&String> {
        self.attribute_conditions.keys().collect::<Vec<&String>>()
    }

    fn add(&mut self, attribute: &Attribute) {
        if attribute.condition.is_some() {
            self.attribute_conditions.insert(
                attribute.name.clone(),
                attribute.condition.as_ref().unwrap().clone(),
            );
        }
        if attribute.dependencies.is_some() {
            self.attribute_dependencies.insert(
                attribute.name.clone(),
                attribute.dependencies.as_ref().unwrap().clone(),
            );
        }
    }
}
impl ConditionalOverlay {
    pub fn new() -> Self {
        Self {
            capture_base: None,
            said: None,
            overlay_type: OverlayType::Conditional,
            attribute_conditions: BTreeMap::new(),
            attribute_dependencies: BTreeMap::new(),
        }
    }
}

impl Default for ConditionalOverlay {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use oca_ast::ast::{AttributeType, NestedAttrType};

    struct Dependency {
        name: String,
        value: Box<dyn Display + 'static>,
    }
    impl core::fmt::Debug for Dependency {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{}={}", self.name, self.value)
        }
    }
    impl Dependency {
        fn new(name: &str, value: Box<dyn Display + 'static>) -> Self {
            Self {
                name: name.to_string(),
                value,
            }
        }
    }

    #[test]
    fn test_checking_condition() {
        let setting: Vec<(&str, Vec<Dependency>, bool)> = vec![
            (
                "${age} > 18 and ${age} < 30",
                vec![
                    Dependency::new("age", Box::new(20)),
                    Dependency::new("name", Box::new("new")),
                ],
                true,
            ),
            (
                "${age} > 18 and ${age} < 30",
                vec![Dependency::new("age", Box::new(30))],
                false,
            ),
            (
                "${age} > 18 and ${age} < 30",
                vec![Dependency::new("age", Box::new(18))],
                false,
            ),
            (
                "${age} > 18 and ${age} < 30",
                vec![Dependency::new("age", Box::new(31))],
                false,
            ),
            (
                "${age} > 18 and ${age} < 30",
                vec![Dependency::new("age", Box::new(17))],
                false,
            ),
            (
                "2021-01-01 > ${start_date}",
                vec![Dependency::new("start_date", Box::new("2024-01-01"))],
                false,
            ),
        ];

        for (condition, values, expected) in setting {
            let attribute = cascade! {
                Attribute::new("name".to_string());
                ..set_attribute_type(NestedAttrType::Value(AttributeType::Text));
                ..set_condition(condition.to_string());
            };

            let mut dependency_values: BTreeMap<String, Box<dyn Display + 'static>> =
                BTreeMap::new();
            let mut dependency_values_str = vec![];
            for value in values.into_iter() {
                dependency_values_str.push(format!("{:?}", value));
                dependency_values.insert(value.name, value.value);
            }

            let r = attribute.check_condition(dependency_values);
            assert_eq!(
                r.unwrap(),
                expected,
                "condition: \"{}\", values: [{}]",
                condition,
                dependency_values_str.join(", ")
            );
        }
    }
}
