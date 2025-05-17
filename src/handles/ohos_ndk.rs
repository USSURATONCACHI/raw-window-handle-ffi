use std::{ffi::c_void, ptr::NonNull};

/// raw_window_handle_ffi:
///         This type is ABI-stable and FFI-compatible analogue for [`raw_window_handle::OhosNdkWindowHandle`].
///         Can be converted to and from the referenced type.
///
/// Raw window handle for Ohos NDK.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OhosNdkWindowHandle {
    pub native_window: NonNull<c_void>,
}

unsafe impl Send for OhosNdkWindowHandle {}
unsafe impl Sync for OhosNdkWindowHandle {}

impl From<raw_window_handle::OhosNdkWindowHandle> for OhosNdkWindowHandle {
    fn from(value: raw_window_handle::OhosNdkWindowHandle) -> Self {
        Self {
            native_window: value.native_window,
        }
    }
}

impl OhosNdkWindowHandle {
    /// Converts FFI type back to the original [`raw_window_handle::AppKitWindowHandle`].
    ///
    /// # Safety
    /// Original type [`raw_window_handle::AppKitWindowHandle`] is marked as `non_exaustive`,
    /// and therefore it is impossible to convert to it completely safely.
    pub unsafe fn into(self) -> raw_window_handle::OhosNdkWindowHandle {
        raw_window_handle::OhosNdkWindowHandle::new(self.native_window)
    }
}
