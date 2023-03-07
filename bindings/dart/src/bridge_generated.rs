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
fn wire_add_meta_attr__method__OcaBox_impl(
    port_: MessagePort,
    that: impl Wire2Api<OcaBox> + UnwindSafe,
    name: impl Wire2Api<String> + UnwindSafe,
    value: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "add_meta_attr__method__OcaBox",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_name = name.wire2api();
            let api_value = value.wire2api();
            move |task_callback| Ok(OcaBox::add_meta_attr(&api_that, api_name, api_value))
        },
    )
}
fn wire_add_attr__method__OcaBox_impl(
    port_: MessagePort,
    that: impl Wire2Api<OcaBox> + UnwindSafe,
    attr: impl Wire2Api<OcaAttr> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "add_attr__method__OcaBox",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_attr = attr.wire2api();
            move |task_callback| Ok(OcaBox::add_attr(&api_that, api_attr))
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
fn wire_new__static_method__OcaAttr_impl(
    port_: MessagePort,
    name: impl Wire2Api<String> + UnwindSafe,
    attr_type: impl Wire2Api<OcaAttrType> + UnwindSafe,
    encoding: impl Wire2Api<OcaEncoding> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "new__static_method__OcaAttr",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_name = name.wire2api();
            let api_attr_type = attr_type.wire2api();
            let api_encoding = encoding.wire2api();
            move |task_callback| Ok(OcaAttr::new(api_name, api_attr_type, api_encoding))
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

// Section: executor

support::lazy_static! {
    pub static ref FLUTTER_RUST_BRIDGE_HANDLER: support::DefaultHandler = Default::default();
}

#[cfg(not(target_family = "wasm"))]
#[path = "bridge_generated.io.rs"]
mod io;
#[cfg(not(target_family = "wasm"))]
pub use io::*;