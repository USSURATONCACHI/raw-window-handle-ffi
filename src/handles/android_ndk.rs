use std::{ffi::c_void, ptr::NonNull};

use crate::{RawWindowHandle, RawWindowHandleData};

/// raw_window_handle_ffi:
///         This type is ABI-stable and FFI-compatible analogue for [`raw_window_handle::AndroidNdkWindowHandle`].
///         Can be converted to and from the referenced type.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AndroidNdkWindowHandle {
    /// A pointer to an `ANativeWindow`.
    pub a_native_window: NonNull<c_void>,
}

unsafe impl Send for AndroidNdkWindowHandle {}
unsafe impl Sync for AndroidNdkWindowHandle {}

impl From<raw_window_handle::AndroidNdkWindowHandle> for AndroidNdkWindowHandle {
    fn from(value: raw_window_handle::AndroidNdkWindowHandle) -> Self {
        Self {
            a_native_window: value.a_native_window,
        }
    }
}

impl AndroidNdkWindowHandle {
    /// Converts FFI type back to the original [`raw_window_handle::AndroidNdkWindowHandle`].
    ///
    /// # Safety
    /// Original type [`raw_window_handle::AndroidNdkWindowHandle`] is marked as `non_exaustive`,
    /// and therefore it is impossible to convert to it completely safely.
    pub unsafe fn into(self) -> raw_window_handle::AndroidNdkWindowHandle {
        raw_window_handle::AndroidNdkWindowHandle::new(self.a_native_window)
    }
}

impl From<AndroidNdkWindowHandle> for RawWindowHandle {
    fn from(val: AndroidNdkWindowHandle) -> RawWindowHandle {
        RawWindowHandle {
            kind: crate::RawWindowHandleKind::AndroidNdkWindowHandle,
            data: RawWindowHandleData { android_ndk: val },
        }
    }
}
