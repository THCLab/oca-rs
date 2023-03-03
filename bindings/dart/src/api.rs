use std::sync::Mutex;

use flutter_rust_bridge::{frb, RustOpaque};
pub use oca_rs::state::{
    attribute::{Attribute as OcaAttrRaw, AttributeType as OcaAttrType},
    encoding::Encoding as OcaEncoding,
    oca::{
        capture_base::CaptureBase as OcaCaptureBaseRaw, OCABox as OcaBoxRaw,
        OCABundle as OcaBundleRaw,
    },
};

pub struct OcaBox(pub RustOpaque<Mutex<OcaBoxRaw>>);

impl OcaBox {
    pub fn new() -> OcaBox {
        OcaBox(RustOpaque::new(Mutex::new(OcaBoxRaw::new())))
    }

    pub fn add_meta_attr(&self, name: String, value: String) {
        let mut oca_box = self.0.lock().unwrap();
        oca_box.add_meta_attribute(name, value);
    }

    pub fn add_attr(&self, attr: OcaAttr) {
        let mut oca_box = self.0.lock().unwrap();
        oca_box.add_attribute(attr.0.lock().unwrap().clone());
    }

    pub fn generate_bundle(&self) -> OcaBundle {
        let mut oca_box = self.0.lock().unwrap();
        let oca_bundle = oca_box.generate_bundle();
        OcaBundle(RustOpaque::new(Mutex::new(oca_bundle)))
    }
}

pub struct OcaAttr(pub RustOpaque<Mutex<OcaAttrRaw>>);

impl OcaAttr {
    pub fn new(name: String, attr_type: OcaAttrType, encoding: OcaEncoding) -> OcaAttr {
        let mut attr = OcaAttrRaw::new(name);
        attr.set_attribute_type(attr_type);
        // attr.set_encoding(encoding);
        OcaAttr(RustOpaque::new(Mutex::new(attr)))
    }

    pub fn set_flagged(&self) {
        let mut attr = self.0.lock().unwrap();
        attr.set_flagged();
    }

    pub fn set_cardinality(&self, cardinality: String) {
        let mut attr = self.0.lock().unwrap();
        // attr.set_cardinality(cardinality);
    }

    pub fn set_conformance(&self, conformance: String) {
        let mut attr = self.0.lock().unwrap();
        // attr.set_conformance(conformance);
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

pub struct OcaBundle(pub RustOpaque<Mutex<OcaBundleRaw>>);

impl OcaBundle {
    pub fn to_json(&self) -> String {
        let oca_bundle = self.0.lock().unwrap();
        serde_json::to_string(&*oca_bundle).unwrap()
    }

    pub fn capture_base(&self) -> OcaCaptureBase {
        let oca_bundle = self.0.lock().unwrap();
        OcaCaptureBase(RustOpaque::new(Mutex::new(oca_bundle.capture_base.clone())))
    }
}

pub struct OcaCaptureBase(pub RustOpaque<Mutex<OcaCaptureBaseRaw>>);

impl OcaCaptureBase {
    pub fn attributes(&self) -> Vec<Vec<String>> {
        let capture_base = self.0.lock().unwrap();
        capture_base
            .attributes
            .iter()
            .map(|(k, v)| vec![k.to_owned(), v.to_owned()])
            .collect()
    }

    pub fn flagged_attributes(&self) -> Vec<String> {
        let capture_base = self.0.lock().unwrap();
        capture_base.flagged_attributes.clone()
    }
}
