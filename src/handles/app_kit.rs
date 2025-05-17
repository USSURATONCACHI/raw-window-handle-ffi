use std::{ffi::c_void, ptr::NonNull};

use crate::{RawWindowHandle, RawWindowHandleData};

/// raw_window_handle_ffi:
///         This type is ABI-stable and FFI-compatible analogue for [`raw_window_handle::AppKitWindowHandle`].
///         Can be converted to and from the referenced type.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AppKitWindowHandle {
    /// A pointer to an `NSView` object.
    pub ns_view: NonNull<c_void>,
}

unsafe impl Send for AppKitWindowHandle {}
unsafe impl Sync for AppKitWindowHandle {}

impl From<raw_window_handle::AppKitWindowHandle> for AppKitWindowHandle {
    fn from(value: raw_window_handle::AppKitWindowHandle) -> Self {
        Self {
            ns_view: value.ns_view,
        }
    }
}

impl AppKitWindowHandle {
    /// Converts FFI type back to the original [`raw_window_handle::AppKitWindowHandle`].
    ///
    /// # Safety
    /// Original type [`raw_window_handle::AppKitWindowHandle`] is marked as `non_exaustive`,
    /// and therefore it is impossible to convert to it completely safely.
    pub unsafe fn into(self) -> raw_window_handle::AppKitWindowHandle {
        raw_window_handle::AppKitWindowHandle::new(self.ns_view)
    }
}

impl From<AppKitWindowHandle> for RawWindowHandle {
    fn from(val: AppKitWindowHandle) -> RawWindowHandle {
        RawWindowHandle {
            kind: crate::RawWindowHandleKind::AppKitWindowHandle,
            data: RawWindowHandleData { app_kit: val },
        }
    }
}
