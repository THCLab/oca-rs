use std::sync::{Arc, Mutex};

use flutter_rust_bridge::{frb, RustOpaque};
use oca_rs::state::{
    attribute::{Attribute, AttributeType},
    encoding::Encoding,
    oca::{OCABox as OCABoxRaw, OCABundle as OCABundleRaw},
};

pub struct OcaBox {
    pub meta_attrs: Vec<OcaMetaAttr>,
    pub attrs: Vec<OcaAttr>,
}

pub struct OcaMetaAttr {
    name: String,
    value: String,
}

pub struct OcaAttr {
    pub name: String,
    pub attribute_type: Box<AttributeType>,
    pub is_flagged: bool,
    pub encoding: Box<Encoding>,
    pub cardinality: Option<String>,
    pub conformance: Option<String>,
}

#[frb(mirror(AttributeType))]
pub enum _AttributeType {
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

#[frb(mirror(Encoding))]
pub enum _Encoding {
    Base64,
    Utf8,
    Iso8859_1,
}

impl OcaBox {
    fn generate_bundle(&self) -> OcaBundle {
        let mut oca_box = OCABoxRaw::new();

        for meta_attr in &self.meta_attrs {
            oca_box.add_meta_attribute(meta_attr.name.clone(), meta_attr.value.clone());
        }

        for attr in &self.attrs {
            let mut attribute = Attribute::new(attr.name.clone());
            attribute.set_attribute_type(*attr.attribute_type.clone());
            // attribute.set_encoding(*attr.encoding.clone());
            if attr.is_flagged {
                attribute.set_flagged();
            }
            // attribute.set_cardinality(attr.cardinality.clone());
            // attribute.set_conformance(attr.conformance.clone());
            oca_box.add_attribute(attribute);
        }

        let oca_bundle = oca_box.generate_bundle();

        OcaBundle(RustOpaque::new(Arc::new(Mutex::new(oca_bundle))))
    }
}

pub struct OcaBundle(RustOpaque<Arc<Mutex<OCABundleRaw>>>);

impl OcaBundle {
    pub fn to_json(&self) -> String {
        let oca_bundle = self.0.lock().unwrap();
        serde_json::to_string(&*oca_bundle).unwrap()
    }

    pub fn capture_base(&self) -> OcaCaptureBase {
        OcaCaptureBase(RustOpaque::new(Arc::clone(&self.0)))
    }

    pub fn overlays(&self) -> Vec<OcaOverlay> {
        let oca_bundle = self.0.lock().unwrap();
        let mut overlays = Vec::new();
        for overlay in &oca_bundle.overlays {
            overlays.push(OcaOverlay {
                said: overlay.said().to_owned(),
            });
        }
        overlays
    }
}

pub struct OcaCaptureBase(RustOpaque<Arc<Mutex<OCABundleRaw>>>);

impl OcaCaptureBase {
    pub fn attributes(&self) -> Vec<OcaCaptureBaseAttr> {
        let oca_bundle = self.0.lock().unwrap();
        let mut attributes = Vec::new();
        for (name, value) in &oca_bundle.capture_base.attributes {
            attributes.push(OcaCaptureBaseAttr {
                name: name.to_string(),
                value: value.to_string(),
            });
        }
        attributes
    }

    pub fn flagged_attributes(&self) -> Vec<String> {
        let oca_bundle = self.0.lock().unwrap();
        oca_bundle.capture_base.flagged_attributes.clone()
    }
}

pub struct OcaCaptureBaseAttr {
    pub name: String,
    pub value: String,
}

pub struct OcaOverlay {
    said: String,
    // TODO: add rest of fields
}
