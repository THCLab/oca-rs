use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_to_json__method__OcaBundle(port_: i64, that: *mut wire_OcaBundle) {
    wire_to_json__method__OcaBundle_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_capture_base__method__OcaBundle(port_: i64, that: *mut wire_OcaBundle) {
    wire_capture_base__method__OcaBundle_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_overlays__method__OcaBundle(port_: i64, that: *mut wire_OcaBundle) {
    wire_overlays__method__OcaBundle_impl(port_, that)
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
pub extern "C" fn new_ArcMutexOcaBundleRaw() -> wire_ArcMutexOcaBundleRaw {
    wire_ArcMutexOcaBundleRaw::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_oca_bundle_0() -> *mut wire_OcaBundle {
    support::new_leak_box_ptr(wire_OcaBundle::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_oca_capture_base_0() -> *mut wire_OcaCaptureBase {
    support::new_leak_box_ptr(wire_OcaCaptureBase::new_with_null_ptr())
}

// Section: related functions

#[no_mangle]
pub extern "C" fn drop_opaque_ArcMutexOcaBundleRaw(ptr: *const c_void) {
    unsafe {
        Arc::<Arc<Mutex<OCABundleRaw>>>::decrement_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn share_opaque_ArcMutexOcaBundleRaw(ptr: *const c_void) -> *const c_void {
    unsafe {
        Arc::<Arc<Mutex<OCABundleRaw>>>::increment_strong_count(ptr as _);
        ptr
    }
}

// Section: impl Wire2Api

impl Wire2Api<RustOpaque<Arc<Mutex<OCABundleRaw>>>> for wire_ArcMutexOcaBundleRaw {
    fn wire2api(self) -> RustOpaque<Arc<Mutex<OCABundleRaw>>> {
        unsafe { support::opaque_from_dart(self.ptr as _) }
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
// Section: wire structs

#[repr(C)]
#[derive(Clone)]
pub struct wire_ArcMutexOcaBundleRaw {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_OcaBundle {
    field0: wire_ArcMutexOcaBundleRaw,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_OcaCaptureBase {
    field0: wire_ArcMutexOcaBundleRaw,
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

impl NewWithNullPtr for wire_ArcMutexOcaBundleRaw {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}

impl NewWithNullPtr for wire_OcaBundle {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: wire_ArcMutexOcaBundleRaw::new_with_null_ptr(),
        }
    }
}

impl NewWithNullPtr for wire_OcaCaptureBase {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: wire_ArcMutexOcaBundleRaw::new_with_null_ptr(),
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
