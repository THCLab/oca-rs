use super::*;
use std::sync::Mutex;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_new__static_method__OcaBox(port_: i64) {
    wire_new__static_method__OcaBox_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_add_meta_attr__method__OcaBox(
    port_: i64,
    that: *mut wire_OcaBox,
    name: *mut wire_uint_8_list,
    value: *mut wire_uint_8_list,
) {
    wire_add_meta_attr__method__OcaBox_impl(port_, that, name, value)
}

#[no_mangle]
pub extern "C" fn wire_add_attr__method__OcaBox(
    port_: i64,
    that: *mut wire_OcaBox,
    attr: *mut wire_OcaAttr,
) {
    wire_add_attr__method__OcaBox_impl(port_, that, attr)
}

#[no_mangle]
pub extern "C" fn wire_generate_bundle__method__OcaBox(port_: i64, that: *mut wire_OcaBox) {
    wire_generate_bundle__method__OcaBox_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_new__static_method__OcaAttr(
    port_: i64,
    name: *mut wire_uint_8_list,
    attr_type: i32,
    encoding: i32,
) {
    wire_new__static_method__OcaAttr_impl(port_, name, attr_type, encoding)
}

#[no_mangle]
pub extern "C" fn wire_set_flagged__method__OcaAttr(port_: i64, that: *mut wire_OcaAttr) {
    wire_set_flagged__method__OcaAttr_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_set_cardinality__method__OcaAttr(
    port_: i64,
    that: *mut wire_OcaAttr,
    cardinality: *mut wire_uint_8_list,
) {
    wire_set_cardinality__method__OcaAttr_impl(port_, that, cardinality)
}

#[no_mangle]
pub extern "C" fn wire_set_conformance__method__OcaAttr(
    port_: i64,
    that: *mut wire_OcaAttr,
    conformance: *mut wire_uint_8_list,
) {
    wire_set_conformance__method__OcaAttr_impl(port_, that, conformance)
}

#[no_mangle]
pub extern "C" fn wire_to_json__method__OcaBundle(port_: i64, that: *mut wire_OcaBundle) {
    wire_to_json__method__OcaBundle_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_capture_base__method__OcaBundle(port_: i64, that: *mut wire_OcaBundle) {
    wire_capture_base__method__OcaBundle_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_attributes__method__OcaCaptureBase(
    port_: i64,
    that: *mut wire_OcaCaptureBase,
) {
    wire_attributes__method__OcaCaptureBase_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_flagged_attributes__method__OcaCaptureBase(
    port_: i64,
    that: *mut wire_OcaCaptureBase,
) {
    wire_flagged_attributes__method__OcaCaptureBase_impl(port_, that)
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_MutexOcaAttrRaw() -> wire_MutexOcaAttrRaw {
    wire_MutexOcaAttrRaw::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_MutexOcaBoxRaw() -> wire_MutexOcaBoxRaw {
    wire_MutexOcaBoxRaw::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_MutexOcaBundleRaw() -> wire_MutexOcaBundleRaw {
    wire_MutexOcaBundleRaw::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_MutexOcaCaptureBaseRaw() -> wire_MutexOcaCaptureBaseRaw {
    wire_MutexOcaCaptureBaseRaw::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_oca_attr_0() -> *mut wire_OcaAttr {
    support::new_leak_box_ptr(wire_OcaAttr::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_oca_box_0() -> *mut wire_OcaBox {
    support::new_leak_box_ptr(wire_OcaBox::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_oca_bundle_0() -> *mut wire_OcaBundle {
    support::new_leak_box_ptr(wire_OcaBundle::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_oca_capture_base_0() -> *mut wire_OcaCaptureBase {
    support::new_leak_box_ptr(wire_OcaCaptureBase::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_uint_8_list_0(len: i32) -> *mut wire_uint_8_list {
    let ans = wire_uint_8_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

// Section: related functions

#[no_mangle]
pub extern "C" fn drop_opaque_MutexOcaAttrRaw(ptr: *const c_void) {
    unsafe {
        Arc::<Mutex<OcaAttrRaw>>::decrement_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn share_opaque_MutexOcaAttrRaw(ptr: *const c_void) -> *const c_void {
    unsafe {
        Arc::<Mutex<OcaAttrRaw>>::increment_strong_count(ptr as _);
        ptr
    }
}

#[no_mangle]
pub extern "C" fn drop_opaque_MutexOcaBoxRaw(ptr: *const c_void) {
    unsafe {
        Arc::<Mutex<OcaBoxRaw>>::decrement_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn share_opaque_MutexOcaBoxRaw(ptr: *const c_void) -> *const c_void {
    unsafe {
        Arc::<Mutex<OcaBoxRaw>>::increment_strong_count(ptr as _);
        ptr
    }
}

#[no_mangle]
pub extern "C" fn drop_opaque_MutexOcaBundleRaw(ptr: *const c_void) {
    unsafe {
        Arc::<Mutex<OcaBundleRaw>>::decrement_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn share_opaque_MutexOcaBundleRaw(ptr: *const c_void) -> *const c_void {
    unsafe {
        Arc::<Mutex<OcaBundleRaw>>::increment_strong_count(ptr as _);
        ptr
    }
}

#[no_mangle]
pub extern "C" fn drop_opaque_MutexOcaCaptureBaseRaw(ptr: *const c_void) {
    unsafe {
        Arc::<Mutex<OcaCaptureBaseRaw>>::decrement_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn share_opaque_MutexOcaCaptureBaseRaw(ptr: *const c_void) -> *const c_void {
    unsafe {
        Arc::<Mutex<OcaCaptureBaseRaw>>::increment_strong_count(ptr as _);
        ptr
    }
}

// Section: impl Wire2Api

impl Wire2Api<RustOpaque<Mutex<OcaAttrRaw>>> for wire_MutexOcaAttrRaw {
    fn wire2api(self) -> RustOpaque<Mutex<OcaAttrRaw>> {
        unsafe { support::opaque_from_dart(self.ptr as _) }
    }
}
impl Wire2Api<RustOpaque<Mutex<OcaBoxRaw>>> for wire_MutexOcaBoxRaw {
    fn wire2api(self) -> RustOpaque<Mutex<OcaBoxRaw>> {
        unsafe { support::opaque_from_dart(self.ptr as _) }
    }
}
impl Wire2Api<RustOpaque<Mutex<OcaBundleRaw>>> for wire_MutexOcaBundleRaw {
    fn wire2api(self) -> RustOpaque<Mutex<OcaBundleRaw>> {
        unsafe { support::opaque_from_dart(self.ptr as _) }
    }
}
impl Wire2Api<RustOpaque<Mutex<OcaCaptureBaseRaw>>> for wire_MutexOcaCaptureBaseRaw {
    fn wire2api(self) -> RustOpaque<Mutex<OcaCaptureBaseRaw>> {
        unsafe { support::opaque_from_dart(self.ptr as _) }
    }
}
impl Wire2Api<String> for *mut wire_uint_8_list {
    fn wire2api(self) -> String {
        let vec: Vec<u8> = self.wire2api();
        String::from_utf8_lossy(&vec).into_owned()
    }
}
impl Wire2Api<OcaAttr> for *mut wire_OcaAttr {
    fn wire2api(self) -> OcaAttr {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<OcaAttr>::wire2api(*wrap).into()
    }
}
impl Wire2Api<OcaBox> for *mut wire_OcaBox {
    fn wire2api(self) -> OcaBox {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<OcaBox>::wire2api(*wrap).into()
    }
}
impl Wire2Api<OcaBundle> for *mut wire_OcaBundle {
    fn wire2api(self) -> OcaBundle {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<OcaBundle>::wire2api(*wrap).into()
    }
}
impl Wire2Api<OcaCaptureBase> for *mut wire_OcaCaptureBase {
    fn wire2api(self) -> OcaCaptureBase {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<OcaCaptureBase>::wire2api(*wrap).into()
    }
}

impl Wire2Api<OcaAttr> for wire_OcaAttr {
    fn wire2api(self) -> OcaAttr {
        OcaAttr(self.field0.wire2api())
    }
}

impl Wire2Api<OcaBox> for wire_OcaBox {
    fn wire2api(self) -> OcaBox {
        OcaBox(self.field0.wire2api())
    }
}
impl Wire2Api<OcaBundle> for wire_OcaBundle {
    fn wire2api(self) -> OcaBundle {
        OcaBundle(self.field0.wire2api())
    }
}
impl Wire2Api<OcaCaptureBase> for wire_OcaCaptureBase {
    fn wire2api(self) -> OcaCaptureBase {
        OcaCaptureBase(self.field0.wire2api())
    }
}

impl Wire2Api<Vec<u8>> for *mut wire_uint_8_list {
    fn wire2api(self) -> Vec<u8> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
// Section: wire structs

#[repr(C)]
#[derive(Clone)]
pub struct wire_MutexOcaAttrRaw {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_MutexOcaBoxRaw {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_MutexOcaBundleRaw {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_MutexOcaCaptureBaseRaw {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_OcaAttr {
    field0: wire_MutexOcaAttrRaw,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_OcaBox {
    field0: wire_MutexOcaBoxRaw,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_OcaBundle {
    field0: wire_MutexOcaBundleRaw,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_OcaCaptureBase {
    field0: wire_MutexOcaCaptureBaseRaw,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_uint_8_list {
    ptr: *mut u8,
    len: i32,
}

// Section: impl NewWithNullPtr

pub trait NewWithNullPtr {
    fn new_with_null_ptr() -> Self;
}

impl<T> NewWithNullPtr for *mut T {
    fn new_with_null_ptr() -> Self {
        std::ptr::null_mut()
    }
}

impl NewWithNullPtr for wire_MutexOcaAttrRaw {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
impl NewWithNullPtr for wire_MutexOcaBoxRaw {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
impl NewWithNullPtr for wire_MutexOcaBundleRaw {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
impl NewWithNullPtr for wire_MutexOcaCaptureBaseRaw {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}

impl NewWithNullPtr for wire_OcaAttr {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: wire_MutexOcaAttrRaw::new_with_null_ptr(),
        }
    }
}

impl NewWithNullPtr for wire_OcaBox {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: wire_MutexOcaBoxRaw::new_with_null_ptr(),
        }
    }
}

impl NewWithNullPtr for wire_OcaBundle {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: wire_MutexOcaBundleRaw::new_with_null_ptr(),
        }
    }
}

impl NewWithNullPtr for wire_OcaCaptureBase {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: wire_MutexOcaCaptureBaseRaw::new_with_null_ptr(),
        }
    }
}

// Section: sync execution mode utility

#[no_mangle]
pub extern "C" fn free_WireSyncReturn(ptr: support::WireSyncReturn) {
    unsafe {
        let _ = support::box_from_leak_ptr(ptr);
    };
}
