#![allow(
    non_camel_case_types,
    unused,
    clippy::redundant_closure,
    clippy::useless_conversion,
    clippy::unit_arg,
    clippy::double_parens,
    non_snake_case,
    clippy::too_many_arguments
)]
// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.63.1.

use crate::api::*;
use core::panic::UnwindSafe;
use flutter_rust_bridge::*;
use std::ffi::c_void;
use std::sync::Arc;

// Section: imports

// Section: wire functions

fn wire_load_oca_impl(port_: MessagePort, json: impl Wire2Api<String> + UnwindSafe) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "load_oca",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_json = json.wire2api();
            move |task_callback| load_oca(api_json)
        },
    )
}
fn wire_new__static_method__OcaBox_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "new__static_method__OcaBox",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| Ok(OcaBox::new()),
    )
}
fn wire_add_meta__method__OcaBox_impl(
    port_: MessagePort,
    that: impl Wire2Api<OcaBox> + UnwindSafe,
    lang: impl Wire2Api<String> + UnwindSafe,
    name: impl Wire2Api<String> + UnwindSafe,
    value: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "add_meta__method__OcaBox",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_lang = lang.wire2api();
            let api_name = name.wire2api();
            let api_value = value.wire2api();
            move |task_callback| OcaBox::add_meta(&api_that, api_lang, api_name, api_value)
        },
    )
}
fn wire_add_attribute__method__OcaBox_impl(
    port_: MessagePort,
    that: impl Wire2Api<OcaBox> + UnwindSafe,
    attr: impl Wire2Api<OcaAttr> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "add_attribute__method__OcaBox",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_attr = attr.wire2api();
            move |task_callback| Ok(OcaBox::add_attribute(&api_that, api_attr))
        },
    )
}
fn wire_generate_bundle__method__OcaBox_impl(
    port_: MessagePort,
    that: impl Wire2Api<OcaBox> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "generate_bundle__method__OcaBox",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            move |task_callback| Ok(OcaBox::generate_bundle(&api_that))
        },
    )
}
fn wire_add_form_layout__method__OcaBox_impl(
    port_: MessagePort,
    that: impl Wire2Api<OcaBox> + UnwindSafe,
    layout: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "add_form_layout__method__OcaBox",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_layout = layout.wire2api();
            move |task_callback| Ok(OcaBox::add_form_layout(&api_that, api_layout))
        },
    )
}
fn wire_add_credential_layout__method__OcaBox_impl(
    port_: MessagePort,
    that: impl Wire2Api<OcaBox> + UnwindSafe,
    layout: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "add_credential_layout__method__OcaBox",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_layout = layout.wire2api();
            move |task_callback| Ok(OcaBox::add_credential_layout(&api_that, api_layout))
        },
    )
}
fn wire_new__static_method__OcaAttr_impl(
    port_: MessagePort,
    name: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "new__static_method__OcaAttr",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_name = name.wire2api();
            move |task_callback| Ok(OcaAttr::new(api_name))
        },
    )
}
fn wire_set_attribute_type__method__OcaAttr_impl(
    port_: MessagePort,
    that: impl Wire2Api<OcaAttr> + UnwindSafe,
    attr_type: impl Wire2Api<OcaAttrType> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "set_attribute_type__method__OcaAttr",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_attr_type = attr_type.wire2api();
            move |task_callback| Ok(OcaAttr::set_attribute_type(&api_that, api_attr_type))
        },
    )
}
fn wire_set_flagged__method__OcaAttr_impl(
    port_: MessagePort,
    that: impl Wire2Api<OcaAttr> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "set_flagged__method__OcaAttr",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            move |task_callback| Ok(OcaAttr::set_flagged(&api_that))
        },
    )
}
fn wire_set_encoding__method__OcaAttr_impl(
    port_: MessagePort,
    that: impl Wire2Api<OcaAttr> + UnwindSafe,
    encoding: impl Wire2Api<OcaEncoding> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "set_encoding__method__OcaAttr",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_encoding = encoding.wire2api();
            move |task_callback| Ok(OcaAttr::set_encoding(&api_that, api_encoding))
        },
    )
}
fn wire_set_cardinality__method__OcaAttr_impl(
    port_: MessagePort,
    that: impl Wire2Api<OcaAttr> + UnwindSafe,
    cardinality: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "set_cardinality__method__OcaAttr",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_cardinality = cardinality.wire2api();
            move |task_callback| Ok(OcaAttr::set_cardinality(&api_that, api_cardinality))
        },
    )
}
fn wire_set_conformance__method__OcaAttr_impl(
    port_: MessagePort,
    that: impl Wire2Api<OcaAttr> + UnwindSafe,
    conformance: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "set_conformance__method__OcaAttr",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_conformance = conformance.wire2api();
            move |task_callback| Ok(OcaAttr::set_conformance(&api_that, api_conformance))
        },
    )
}
fn wire_set_label__method__OcaAttr_impl(
    port_: MessagePort,
    that: impl Wire2Api<OcaAttr> + UnwindSafe,
    lang: impl Wire2Api<String> + UnwindSafe,
    label: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "set_label__method__OcaAttr",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_lang = lang.wire2api();
            let api_label = label.wire2api();
            move |task_callback| OcaAttr::set_label(&api_that, api_lang, api_label)
        },
    )
}
fn wire_set_information__method__OcaAttr_impl(
    port_: MessagePort,
    that: impl Wire2Api<OcaAttr> + UnwindSafe,
    lang: impl Wire2Api<String> + UnwindSafe,
    information: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "set_information__method__OcaAttr",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_lang = lang.wire2api();
            let api_information = information.wire2api();
            move |task_callback| OcaAttr::set_information(&api_that, api_lang, api_information)
        },
    )
}
fn wire_set_entry_codes__method__OcaAttr_impl(
    port_: MessagePort,
    that: impl Wire2Api<OcaAttr> + UnwindSafe,
    entry_codes: impl Wire2Api<Vec<String>> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "set_entry_codes__method__OcaAttr",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_entry_codes = entry_codes.wire2api();
            move |task_callback| Ok(OcaAttr::set_entry_codes(&api_that, api_entry_codes))
        },
    )
}
fn wire_set_entry_codes_sai__method__OcaAttr_impl(
    port_: MessagePort,
    that: impl Wire2Api<OcaAttr> + UnwindSafe,
    sai: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "set_entry_codes_sai__method__OcaAttr",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_sai = sai.wire2api();
            move |task_callback| Ok(OcaAttr::set_entry_codes_sai(&api_that, api_sai))
        },
    )
}
fn wire_set_entry__method__OcaAttr_impl(
    port_: MessagePort,
    that: impl Wire2Api<OcaAttr> + UnwindSafe,
    lang: impl Wire2Api<String> + UnwindSafe,
    entries: impl Wire2Api<OcaMap> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "set_entry__method__OcaAttr",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_lang = lang.wire2api();
            let api_entries = entries.wire2api();
            move |task_callback| OcaAttr::set_entry(&api_that, api_lang, api_entries)
        },
    )
}
fn wire_set_unit_metric__method__OcaAttr_impl(
    port_: MessagePort,
    that: impl Wire2Api<OcaAttr> + UnwindSafe,
    unit: impl Wire2Api<OcaMetricUnit> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "set_unit_metric__method__OcaAttr",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_unit = unit.wire2api();
            move |task_callback| Ok(OcaAttr::set_unit_metric(&api_that, api_unit))
        },
    )
}
fn wire_set_unit_imperial__method__OcaAttr_impl(
    port_: MessagePort,
    that: impl Wire2Api<OcaAttr> + UnwindSafe,
    unit: impl Wire2Api<OcaImperialUnit> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "set_unit_imperial__method__OcaAttr",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_unit = unit.wire2api();
            move |task_callback| Ok(OcaAttr::set_unit_imperial(&api_that, api_unit))
        },
    )
}
fn wire_set_format__method__OcaAttr_impl(
    port_: MessagePort,
    that: impl Wire2Api<OcaAttr> + UnwindSafe,
    format: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "set_format__method__OcaAttr",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_format = format.wire2api();
            move |task_callback| Ok(OcaAttr::set_format(&api_that, api_format))
        },
    )
}
fn wire_to_json__method__OcaBundle_impl(
    port_: MessagePort,
    that: impl Wire2Api<OcaBundle> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "to_json__method__OcaBundle",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            move |task_callback| Ok(OcaBundle::to_json(&api_that))
        },
    )
}
fn wire_said__method__OcaBundle_impl(
    port_: MessagePort,
    that: impl Wire2Api<OcaBundle> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "said__method__OcaBundle",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            move |task_callback| Ok(OcaBundle::said(&api_that))
        },
    )
}
fn wire_capture_base__method__OcaBundle_impl(
    port_: MessagePort,
    that: impl Wire2Api<OcaBundle> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "capture_base__method__OcaBundle",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            move |task_callback| Ok(OcaBundle::capture_base(&api_that))
        },
    )
}
fn wire_overlays__method__OcaBundle_impl(
    port_: MessagePort,
    that: impl Wire2Api<OcaBundle> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "overlays__method__OcaBundle",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            move |task_callback| Ok(OcaBundle::overlays(&api_that))
        },
    )
}
fn wire_attributes__method__OcaCaptureBase_impl(
    port_: MessagePort,
    that: impl Wire2Api<OcaCaptureBase> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "attributes__method__OcaCaptureBase",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            move |task_callback| Ok(OcaCaptureBase::attributes(&api_that))
        },
    )
}
fn wire_flagged_attributes__method__OcaCaptureBase_impl(
    port_: MessagePort,
    that: impl Wire2Api<OcaCaptureBase> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "flagged_attributes__method__OcaCaptureBase",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            move |task_callback| Ok(OcaCaptureBase::flagged_attributes(&api_that))
        },
    )
}
fn wire_new__static_method__OcaMap_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "new__static_method__OcaMap",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| Ok(OcaMap::new()),
    )
}
fn wire_insert__method__OcaMap_impl(
    port_: MessagePort,
    that: impl Wire2Api<OcaMap> + UnwindSafe,
    key: impl Wire2Api<String> + UnwindSafe,
    value: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "insert__method__OcaMap",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_key = key.wire2api();
            let api_value = value.wire2api();
            move |task_callback| Ok(OcaMap::insert(&api_that, api_key, api_value))
        },
    )
}
fn wire_get__method__OcaMap_impl(
    port_: MessagePort,
    that: impl Wire2Api<OcaMap> + UnwindSafe,
    key: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "get__method__OcaMap",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_key = key.wire2api();
            move |task_callback| Ok(OcaMap::get(&api_that, api_key))
        },
    )
}
fn wire_remove__method__OcaMap_impl(
    port_: MessagePort,
    that: impl Wire2Api<OcaMap> + UnwindSafe,
    key: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "remove__method__OcaMap",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_key = key.wire2api();
            move |task_callback| Ok(OcaMap::remove(&api_that, api_key))
        },
    )
}
fn wire_get_keys__method__OcaMap_impl(
    port_: MessagePort,
    that: impl Wire2Api<OcaMap> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "get_keys__method__OcaMap",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            move |task_callback| Ok(OcaMap::get_keys(&api_that))
        },
    )
}
// Section: wrapper structs

// Section: static checks

// Section: allocate functions

// Section: related functions

// Section: impl Wire2Api

pub trait Wire2Api<T> {
    fn wire2api(self) -> T;
}

impl<T, S> Wire2Api<Option<T>> for *mut S
where
    *mut S: Wire2Api<T>,
{
    fn wire2api(self) -> Option<T> {
        (!self.is_null()).then(|| self.wire2api())
    }
}

impl Wire2Api<i32> for i32 {
    fn wire2api(self) -> i32 {
        self
    }
}

impl Wire2Api<OcaAttrType> for i32 {
    fn wire2api(self) -> OcaAttrType {
        match self {
            0 => OcaAttrType::Boolean,
            1 => OcaAttrType::ArrayBoolean,
            2 => OcaAttrType::Binary,
            3 => OcaAttrType::ArrayBinary,
            4 => OcaAttrType::Text,
            5 => OcaAttrType::ArrayText,
            6 => OcaAttrType::Numeric,
            7 => OcaAttrType::ArrayNumeric,
            8 => OcaAttrType::DateTime,
            9 => OcaAttrType::ArrayDateTime,
            10 => OcaAttrType::Reference,
            11 => OcaAttrType::ArrayReference,
            _ => unreachable!("Invalid variant for OcaAttrType: {}", self),
        }
    }
}

impl Wire2Api<OcaEncoding> for i32 {
    fn wire2api(self) -> OcaEncoding {
        match self {
            0 => OcaEncoding::Base64,
            1 => OcaEncoding::Utf8,
            2 => OcaEncoding::Iso8859_1,
            _ => unreachable!("Invalid variant for OcaEncoding: {}", self),
        }
    }
}
impl Wire2Api<OcaImperialUnit> for i32 {
    fn wire2api(self) -> OcaImperialUnit {
        match self {
            0 => OcaImperialUnit::Pound,
            1 => OcaImperialUnit::Ounce,
            2 => OcaImperialUnit::Gallon,
            3 => OcaImperialUnit::Quart,
            4 => OcaImperialUnit::Pint,
            5 => OcaImperialUnit::FluidOunce,
            6 => OcaImperialUnit::Inch,
            7 => OcaImperialUnit::Foot,
            8 => OcaImperialUnit::Yard,
            9 => OcaImperialUnit::Mile,
            10 => OcaImperialUnit::Celsius,
            11 => OcaImperialUnit::Fahrenheit,
            12 => OcaImperialUnit::Kelvin,
            13 => OcaImperialUnit::Percent,
            14 => OcaImperialUnit::Count,
            15 => OcaImperialUnit::Other,
            _ => unreachable!("Invalid variant for OcaImperialUnit: {}", self),
        }
    }
}

impl Wire2Api<OcaMetricUnit> for i32 {
    fn wire2api(self) -> OcaMetricUnit {
        match self {
            0 => OcaMetricUnit::Kilogram,
            1 => OcaMetricUnit::Gram,
            2 => OcaMetricUnit::Milligram,
            3 => OcaMetricUnit::Liter,
            4 => OcaMetricUnit::Milliliter,
            5 => OcaMetricUnit::Centimeter,
            6 => OcaMetricUnit::Millimeter,
            7 => OcaMetricUnit::Inch,
            8 => OcaMetricUnit::Foot,
            9 => OcaMetricUnit::Yard,
            10 => OcaMetricUnit::Mile,
            11 => OcaMetricUnit::Celsius,
            12 => OcaMetricUnit::Fahrenheit,
            13 => OcaMetricUnit::Kelvin,
            14 => OcaMetricUnit::Percent,
            15 => OcaMetricUnit::Count,
            16 => OcaMetricUnit::Other,
            _ => unreachable!("Invalid variant for OcaMetricUnit: {}", self),
        }
    }
}
impl Wire2Api<u8> for u8 {
    fn wire2api(self) -> u8 {
        self
    }
}

// Section: impl IntoDart

impl support::IntoDart for OcaAttr {
    fn into_dart(self) -> support::DartAbi {
        vec![self.0.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for OcaAttr {}

impl support::IntoDart for OcaBox {
    fn into_dart(self) -> support::DartAbi {
        vec![self.0.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for OcaBox {}

impl support::IntoDart for OcaBundle {
    fn into_dart(self) -> support::DartAbi {
        vec![self.0.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for OcaBundle {}

impl support::IntoDart for OcaCaptureBase {
    fn into_dart(self) -> support::DartAbi {
        vec![self.0.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for OcaCaptureBase {}

impl support::IntoDart for OcaMap {
    fn into_dart(self) -> support::DartAbi {
        vec![self.0.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for OcaMap {}

impl support::IntoDart for OcaOverlay {
    fn into_dart(self) -> support::DartAbi {
        vec![self.0.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for OcaOverlay {}

// Section: executor

support::lazy_static! {
    pub static ref FLUTTER_RUST_BRIDGE_HANDLER: support::DefaultHandler = Default::default();
}

#[cfg(not(target_family = "wasm"))]
#[path = "bridge_generated.io.rs"]
mod io;
#[cfg(not(target_family = "wasm"))]
pub use io::*;
