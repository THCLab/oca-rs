// use said::{sad::SAD, version::SerializationInfo};
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

pub struct OCABox {
    pub attributes: HashMap<String, Attribute>,
    pub credential_layouts: Option<Vec<CredentialLayout>>,
    pub form_layouts: Option<Vec<FormLayout>>,
    pub mappings: Option<Vec<overlay::AttributeMapping>>,
    pub meta: Option<HashMap<Language, HashMap<String, String>>>,
    pub classification: Option<String>,
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
            return;
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
                    .find(|x| x.overlay_type().contains("/character_encoding/"));
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
                    .find(|x| x.overlay_type().contains("/format/"));
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
                    .find(|x| x.overlay_type().contains("/conformance/"));
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
                    .find(|x| x.overlay_type().contains("/cardinality/"));
                if cardinality_ov.is_none() {
                    overlays.push(Box::new(overlay::Cardinality::new()));
                    cardinality_ov = overlays.last_mut();
                }
                if let Some(ov) = cardinality_ov {
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
                    .find(|x| x.overlay_type().contains("/entry_code/"));
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
                        .find(|x| x.overlay_type().contains("/entry/") && x.language() == Some(lang));
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
                        .find(|x| x.overlay_type().contains("/label/") && x.language() == Some(lang));
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
                        .find(|x| x.overlay_type().contains("/information/") && x.language() == Some(lang));
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
            if let Some(serde_value::Value::String(overlay_type)) =
                overlay.get(&serde_value::Value::String("type".to_string()))
            {
                if overlay_type.contains("/mapping/") {
                    return Ok(Box::new(
                        de_overlay
                            .deserialize_into::<overlay::AttributeMapping>()
                            .map_err(|e| serde::de::Error::custom(format!("Meta overlay: {e}")))?,
                    ));
                }
                if overlay_type.contains("/character_encoding/") {
                    return Ok(Box::new(
                        de_overlay
                            .deserialize_into::<overlay::CharacterEncoding>()
                            .map_err(|e| {
                                serde::de::Error::custom(format!("Character Encoding overlay: {e}"))
                            })?,
                    ));
                }
                if overlay_type.contains("/cardinality/") {
                    return Ok(Box::new(
                        de_overlay
                            .deserialize_into::<overlay::Cardinality>()
                            .map_err(|e| {
                                serde::de::Error::custom(format!("Cardinality overlay: {e}"))
                            })?,
                    ));
                }
                if overlay_type.contains("/conformance/") {
                    return Ok(Box::new(
                        de_overlay
                            .deserialize_into::<overlay::Conformance>()
                            .map_err(|e| {
                                serde::de::Error::custom(format!("Conformance overlay: {e}"))
                            })?,
                    ));
                }
                if overlay_type.contains("/conditional/") {
                    return Ok(Box::new(
                        de_overlay
                            .deserialize_into::<overlay::Conditional>()
                            .map_err(|e| {
                                serde::de::Error::custom(format!("Conditional overlay: {e}"))
                            })?,
                    ));
                }
                if overlay_type.contains("/entry/") {
                    return Ok(Box::new(
                        de_overlay
                            .deserialize_into::<overlay::Entry>()
                            .map_err(|e| serde::de::Error::custom(format!("Entry overlay: {e}")))?,
                    ));
                }
                if overlay_type.contains("/entry_code/") {
                    return Ok(Box::new(
                        de_overlay
                            .deserialize_into::<overlay::EntryCode>()
                            .map_err(|e| {
                                serde::de::Error::custom(format!("Entry Code overlay: {e}"))
                            })?,
                    ));
                }
                if overlay_type.contains("/entry_code_mapping/") {
                    return Ok(Box::new(
                        de_overlay
                            .deserialize_into::<overlay::EntryCodeMapping>()
                            .map_err(|e| {
                                serde::de::Error::custom(format!("Entry Code Mapping overlay: {e}"))
                            })?,
                    ));
                }

                #[cfg(feature = "format_overlay")]
                if overlay_type.contains("/format/") {
                    return Ok(Box::new(
                        de_overlay
                            .deserialize_into::<overlay::Format>()
                            .map_err(|e| {
                                serde::de::Error::custom(format!("Format overlay: {e}"))
                            })?,
                    ));
                }

                if overlay_type.contains("/information/") {
                    return Ok(Box::new(
                        de_overlay
                            .deserialize_into::<overlay::Information>()
                            .map_err(|e| {
                                serde::de::Error::custom(format!("Information overlay: {e}"))
                            })?,
                    ));
                }
                if overlay_type.contains("/label/") {
                    return Ok(Box::new(
                        de_overlay
                            .deserialize_into::<overlay::Label>()
                            .map_err(|e| serde::de::Error::custom(format!("Label overlay: {e}")))?,
                    ));
                }
                if overlay_type.contains("/unit/") {
                    return Ok(Box::new(
                        de_overlay
                            .deserialize_into::<overlay::Unit>()
                            .map_err(|e| serde::de::Error::custom(format!("Unit overlay: {e}")))?,
                    ));
                }
                if overlay_type.contains("/meta/") {
                    return Ok(Box::new(
                        de_overlay
                            .deserialize_into::<overlay::Meta>()
                            .map_err(|e| serde::de::Error::custom(format!("Meta overlay: {e}")))?,
                    ));
                }
                if overlay_type.contains("/form_layout/") {
                    return Ok(Box::new(
                        de_overlay
                            .deserialize_into::<overlay::FormLayout>()
                            .map_err(|e| {
                                serde::de::Error::custom(format!("Form Layout overlay: {e}"))
                            })?,
                    ));
                }
                if overlay_type.contains("/credential_layout/") {
                    return Ok(Box::new(
                        de_overlay
                            .deserialize_into::<overlay::CredentialLayout>()
                            .map_err(|e| {
                                serde::de::Error::custom(format!("Credential Layout overlay: {e}"))
                            })?,
                    ));
                }
                if overlay_type.contains("/subset/") {
                    return Ok(Box::new(
                        de_overlay
                            .deserialize_into::<overlay::Subset>()
                            .map_err(|e| {
                                serde::de::Error::custom(format!("Subset overlay: {e}"))
                            })?,
                    ));
                }
                if overlay_type.contains("/standard/") {
                    return Ok(Box::new(
                        de_overlay
                            .deserialize_into::<overlay::Standard>()
                            .map_err(|e| {
                                serde::de::Error::custom(format!("Standard overlay: {e}"))
                            })?,
                    ));
                }

                return Err(serde::de::Error::custom(format!(
                    "unknown overlay type: {overlay_type}"
                )));
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
        "character_encoding",
        "format",
        "meta",
        "label",
        "information",
        "standard",
        "conditional",
        "conformance",
        "entry_code",
        "entry",
        "cardinality",
        "unit",
        "attribute_mapping",
        "entry_code_mapping",
        "unit_mapping",
        "subset",
        "credential_layout",
        "form_layout"
    ];
    for o_type in overlays_order {
        for overlay in overlays {
            let o_type_tmp = format!("/{o_type}/");
            if overlay.overlay_type().contains(&o_type_tmp) {
                match overlay.language() {
                    Some(_) => {
                        if let Some(OverlayValue::Array(ov)) = overlays_map.get_mut(&Value::String(o_type.to_string())) {
                            ov.push(overlay.clone());
                        } else {
                            overlays_map.insert(
                                Value::String(o_type.to_string()),
                                OverlayValue::Array(vec![
                                    overlay.clone()
                                ])
                            );
                        }
                    },
                    None => {
                        overlays_map.insert(
                            Value::String(o_type.to_string()),
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


impl OCABundle {
    pub fn fill_said(&mut self) {
        self.compute_digest();
    }
}
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

#[cfg(test)]
mod tests {
    use crate::state::attribute::AttributeType;

    use super::*;
    use crate::state::oca::overlay::meta::Metas;

    #[test]
    fn build_oca_bundle() {
        let mut oca = OCABox::new();
        oca.add_classification("test".to_string());
        oca.add_meta(Language::Eng, "name".to_string(), "test name".to_string());
        oca.add_meta(Language::Eng, "description".to_string(), "test desc".to_string());
        let mut attr = Attribute::new("first_name".to_string());
        attr.set_attribute_type(AttributeType::Text);
        oca.add_attribute(attr);

        let mut attr = Attribute::new("last_name".to_string());
        attr.set_attribute_type(AttributeType::Text);
        oca.add_attribute(attr);
        // oca.add_attribute(Attribute::new("last_name".to_string()));
        let oca_bundle = oca.generate_bundle();
        let oca_bundle_encoded = oca_bundle.encode().unwrap();
        let oca_bundle_json = String::from_utf8(oca_bundle_encoded).unwrap();
        println!("{}", oca_bundle_json);
        let said = oca_bundle.said.clone();
        let oca_bundle = oca.generate_bundle();
        let said2 = oca_bundle.said.clone();
        let oca_bundle_json = serde_json::to_string_pretty(&oca_bundle).unwrap();
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
