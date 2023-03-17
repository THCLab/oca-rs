pub(crate) use std::collections::HashMap;
pub(crate) use std::sync::Mutex;

use anyhow::{Context, Result};
use flutter_rust_bridge::{frb, RustOpaque};
pub(crate) use oca_rs::state::{
    attribute::{Attribute as OcaAttrRaw, AttributeType as OcaAttrType},
    encoding::Encoding as OcaEncoding,
    entries::EntriesElement,
    entry_codes::EntryCodes,
    oca::{
        capture_base::CaptureBase as OcaCaptureBaseRaw,
        overlay::{
            self,
            unit::{ImperialUnit as OcaImperialUnit, MetricUnit as OcaMetricUnit},
        },
        DynOverlay, OCABox as OcaBoxRaw, OCABundle as OcaBundleRaw,
    },
};

pub struct OcaBox(pub RustOpaque<Mutex<OcaBoxRaw>>);

impl OcaBox {
    pub fn new() -> OcaBox {
        OcaBox(RustOpaque::new(Mutex::new(OcaBoxRaw::new())))
    }

    pub fn add_meta(&self, lang: String, name: String, value: String) -> Result<()> {
        let mut oca_box = self.0.lock().unwrap();
        let lang = lang.parse().context("Invalid language")?;
        overlay::meta::Metas::add_meta(&mut *oca_box, lang, name, value);
        Ok(())
    }

    pub fn add_attribute(&self, attr: OcaAttr) {
        let mut oca_box = self.0.lock().unwrap();
        oca_box.add_attribute(attr.0.lock().unwrap().clone());
    }

    pub fn generate_bundle(&self) -> OcaBundle {
        let mut oca_box = self.0.lock().unwrap();
        let oca_bundle = oca_box.generate_bundle();
        OcaBundle(RustOpaque::new(Mutex::new(oca_bundle)))
    }

    pub fn add_form_layout(&self, layout: String) {
        let mut oca_box = self.0.lock().unwrap();
        overlay::form_layout::FormLayouts::add_form_layout(&mut *oca_box, layout);
    }

    pub fn add_credential_layout(&self, layout: String) {
        let mut oca_box = self.0.lock().unwrap();
        overlay::credential_layout::CredentialLayouts::add_credential_layout(&mut *oca_box, layout);
    }
}

pub struct OcaAttr(pub RustOpaque<Mutex<OcaAttrRaw>>);

impl OcaAttr {
    pub fn new(name: String) -> OcaAttr {
        let attr = OcaAttrRaw::new(name);
        OcaAttr(RustOpaque::new(Mutex::new(attr)))
    }

    pub fn set_attribute_type(&self, attr_type: OcaAttrType) {
        let mut attr = self.0.lock().unwrap();
        attr.set_attribute_type(attr_type);
    }

    pub fn set_flagged(&self) {
        let mut attr = self.0.lock().unwrap();
        attr.set_flagged();
    }

    pub fn set_encoding(&self, encoding: OcaEncoding) {
        let mut attr = self.0.lock().unwrap();
        overlay::character_encoding::CharacterEncodings::set_encoding(&mut *attr, encoding);
    }

    pub fn set_cardinality(&self, cardinality: String) {
        let mut attr = self.0.lock().unwrap();
        overlay::cardinality::Cardinalitys::set_cardinality(&mut *attr, cardinality);
    }

    pub fn set_conformance(&self, conformance: String) {
        let mut attr = self.0.lock().unwrap();
        overlay::conformance::Conformances::set_conformance(&mut *attr, conformance);
    }

    pub fn set_label(&self, lang: String, label: String) -> Result<()> {
        let mut attr = self.0.lock().unwrap();
        let lang = lang.parse().context("Invalid language")?;
        overlay::label::Labels::set_label(&mut *attr, lang, label);
        Ok(())
    }

    pub fn set_information(&self, lang: String, information: String) -> Result<()> {
        let mut attr = self.0.lock().unwrap();
        let lang = lang.parse().context("Invalid language")?;
        overlay::information::Information::set_information(&mut *attr, lang, information);
        Ok(())
    }

    pub fn set_entry_codes(&self, entry_codes: Vec<String>) {
        let mut attr = self.0.lock().unwrap();
        overlay::entry_code::EntryCodes::set_entry_codes(
            &mut *attr,
            EntryCodes::Array(entry_codes),
        );
    }

    pub fn set_entry_codes_sai(&self, sai: String) {
        let mut attr = self.0.lock().unwrap();
        overlay::entry_code::EntryCodes::set_entry_codes(&mut *attr, EntryCodes::Sai(sai));
    }

    pub fn set_entry(&self, lang: String, entries: OcaMap) -> Result<()> {
        let mut attr = self.0.lock().unwrap();
        let lang = lang.parse().context("Invalid language")?;
        let entries = entries.0.lock().unwrap().0.clone();
        overlay::entry::Entries::set_entry(&mut *attr, lang, EntriesElement::Object(entries));
        Ok(())
    }

    pub fn set_unit_metric(&self, unit: OcaMetricUnit) {
        let mut attr = self.0.lock().unwrap();
        overlay::unit::Unit::set_unit(
            &mut *attr,
            overlay::unit::AttributeUnit {
                measurement_system: overlay::unit::MeasurementSystem::Metric,
                unit: overlay::unit::MeasurementUnit::Metric(unit),
            },
        );
    }

    pub fn set_unit_imperial(&self, unit: OcaImperialUnit) {
        let mut attr = self.0.lock().unwrap();
        overlay::unit::Unit::set_unit(
            &mut *attr,
            overlay::unit::AttributeUnit {
                measurement_system: overlay::unit::MeasurementSystem::Imperial,
                unit: overlay::unit::MeasurementUnit::Imperial(unit),
            },
        );
    }

    pub fn set_format(&self, format: String) {
        let mut attr = self.0.lock().unwrap();
        overlay::format::Formats::set_format(&mut *attr, format);
    }
}

#[frb(mirror(OcaAttrType))]
pub enum _OcaAttrType {
    Boolean,
    ArrayBoolean,
    Binary,
    ArrayBinary,
    Text,
    ArrayText,
    Numeric,
    ArrayNumeric,
    DateTime,
    ArrayDateTime,
    Reference,
    ArrayReference,
}

#[frb(mirror(OcaEncoding))]
pub enum _OcaEncoding {
    Base64,
    Utf8,
    Iso8859_1,
}

#[frb(mirror(OcaMetricUnit))]
pub enum _OcaMetricUnit {
    Kilogram,
    Gram,
    Milligram,
    Liter,
    Milliliter,
    Centimeter,
    Millimeter,
    Inch,
    Foot,
    Yard,
    Mile,
    Celsius,
    Fahrenheit,
    Kelvin,
    Percent,
    Count,
    Other,
}

#[frb(mirror(OcaImperialUnit))]
pub enum _OcaImperialUnit {
    Pound,
    Ounce,
    Gallon,
    Quart,
    Pint,
    FluidOunce,
    Inch,
    Foot,
    Yard,
    Mile,
    Celsius,
    Fahrenheit,
    Kelvin,
    Percent,
    Count,
    Other,
}

pub struct OcaBundle(pub RustOpaque<Mutex<OcaBundleRaw>>);

impl OcaBundle {
    pub fn to_json(&self) -> String {
        let oca_bundle = self.0.lock().unwrap();
        serde_json::to_string_pretty(&*oca_bundle).unwrap()
    }

    pub fn said(&self) -> String {
        let oca_bundle = self.0.lock().unwrap();
        oca_bundle.said.clone()
    }

    pub fn capture_base(&self) -> OcaCaptureBase {
        let oca_bundle = self.0.lock().unwrap();
        OcaCaptureBase(RustOpaque::new(Mutex::new(oca_bundle.capture_base.clone())))
    }

    pub fn overlays(&self) -> Vec<OcaOverlay> {
        let oca_bundle = self.0.lock().unwrap();
        oca_bundle
            .overlays
            .iter()
            .map(|overlay| {
                OcaOverlay(RustOpaque::new(Mutex::new(oca_rs::dyn_clone::clone_box(
                    &**overlay,
                ))))
            })
            .collect()
    }
}

pub struct OcaCaptureBase(pub RustOpaque<Mutex<OcaCaptureBaseRaw>>);

impl OcaCaptureBase {
    pub fn attributes(&self) -> OcaMap {
        let capture_base = self.0.lock().unwrap();
        OcaMap(RustOpaque::new(Mutex::new(StringMap(
            capture_base.attributes.clone(),
        ))))
    }

    pub fn flagged_attributes(&self) -> Vec<String> {
        let capture_base = self.0.lock().unwrap();
        capture_base.flagged_attributes.clone()
    }
}

pub struct OcaOverlay(pub RustOpaque<Mutex<DynOverlay>>);

// TODO: use regular HashMap when FRB supports it
pub struct OcaMap(pub RustOpaque<Mutex<StringMap>>);
pub struct StringMap(HashMap<String, String>);

impl OcaMap {
    pub fn new() -> OcaMap {
        OcaMap(RustOpaque::new(Mutex::new(StringMap(HashMap::new()))))
    }

    pub fn insert(&self, key: String, value: String) {
        let mut map = self.0.lock().unwrap();
        map.0.insert(key, value);
    }

    pub fn get(&self, key: String) -> Option<String> {
        let map = self.0.lock().unwrap();
        map.0.get(&key).map(|v| v.to_owned())
    }

    pub fn remove(&self, key: String) {
        let mut map = self.0.lock().unwrap();
        map.0.remove(&key);
    }

    pub fn get_keys(&self) -> Vec<String> {
        let map = self.0.lock().unwrap();
        map.0.keys().map(|k| k.to_owned()).collect()
    }
}
