use crate::state::oca::overlay::information::Information;
use crate::state::oca::overlay::entry::Entries;
use crate::state::oca::overlay::entry_code::EntryCodes;
use crate::state::oca::overlay::unit::{Unit, AttributeUnit};
#[cfg(feature = "format_overlay")]
use crate::state::oca::overlay::format::Formats;
use crate::state::oca::overlay::cardinality::Cardinalitys;
use crate::state::oca::overlay::conditional::Conditionals;
use crate::state::oca::overlay::conformance::Conformances;
use crate::state::oca::overlay::meta::Metas;
use crate::state::oca::overlay::character_encoding::CharacterEncodings;
use crate::state::oca::overlay::label::Labels;
use indexmap::IndexMap;
use said::version::SerializationInfo;
use said::sad::{SerializationFormats, SAD};
use crate::state::oca::layout::credential::Layout as CredentialLayout;
use crate::state::oca::layout::form::Layout as FormLayout;
use serde::{Deserialize, Deserializer, Serialize, Serializer, ser::SerializeMap};
use std::collections::HashMap;
use linked_hash_map::LinkedHashMap;
pub mod capture_base;
mod layout;
pub mod overlay;
use isolang::Language;
use crate::state::{
    attribute::Attribute,
    oca::{capture_base::CaptureBase, overlay::Overlay},
};
use oca_ast::ast::{OverlayType, NestedValue, OCAAst, Command, CommandType, ObjectKind, Content, CaptureContent};
use convert_case::{Case, Casing};
/// Internal representation of OCA objects in split between non-attributes values and attributes.
/// It is used to build dynamically objects without knowing yet whole structure of the object.
/// Used mainly as a container to hold information while parsing OCAfile.
/// Example of usage:
///
/// let oca = OCABox::new()
/// let attr = Attribute::new("name")
/// oca.add_attribute(attr)
/// oca.get_attribute_by_name("name").setEncoding(Encoding::UTF8)
/// oca.get_attribute_by_name("name").setLabel(Language::English, "Name")
/// oca.get_attribute_by_name("name").setInformation(Language::German, "Name")
/// oca.get_attribute_by_name("name").setUnit("kg")
/// oca.get_attribute_by_name("name").setStandard("ISO 1234")
/// oca.get_attribute_by_name("name").setCategory("personal")
/// oca.generate_bundle().unwrap()
///
///
/// TODO:
/// How to add multiple overlays like mapping or layout (how to identify them?)

#[derive(Clone)]
pub struct OCABox {
    pub attributes: HashMap<String, Attribute>,
    pub credential_layouts: Option<Vec<CredentialLayout>>,
    pub form_layouts: Option<Vec<FormLayout>>,
    pub mappings: Option<Vec<overlay::AttributeMapping>>,
    pub meta: Option<HashMap<Language, HashMap<String, String>>>,
    pub classification: Option<String>,
}

impl Default for OCABox {
    fn default() -> Self {
        Self::new()
    }
}

impl OCABox {
    pub fn new() -> Self {
        OCABox {
            attributes: HashMap::new(),
            credential_layouts: None,
            form_layouts: None,
            mappings: None,
            meta: None,
            classification: None,
        }
    }
    /// Add an attribute to the OCA Bundle
    /// If the attribute already exists, it will be merged with the new attribute
    /// for simple types: the new value will overwrite the old value
    /// for complex types: the new value will be added to the old value
    pub fn add_attribute(&mut self, attribute: Attribute) {
        if let Some(attr) = self.get_attribute_mut(&attribute.name) {
            attr.merge(&attribute);
        } else {
            self.attributes.insert(attribute.name.clone(), attribute);
        }
    }
    pub fn get_attribute_by_name(&self, name: &str) -> Option<&Attribute> {
        self.attributes.get(name)
    }

    pub fn add_attribute_mapping(&mut self, mapping: overlay::AttributeMapping) {
        match self.mappings {
            Some(ref mut mappings) => mappings.push(mapping),
            None => self.mappings = Some(vec![mapping]),
        }
    }
    pub fn add_classification(&mut self, classification: String) {
        self.classification = Some(classification);
    }

    pub fn generate_bundle(&mut self) -> OCABundle {
        let mut capture_base = self.generate_capture_base();
        let mut overlays = self.generate_overlays();

        capture_base.sign();

        let cb_said = capture_base.said.as_ref();
        overlays.iter_mut().for_each(|x| x.sign(cb_said.unwrap()));

        let mut oca_bundle = OCABundle {
            said: None,
            capture_base,
            overlays,
        };

        oca_bundle.compute_digest();
        oca_bundle
    }

    fn generate_overlays(&mut self) -> Vec<DynOverlay> {
        let mut overlays: Vec<DynOverlay> = Vec::new();
        if let Some(mappings) = &self.mappings {
            for mapping in mappings {
                overlays.push(Box::new(mapping.clone()));
            }
        }
        if let Some(meta) = &self.meta {
            for (lang, attr_pairs) in meta {
                let meta_ov = overlay::Meta::new(
                    *lang,
                    attr_pairs.clone()
                );
                overlays.push(Box::new(meta_ov));
            }
        }
        if let Some(layouts) = &self.form_layouts {
            for layout in layouts {
                let layout_ov = overlay::FormLayout::new(layout.clone());
                overlays.push(Box::new(layout_ov));
            }
        }
        if let Some(layouts) = &self.credential_layouts {
            for layout in layouts {
                let layout_ov = overlay::CredentialLayout::new(layout.clone());
                overlays.push(Box::new(layout_ov));
            }
        }

        for attribute in self.attributes.values() {
            if attribute.encoding.is_some() {
                let mut encoding_ov = overlays
                    .iter_mut()
                    .find(|x| x.overlay_type().eq(&OverlayType::CharacterEncoding));
                if encoding_ov.is_none() {
                    overlays.push(Box::new(overlay::CharacterEncoding::new()));
                    encoding_ov = overlays.last_mut();
                }
                if let Some(ov) = encoding_ov {
                    ov.add(attribute);
                }
            }

            #[cfg(feature = "format_overlay")]
            if attribute.format.is_some() {
                let mut format_ov = overlays
                    .iter_mut()
                    .find(|x| x.overlay_type().eq(&OverlayType::Format));
                if format_ov.is_none() {
                    overlays.push(Box::new(overlay::Format::new()));
                    format_ov = overlays.last_mut();
                }
                if let Some(ov) = format_ov {
                    ov.add(attribute);
                }
            }

            if attribute.conformance.is_some() {
                let mut conformance_ov = overlays
                    .iter_mut()
                    .find(|x| x.overlay_type().eq(&OverlayType::Conformance));
                if conformance_ov.is_none() {
                    overlays.push(Box::new(overlay::Conformance::new()));
                    conformance_ov = overlays.last_mut();
                }
                if let Some(ov) = conformance_ov {
                    ov.add(attribute);
                }
            }

            if attribute.cardinality.is_some() {
                let mut cardinality_ov = overlays
                    .iter_mut()
                    .find(|x| x.overlay_type().eq(&OverlayType::Cardinality));
                if cardinality_ov.is_none() {
                    overlays.push(Box::new(overlay::Cardinality::new()));
                    cardinality_ov = overlays.last_mut();
                }
                if let Some(ov) = cardinality_ov {
                    ov.add(attribute);
                }
            }

            if attribute.condition.is_some() {
                let mut conditional_ov = overlays
                    .iter_mut()
                    .find(|x| x.overlay_type().eq(&OverlayType::Conditional));
                if conditional_ov.is_none() {
                    overlays.push(Box::new(overlay::Conditional::new()));
                    conditional_ov = overlays.last_mut();
                }
                if let Some(ov) = conditional_ov {
                    ov.add(attribute);
                }
            }

            if let Some(units) = &attribute.units {
                for measurement_system in units.keys() {
                    let mut unit_ov = overlays
                        .iter_mut()
                        .find(|x| if let Some(x_unit) = x.as_any().downcast_ref::<overlay::Unit>() {

                            x_unit.measurement_system() == Some(measurement_system)
                        } else {
                            false
                        });
                    if unit_ov.is_none() {
                        overlays.push(Box::new(overlay::Unit::new(measurement_system.clone())));
                        unit_ov = overlays.last_mut();
                    }
                    if let Some(ov) = unit_ov {
                        ov.add(attribute);
                    }
                }
            }

            if attribute.entry_codes.is_some() {
                let mut entry_code_ov = overlays
                    .iter_mut()
                    .find(|x| x.overlay_type().eq(&OverlayType::EntryCode));
                if entry_code_ov.is_none() {
                    overlays.push(Box::new(overlay::EntryCode::new()));
                    entry_code_ov = overlays.last_mut();
                }
                if let Some(ov) = entry_code_ov {
                    ov.add(attribute);
                }
            }

            if let Some(entries) = &attribute.entries {
                for lang in entries.keys() {
                    let mut entry_ov = overlays
                        .iter_mut()
                        .find(|x| x.overlay_type().eq(&OverlayType::Entry) && x.language() == Some(lang));
                    if entry_ov.is_none() {
                        overlays.push(Box::new(overlay::Entry::new(*lang)));
                        entry_ov = overlays.last_mut();
                    }
                    if let Some(ov) = entry_ov {
                        ov.add(attribute);
                    }
                }
            }

            if let Some(labels) = &attribute.labels {
                for lang in labels.keys() {
                    let mut label_ov = overlays
                        .iter_mut()
                        .find(|x| x.overlay_type().eq(&OverlayType::Label) && x.language() == Some(lang));
                    if label_ov.is_none() {
                        overlays.push(Box::new(overlay::Label::new(*lang)));
                        label_ov = overlays.last_mut();
                    }
                    if let Some(ov) = label_ov {
                        ov.add(attribute);
                    }
                }
            }

            if let Some(information) = &attribute.informations {
                for lang in information.keys() {
                    let mut info_ov = overlays
                        .iter_mut()
                        .find(|x| x.overlay_type().eq(&OverlayType::Information) && x.language() == Some(lang));
                    if info_ov.is_none() {
                        overlays.push(Box::new(overlay::Information::new(*lang)));
                        info_ov = overlays.last_mut();
                    }
                    if let Some(ov) = info_ov {
                        ov.add(attribute);
                    }
                }
            }
        }

        overlays
    }
    fn generate_capture_base(&mut self) -> CaptureBase {
        let mut capture_base = CaptureBase::new();
        if let Some(classification) = &self.classification {
            capture_base.set_classification(classification);
        }
        for attribute in self.attributes.values() {
            capture_base.add(attribute);
        }
        capture_base
    }
    fn get_attribute_mut(&mut self, name: &str) -> Option<&mut Attribute> {
        self.attributes.get_mut(name)
    }
}

pub type DynOverlay = Box<dyn Overlay + Send + Sync + 'static>;

impl<'de> Deserialize<'de> for DynOverlay {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let de_overlay = serde_value::Value::deserialize(deserializer)?;
        if let serde_value::Value::Map(ref overlay) = de_overlay {
            if let Some(de_overlay_type) =
                overlay.get(&serde_value::Value::String("type".to_string()))
            {
                let overlay_type = de_overlay_type.clone().deserialize_into::<OverlayType>().map_err(|e| {
                    serde::de::Error::custom(format!("Overlay type: {e}"))
                })?;

                match overlay_type {
                    OverlayType::AttributeMapping => {
                        return Ok(Box::new(
                            de_overlay
                                .deserialize_into::<overlay::AttributeMapping>()
                                .map_err(|e| {
                                    serde::de::Error::custom(format!(
                                        "Attribute Mapping overlay: {e}"
                                    ))
                                })?,
                        ));
                    }
                    OverlayType::CharacterEncoding => {
                        return Ok(Box::new(
                            de_overlay
                                .deserialize_into::<overlay::CharacterEncoding>(
                                )
                                .map_err(|e| {
                                    serde::de::Error::custom(format!(
                                        "Character Encoding overlay: {e}"
                                    ))
                                })?,
                        ));
                    }
                    OverlayType::Cardinality => {
                        return Ok(Box::new(
                            de_overlay
                                .deserialize_into::<overlay::Cardinality>()
                                .map_err(|e| {
                                    serde::de::Error::custom(format!(
                                        "Cardinality overlay: {e}"
                                    ))
                                })?,
                        ));
                    }
                    OverlayType::Conformance => {
                        return Ok(Box::new(
                            de_overlay
                                .deserialize_into::<overlay::Conformance>()
                                .map_err(|e| {
                                    serde::de::Error::custom(format!(
                                        "Conformance overlay: {e}"
                                    ))
                                })?,
                        ));
                    }
                    OverlayType::Conditional => {
                        return Ok(Box::new(
                            de_overlay
                                .deserialize_into::<overlay::Conditional>()
                                .map_err(|e| {
                                    serde::de::Error::custom(format!(
                                        "Conditional overlay: {e}"
                                    ))
                                })?,
                        ));
                    }
                    OverlayType::Entry => {
                        return Ok(Box::new(
                            de_overlay
                                .deserialize_into::<overlay::Entry>()
                                .map_err(|e| {
                                    serde::de::Error::custom(format!(
                                        "Entry overlay: {e}"
                                    ))
                                })?,
                        ));
                    }
                    OverlayType::EntryCode => {
                        return Ok(Box::new(
                            de_overlay
                                .deserialize_into::<overlay::EntryCode>()
                                .map_err(|e| {
                                    serde::de::Error::custom(format!(
                                        "Entry Code overlay: {e}"
                                    ))
                                })?,
                        ));
                    }
                    OverlayType::EntryCodeMapping => {
                        return Ok(Box::new(
                            de_overlay
                                .deserialize_into::<overlay::EntryCodeMapping>()
                                .map_err(|e| {
                                    serde::de::Error::custom(format!(
                                        "Entry Code Mapping overlay: {e}"
                                    ))
                                })?,
                        ));
                    }
                    OverlayType::Unit => {
                        return Ok(Box::new(
                            de_overlay
                                .deserialize_into::<overlay::Unit>()
                                .map_err(|e| {
                                    serde::de::Error::custom(format!(
                                        "Unit overlay: {e}"
                                    ))
                                })?,
                        ));
                    }

                    #[cfg(feature = "format_overlay")]
                    OverlayType::Format => {
                        return Ok(Box::new(
                            de_overlay
                                .deserialize_into::<overlay::Format>()
                                .map_err(|e| {
                                    serde::de::Error::custom(format!(
                                        "Format overlay: {e}"
                                    ))
                                })?,
                        ));
                    }

                    OverlayType::Information => {
                        return Ok(Box::new(
                            de_overlay
                                .deserialize_into::<overlay::Information>()
                                .map_err(|e| {
                                    serde::de::Error::custom(format!(
                                        "Information overlay: {e}"
                                    ))
                                })?,
                        ));
                    }
                    OverlayType::Label => {
                        return Ok(Box::new(
                            de_overlay
                                .deserialize_into::<overlay::Label>()
                                .map_err(|e| {
                                    serde::de::Error::custom(format!(
                                        "Label overlay: {e}"
                                    ))
                                })?,
                        ));
                    }
                    OverlayType::Meta => {
                        return Ok(Box::new(
                            de_overlay
                                .deserialize_into::<overlay::Meta>()
                                .map_err(|e| {
                                    serde::de::Error::custom(format!(
                                        "Meta overlay: {e}"
                                    ))
                                })?,
                        ));
                    }

                    /* OverlayType::FormLayout => {
                        return Ok(Box::new(
                            de_overlay
                                .deserialize_into::<overlay::FormLayout>()
                                .map_err(|e| {
                                    serde::de::Error::custom(format!("Form Layout overlay: {e}"))
                                })?,
                        ));
                    }
                    OverlayType::CredentialLayout => {
                        return Ok(Box::new(
                            de_overlay
                                .deserialize_into::<overlay::CredentialLayout>()
                                .map_err(|e| {
                                    serde::de::Error::custom(format!("Credential Layout overlay: {e}"))
                                })?,
                        ));
                    } */
                    OverlayType::Subset => {
                        return Ok(Box::new(
                            de_overlay
                                .deserialize_into::<overlay::Subset>()
                                .map_err(|e| {
                                    serde::de::Error::custom(format!(
                                        "Subset overlay: {e}"
                                    ))
                                })?,
                        ));
                    }
                    OverlayType::Standard => {
                        return Ok(Box::new(
                            de_overlay
                                .deserialize_into::<overlay::Standard>()
                                .map_err(|e| {
                                    serde::de::Error::custom(format!(
                                        "Standard overlay: {e}"
                                    ))
                                })?,
                        ));
                    }
                    _ => {
                        return Err(serde::de::Error::custom(format!(
                            "Overlay type not supported: {:?}",
                            overlay_type
                        )));
                    }
                }
            } else {
                return Err(serde::de::Error::missing_field("type"));
            }
        }

        Err(serde::de::Error::custom(format!(
            "overlay must be an object, got: {de_overlay:?}"
        )))
    }
}

pub fn serialize_overlays<S>(overlays: &Vec<DynOverlay>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    use serde_value::Value;
    #[derive(Serialize)]
    #[serde(untagged)]
    enum OverlayValue {
        Array(Vec<DynOverlay>),
        Object(Box<dyn Overlay + Send>),
    }

    let mut overlays_map: LinkedHashMap<Value, OverlayValue> = LinkedHashMap::new();
    let overlays_order = [
        OverlayType::CharacterEncoding,
        OverlayType::Format,
        OverlayType::Meta,
        OverlayType::Label,
        OverlayType::Information,
        OverlayType::Standard,
        OverlayType::Conditional,
        OverlayType::Conformance,
        OverlayType::EntryCode,
        OverlayType::Entry,
        OverlayType::Cardinality,
        OverlayType::Unit,
        OverlayType::AttributeMapping,
        OverlayType::EntryCodeMapping,
        OverlayType::UnitMapping,
        OverlayType::Subset,
        /* OverlayType::CredentialLayout,
        OverlayType::FormLayout, */
    ];
    for o_type in overlays_order {
        for overlay in overlays {
            let o_type_str = o_type.to_string().to_case(Case::Snake);
            if overlay.overlay_type().eq(&o_type) {
                match overlay.language() {
                    Some(_) => {
                        if let Some(OverlayValue::Array(ov)) = overlays_map.get_mut(&Value::String(o_type_str.clone())) {
                            ov.push(overlay.clone());
                        } else {
                            overlays_map.insert(
                                Value::String(o_type_str.clone()),
                                OverlayValue::Array(vec![
                                    overlay.clone()
                                ])
                            );
                        }
                    },
                    None => {
                        overlays_map.insert(
                            Value::String(o_type_str),
                            OverlayValue::Object(overlay.clone())
                        );
                    }
                }
            }
        }
    }

    let mut ser = s.serialize_map(Some(overlays_map.len()))?;
    for (ov_type, v) in overlays_map.iter_mut() {
        if let OverlayValue::Array(ov) = v {
            ov.sort_by(|a, b| {
                if let Some(a_lang) = a.language() {
                    if let Some(b_lang) = b.language() {
                        a_lang.cmp(b_lang)
                    } else {
                        std::cmp::Ordering::Equal
                    }
                } else {
                    std::cmp::Ordering::Equal
                }
            });
        }

        ser.serialize_entry(ov_type, v)?;
    }
    ser.end()
}

fn deserialize_overlays<'de, D>(deserializer: D) -> Result<Vec<DynOverlay>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    struct OverlaysVisitor;

    impl<'de> serde::de::Visitor<'de> for OverlaysVisitor {
        type Value = Vec<DynOverlay>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("vector of overlays")
        }

        fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
        where
            V: serde::de::MapAccess<'de>,
        {
            let mut overlays = vec![];

            while let Some((_, value)) = map.next_entry::<String, serde_value::Value>()? {
                if let serde_value::Value::Seq(ov) = value {
                    for o in ov {
                        overlays.push(o.deserialize_into().unwrap());
                    }
                } else if let serde_value::Value::Map(_) = value {
                    overlays.push(value.deserialize_into().unwrap());
                }
            }

            Ok(overlays)
        }
    }

    deserializer.deserialize_any(OverlaysVisitor)
}

impl std::fmt::Debug for DynOverlay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DynOverlay {{ overlay_type: {}, attributes: {:?} }}", self.overlay_type(), self.attributes())
    }
}

#[derive(SAD, Serialize, Debug, Deserialize, Clone)]
#[version(protocol = "OCAB", major = 1, minor = 0)]
// #[said(format = "JSON")]
pub struct OCABundle {
    #[said]
    #[serde(rename = "d")]
    pub said: Option<said::SelfAddressingIdentifier>,
    pub capture_base: CaptureBase,
    #[serde(serialize_with = "serialize_overlays", deserialize_with = "deserialize_overlays")]
    pub overlays: Vec<DynOverlay>,
}

impl From<OCABundle> for OCABox {
    fn from(oca_bundle: OCABundle) -> Self {
        let mut oca_box = OCABox::new();
        oca_box.add_classification(oca_bundle.capture_base.classification);

        let mut attributes: HashMap<String, Attribute> = HashMap::new();
        for (attr_name, attr_type) in oca_bundle.capture_base.attributes {
            let attr = Attribute {
                name: attr_name.clone(),
                attribute_type: Some(attr_type),
                // TODO find out how to make sure that said or Array said would be in a attr type
               // reference_sai: ref_said,
                ..Default::default()
            };
            attributes.insert(attr_name.clone(), attr);
        }
        for attr_name in oca_bundle.capture_base.flagged_attributes {
            attributes.get_mut(&attr_name).unwrap().set_flagged();
        }

        let meta_overlays = oca_bundle
            .overlays
            .iter()
            .filter_map(|x| x.as_any().downcast_ref::<overlay::Meta>())
            .collect::<Vec<_>>();
        for overlay in meta_overlays {
            for (meta_name, meta_value) in overlay.attr_pairs.iter() {
                oca_box.add_meta(*overlay.language().unwrap(), meta_name.clone(), meta_value.clone());
            }
        }

        let character_encoding_overlays = oca_bundle
            .overlays
            .iter()
            .filter_map(|x| x.as_any().downcast_ref::<overlay::CharacterEncoding>())
            .collect::<Vec<_>>();
        for overlay in character_encoding_overlays {
            for (attr_name, encoding) in overlay.attribute_character_encoding.iter() {
                attributes
                    .get_mut(attr_name)
                    .unwrap()
                    .set_encoding(*encoding);
            }
        }

        let conformance_overlays = oca_bundle
            .overlays
            .iter()
            .filter_map(|x| x.as_any().downcast_ref::<overlay::Conformance>())
            .collect::<Vec<_>>();
        for overlay in conformance_overlays {
            for (attr_name, conformance) in overlay.attribute_conformance.iter() {
                attributes
                    .get_mut(attr_name)
                    .unwrap()
                    .set_conformance(conformance.clone());
            }
        }

        let conditional_overlays = oca_bundle
            .overlays
            .iter()
            .filter_map(|x| x.as_any().downcast_ref::<overlay::Conditional>())
            .collect::<Vec<_>>();
        for overlay in conditional_overlays {
            let re = regex::Regex::new(r"\$\{(\d+)\}").unwrap();

            for (attr_name, condition) in overlay.attribute_conditions.iter() {
                let condition_dependencies = overlay
                    .attribute_dependencies
                    .get(attr_name)
                    .unwrap(); // todo
                let cond = re.replace_all(condition, |caps: &regex::Captures| {
                    let dep = condition_dependencies[
                        caps[1].parse::<usize>().unwrap()
                    ].clone();
                    format!("${{{}}}", dep)
                }).to_string();

                attributes
                    .get_mut(attr_name)
                    .unwrap()
                    .set_condition(cond.clone());
            }
        }

        let cardinality_overlays = oca_bundle
            .overlays
            .iter()
            .filter_map(|x| x.as_any().downcast_ref::<overlay::Cardinality>())
            .collect::<Vec<_>>();
        for overlay in cardinality_overlays {
            for (attr_name, cardinality) in overlay.attribute_cardinality.iter() {
                attributes
                    .get_mut(attr_name)
                    .unwrap()
                    .set_cardinality(cardinality.clone());
            }
        }

        #[cfg(feature = "format_overlay")]
        {
            let format_overlays = oca_bundle
                .overlays
                .iter()
                .filter_map(|x| x.as_any().downcast_ref::<overlay::Format>())
                .collect::<Vec<_>>();
            for overlay in format_overlays {
                for (attr_name, format) in overlay.attribute_formats.iter() {
                    attributes
                        .get_mut(attr_name)
                        .unwrap()
                        .set_format(format.clone());
                }
            }
        }

        let unit_overlays = oca_bundle
            .overlays
            .iter()
            .filter_map(|x| x.as_any().downcast_ref::<overlay::Unit>())
            .collect::<Vec<_>>();
        for overlay in unit_overlays {
            for (attr_name, unit) in overlay.attribute_units.iter() {
                attributes.get_mut(attr_name).unwrap().set_unit(AttributeUnit {
                    measurement_system: overlay.measurement_system().unwrap().clone(),
                    unit: unit.clone(),
                });
            }
        }

        let entry_code_overlays = oca_bundle
            .overlays
            .iter()
            .filter_map(|x| x.as_any().downcast_ref::<overlay::EntryCode>())
            .collect::<Vec<_>>();
        for overlay in entry_code_overlays {
            for (attr_name, entry_code) in overlay.attribute_entry_codes.iter() {
                attributes
                    .get_mut(attr_name)
                    .unwrap()
                    .set_entry_codes(entry_code.clone());
            }
        }

        let entry_overlays = oca_bundle
            .overlays
            .iter()
            .filter_map(|x| x.as_any().downcast_ref::<overlay::Entry>())
            .collect::<Vec<_>>();
        for overlay in entry_overlays {
            for (attr_name, entries) in overlay.attribute_entries.iter() {
                attributes
                    .get_mut(attr_name)
                    .unwrap()
                    .set_entry(*overlay.language().unwrap(), entries.clone());
            }
        }

        let label_overlays = oca_bundle
            .overlays
            .iter()
            .filter_map(|x| x.as_any().downcast_ref::<overlay::Label>())
            .collect::<Vec<_>>();
        for overlay in label_overlays {
            for (attr_name, label) in overlay.attribute_labels.iter() {
                attributes
                    .get_mut(attr_name)
                    .unwrap()
                    .set_label(*overlay.language().unwrap(), label.clone());
            }
        }

        let information_overlays = oca_bundle
            .overlays
            .iter()
            .filter_map(|x| x.as_any().downcast_ref::<overlay::Information>())
            .collect::<Vec<_>>();
        for overlay in information_overlays {
            for (attr_name, information) in overlay.attribute_information.iter() {
                attributes
                    .get_mut(attr_name)
                    .unwrap()
                    .set_information(*overlay.language().unwrap(), information.clone());
            }
        }

        for (_, attribute) in attributes {
            oca_box.add_attribute(attribute);
        }

        oca_box
    }
}

impl OCABundle {
    pub fn fill_said(&mut self) {
        self.compute_digest();
    }

    pub fn to_ast(&self) -> OCAAst {
        let mut ast = OCAAst::new();

        let mut properties = None;
        if !self.capture_base.classification.is_empty() {
            properties = Some(IndexMap::new());
            properties.as_mut().unwrap().insert(
                "classification".to_string(),
                NestedValue::Value(self.capture_base.classification.clone())
            );
        }

        let mut attributes = IndexMap::new();
        self.capture_base.attributes.iter().for_each(|(attr_name, attr_type)| {
            attributes.insert(
                attr_name.clone(),
                attr_type.clone(),
            );
        });

        let command = Command {
            kind: CommandType::Add,
            object_kind: ObjectKind::CaptureBase( CaptureContent {
                // TODO find out if we can use indexmap in capture base to simplify stuff
                attributes: Some(self.capture_base.attributes.clone().into_iter().collect()),
                properties,
            }),
        };
        ast.commands.push(command);

        self.overlays.iter().for_each(|overlay| {
            match overlay.overlay_type() {
                OverlayType::CharacterEncoding => {
                    let character_encoding = overlay.as_any().downcast_ref::<overlay::CharacterEncoding>().unwrap();
                    let mut attributes = IndexMap::new();
                    for (attr_name, encoding) in character_encoding.attribute_character_encoding.iter() {
                        let encoding_val = serde_json::to_value(encoding).unwrap();
                        attributes.insert(attr_name.clone(), NestedValue::Value(encoding_val.as_str().unwrap().to_string()));
                    }
                    let command = Command {
                        kind: CommandType::Add,
                        object_kind: ObjectKind::Overlay(OverlayType::CharacterEncoding, Content {
                            attributes: Some(attributes),
                            properties: None,
                        }),
                    };
                    ast.commands.push(command);
                },
                #[cfg(feature = "format_overlay")]
                OverlayType::Format => {
                    let format = overlay.as_any().downcast_ref::<overlay::Format>().unwrap();
                    let mut attributes = IndexMap::new();
                    for (attr_name, format) in format.attribute_formats.iter() {
                        attributes.insert(attr_name.clone(), NestedValue::Value(format.clone()));
                    }
                    let command = Command {
                        kind: CommandType::Add,
                        object_kind: ObjectKind::Overlay(OverlayType::Format, Content {
                            attributes: Some(attributes),
                            properties: None,
                        }),
                    };
                    ast.commands.push(command);
                },
                OverlayType::Meta => {
                    let meta = overlay.as_any().downcast_ref::<overlay::Meta>().unwrap();
                    let mut properties = IndexMap::new();
                    properties.insert(
                        "lang".to_string(),
                        NestedValue::Value(
                            meta
                                .language()
                                .unwrap()
                                .to_639_1()
                                .unwrap()
                                .to_string()
                        )
                    );
                    for (meta_name, meta_value) in meta.attr_pairs.iter() {
                        properties.insert(meta_name.clone(), NestedValue::Value(meta_value.clone()));
                    }
                    let command = Command {
                        kind: CommandType::Add,
                        object_kind: ObjectKind::Overlay(OverlayType::Meta, Content {
                            attributes: None,
                            properties: Some(properties),
                        }),
                    };
                    ast.commands.push(command);
                },
                OverlayType::Label => {
                    let label = overlay.as_any().downcast_ref::<overlay::Label>().unwrap();
                    let mut properties = IndexMap::new();
                    properties.insert(
                        "lang".to_string(),
                        NestedValue::Value(
                            label
                                .language()
                                .unwrap()
                                .to_639_1()
                                .unwrap()
                                .to_string()
                        )
                    );
                    let mut attributes = IndexMap::new();
                    for (attr_name, label) in label.attribute_labels.iter() {
                        attributes.insert(attr_name.clone(), NestedValue::Value(label.clone()));
                    }
                    let command = Command {
                        kind: CommandType::Add,
                        object_kind: ObjectKind::Overlay(OverlayType::Label, Content {
                            attributes: Some(attributes),
                            properties: Some(properties),
                        }),
                    };
                    ast.commands.push(command);
                },
                OverlayType::Information => {
                    let information = overlay.as_any().downcast_ref::<overlay::Information>().unwrap();
                    let mut properties = IndexMap::new();
                    properties.insert(
                        "lang".to_string(),
                        NestedValue::Value(
                            information
                                .language()
                                .unwrap()
                                .to_639_1()
                                .unwrap()
                                .to_string()
                        )
                    );
                    let mut attributes = IndexMap::new();
                    for (attr_name, information) in information.attribute_information.iter() {
                        attributes.insert(attr_name.clone(), NestedValue::Value(information.clone()));
                    }
                    let command = Command {
                        kind: CommandType::Add,
                        object_kind: ObjectKind::Overlay(OverlayType::Information, Content {
                            attributes: Some(attributes),
                            properties: Some(properties),
                        }),
                    };
                    ast.commands.push(command);
                },
                OverlayType::Conditional => {
                    let conditional = overlay.as_any().downcast_ref::<overlay::Conditional>().unwrap();
                    let mut attributes = IndexMap::new();
                    let re = regex::Regex::new(r"\$\{(\d+)\}").unwrap();
                    for (attr_name, condition) in conditional.attribute_conditions.iter() {

                        let condition_dependencies = conditional
                            .attribute_dependencies
                            .get(attr_name)
                            .unwrap(); // todo
                        let cond = re.replace_all(condition, |caps: &regex::Captures| {
                            let dep = condition_dependencies[
                                caps[1].parse::<usize>().unwrap()
                            ].clone();
                            format!("${{{}}}", dep)
                        }).to_string();

                        attributes.insert(attr_name.clone(), NestedValue::Value(cond.clone()));
                    }
                    let command = Command {
                        kind: CommandType::Add,
                        object_kind: ObjectKind::Overlay(OverlayType::Conditional, Content  {
                            attributes: Some(attributes),
                            properties: None,
                        }),
                    };
                    ast.commands.push(command);
                },
                OverlayType::Conformance => {
                    let conformance = overlay.as_any().downcast_ref::<overlay::Conformance>().unwrap();
                    let mut attributes = IndexMap::new();
                    for (attr_name, conformance) in conformance.attribute_conformance.iter() {
                        attributes.insert(attr_name.clone(), NestedValue::Value(conformance.clone()));
                    }
                    let command = Command {
                        kind: CommandType::Add,
                        object_kind: ObjectKind::Overlay(OverlayType::Conformance, Content {
                            attributes: Some(attributes),
                            properties: None,
                        }),
                    };
                    ast.commands.push(command);
                },
                OverlayType::EntryCode => {
                    let entry_code = overlay.as_any().downcast_ref::<overlay::EntryCode>().unwrap();
                    let mut attributes = IndexMap::new();
                    for (attr_name, entry_code) in entry_code.attribute_entry_codes.iter() {
                        match entry_code {
                            crate::state::entry_codes::EntryCodes::Sai(said) => {
                                attributes.insert(attr_name.clone(), NestedValue::Value(said.to_string()));
                            },
                            crate::state::entry_codes::EntryCodes::Array(entry_codes) => {
                                attributes.insert(
                                    attr_name.clone(),
                                    NestedValue::Array(
                                        entry_codes
                                            .iter()
                                            .map(|code|
                                                NestedValue::Value(
                                                    code.clone()
                                                )
                                            ).collect()
                                    )
                                );
                            }
                        }
                    }
                    let command = Command {
                        kind: CommandType::Add,
                        object_kind: ObjectKind::Overlay(OverlayType::EntryCode, Content {
                            attributes: Some(attributes),
                            properties: None,
                        }),
                    };
                    ast.commands.push(command);
                },
                OverlayType::Entry => {
                    let entry = overlay.as_any().downcast_ref::<overlay::Entry>().unwrap();
                    let mut properties = IndexMap::new();
                    properties.insert(
                        "lang".to_string(),
                        NestedValue::Value(
                            entry
                                .language()
                                .unwrap()
                                .to_639_1()
                                .unwrap()
                                .to_string()
                        )
                    );
                    let mut attributes = IndexMap::new();
                    for (attr_name, entries) in entry.attribute_entries.iter() {
                        match entries {
                            crate::state::entries::EntriesElement::Sai(said) => {
                                attributes.insert(attr_name.clone(), NestedValue::Value(said.to_string()));
                            },
                            crate::state::entries::EntriesElement::Object(entries) => {
                                attributes.insert(
                                    attr_name.clone(),
                                    NestedValue::Object(
                                        entries
                                            .iter()
                                            .map(|(k, v)|
                                                (
                                                    k.clone(),
                                                    NestedValue::Value(
                                                        v.clone()
                                                    )
                                                )
                                            ).collect()
                                    )
                                );
                            }
                        }
                    }
                    let command = Command {
                        kind: CommandType::Add,
                        object_kind: ObjectKind::Overlay(OverlayType::Entry, Content {
                            attributes: Some(attributes),
                            properties: Some(properties),
                        }),
                    };
                    ast.commands.push(command);
                },
                OverlayType::Cardinality => {
                    let cardinality = overlay.as_any().downcast_ref::<overlay::Cardinality>().unwrap();
                    let mut attributes = IndexMap::new();
                    for (attr_name, cardinality) in cardinality.attribute_cardinality.iter() {
                        attributes.insert(attr_name.clone(), NestedValue::Value(cardinality.clone()));
                    }
                    let command = Command {
                        kind: CommandType::Add,
                        object_kind: ObjectKind::Overlay(OverlayType::Cardinality, Content {
                            attributes: Some(attributes),
                            properties: None,
                        }),
                    };
                    ast.commands.push(command);
                },
                OverlayType::Unit => {
                    let unit_ov = overlay.as_any().downcast_ref::<overlay::Unit>().unwrap();
                    let mut properties = IndexMap::new();
                    let unit_system_val = serde_json::to_value(unit_ov.measurement_system().unwrap()).unwrap();
                    properties.insert(
                        "unit_system".to_string(),
                        NestedValue::Value(
                            unit_system_val.as_str().unwrap().to_string()
                        )
                    );
                    let mut attributes = IndexMap::new();
                    for (attr_name, unit) in unit_ov.attribute_units.iter() {
                        let unit_val = serde_json::to_value(unit).unwrap();
                        attributes.insert(
                            attr_name.clone(),
                            NestedValue::Value(
                                unit_val.as_str().unwrap().to_string()
                            )
                        );
                    }
                    let command = Command {
                        kind: CommandType::Add,
                        object_kind: ObjectKind::Overlay(OverlayType::Unit, Content {
                            attributes: Some(attributes),
                            properties: Some(properties),
                        }),
                    };
                    ast.commands.push(command);
                },
                _ => {}
            }
        });

        ast
    }
}

/*
#[derive(Clone)]
struct AttributeLayoutValues {
    pub reference_sai: Option<String>,
    pub has_unit: bool,
}

impl AttributeLayoutValues {
    pub fn new() -> Self {
        Self {
            reference_sai: None,
            has_unit: false,
        }
    }

    pub fn add_reference_sai(&mut self, reference_sai: String) {
        self.reference_sai = Some(reference_sai);
    }

    pub fn add_unit(&mut self) {
        self.has_unit = true;
    }
}
*/

#[cfg(test)]
mod tests {
    use maplit::hashmap;
    use oca_ast::ast::{NestedAttrType, RefValue};
    use said::SelfAddressingIdentifier;

    use crate::state::{attribute::AttributeType, entries::EntriesElement};
    use super::*;

    #[test]
    fn build_oca_bundle() {
        let mut oca = OCABox::new();
        oca.add_classification("test".to_string());
        oca.add_meta(Language::Eng, "name".to_string(), "test name".to_string());
        oca.add_meta(Language::Eng, "description".to_string(), "test desc".to_string());
        let mut attr = Attribute::new("first_name".to_string());
        attr.set_attribute_type(NestedAttrType::Value(AttributeType::Text));
        oca.add_attribute(attr);

        let mut attr2 = Attribute::new("gender".to_string());
        let entries = EntriesElement::Object(hashmap! {});
        attr2.set_entry(Language::Eng, entries);

        let mut attr = Attribute::new("last_name".to_string());
        attr.set_attribute_type(NestedAttrType::Value(AttributeType::Text));
        oca.add_attribute(attr);
        // oca.add_attribute(Attribute::new("last_name".to_string()));
        let oca_bundle = oca.generate_bundle();
        /* let oca_bundle_encoded = oca_bundle.encode().unwrap();
        let oca_bundle_json = String::from_utf8(oca_bundle_encoded).unwrap();
        println!("{}", oca_bundle_json); */
        let said = oca_bundle.said;
        let oca_bundle = oca.generate_bundle();
        let said2 = oca_bundle.said;
        // let oca_bundle_json = serde_json::to_string_pretty(&oca_bundle).unwrap();
        assert_eq!(said, said2);
    }

    #[test]
    fn load_oca_box_from_oca_bundle() {
        let mut oca = OCABox::new();
        oca.add_meta(Language::Eng, "name".to_string(), "test name".to_string());
        oca.add_meta(Language::Eng, "description".to_string(), "test desc".to_string());
        let mut attr = Attribute::new("first_name".to_string());
        attr.set_attribute_type(NestedAttrType::Value(AttributeType::Text));
        oca.add_attribute(attr);

        let mut attr = Attribute::new("last_name".to_string());
        attr.set_attribute_type(NestedAttrType::Value(AttributeType::Text));
        attr.set_condition("string.len(${first_name}) > 0".to_string());
        oca.add_attribute(attr);

        let mut attr = Attribute::new("ref".to_string());
        let said = SelfAddressingIdentifier::default();
        attr.set_attribute_type(NestedAttrType::Reference(RefValue::Said(said)));
        oca.add_attribute(attr);

        let oca_bundle = oca.generate_bundle();
        let said = oca_bundle.said.clone();

        let mut oca_box = OCABox::from(oca_bundle);
        let oca_bundle = oca_box.generate_bundle();
        let said2 = oca_bundle.said;

        assert_eq!(said, said2);
    }
}

/* struct CatAttributes {
    category_labels: HashMap<String, String>,
    categorized: IndexMap<String, IndexMap<String, AttributeLayoutValues>>,
    uncategorized: IndexMap<String, AttributeLayoutValues>,
    lang: String,
}

impl CatAttributes {
    fn add_to_category(&mut self, categories: Vec<&str>, attribute: &Attribute) {
        let mut attribute_layout_values = AttributeLayoutValues::new();
        if let Some(sai) = &attribute.reference_sai {
            attribute_layout_values.add_reference_sai(sai.clone());
        }
        // if attribute.unit.is_some() {
        //     attribute_layout_values.add_unit();
        // }
        if categories.is_empty() {
            self.uncategorized
                .insert(attribute.name.clone(), attribute_layout_values);
            return;
        }
        let mut supercats: Vec<i32> = vec![];
        for (i, category) in categories.iter().enumerate() {
            let supercats_str: Vec<String> = supercats.iter().map(|c| c.to_string()).collect();
            let mut supercat = String::new();
            if !supercats_str.is_empty() {
                supercat = format!("-{}", supercats_str.join("-"))
            }
            let regex = regex::Regex::new(format!("^_cat{supercat}(-[0-9]*)_$").as_str()).unwrap();
            let mut acctual_cat_id = String::new();
            let mut category_exists = false;
            for (cat_id, cat_label) in self.category_labels.iter() {
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
                self.category_labels
                    .insert(acctual_cat_id.clone(), category.to_string());
                self.categorized
                    .insert(acctual_cat_id.clone(), IndexMap::new());
            }

            if i + 1 == categories.len() {
                self.categorized
                    .get_mut(acctual_cat_id.as_str())
                    .unwrap()
                    .insert(attribute.name.clone(), attribute_layout_values.clone());
            }
        }
    }
}
 */

/*
#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{
        attribute::{AttributeType, Entries, Entry},
        encoding::Encoding,
        entry_codes::EntryCodes,
    };
    use maplit::hashmap;

    #[test]
    fn build_oca_without_attributes() {
        let oca = OCABuilder::new(Encoding::Utf8)
            .add_classification("GICS:35102020".to_string())
            /*             .add_name(hashmap! {
                "En".to_string() => "Driving Licence".to_string(),
                "Pl".to_string() => "Prawo Jazdy".to_string(),
            })
            .add_description(hashmap! {
                "En".to_string() => "Driving Licence".to_string(),
                "Pl".to_string() => "Prawo Jazdy".to_string(),
            }) */
            .finalize();

        // println!("{:#?}", serde_json::to_string(&oca).unwrap());

        assert_eq!(oca.capture_base.attributes.len(), 0);
        assert_eq!(oca.capture_base.classification, "GICS:35102020");
    }
    /*
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

            let attr2 = AttributeBuilder::new(String::from("n2"), AttributeType::DateTime)
                .add_label(hashmap! {
                    "En".to_string() => "Date: ".to_string(),
                    "Pl".to_string() => "Data: ".to_string(),
                })
                .add_condition("${0} == 'op1'".to_string(), vec!["n1".to_string()])
                .add_encoding(Encoding::Iso8859_1)
                .add_format("DD/MM/YYYY".to_string())
                .build();

            let attr3 = AttributeBuilder::new(String::from("n3"), AttributeType::Reference)
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
                Some(&"Reference:sai".to_string())
            );
            assert_eq!(oca.capture_base.attributes.len(), 3);
            assert_eq!(oca.capture_base.flagged_attributes.len(), 1);
        } */
}
*/
