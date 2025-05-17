use crate::{RawWindowHandle, RawWindowHandleData};
use std::{ffi::c_void, ptr::NonNull};

/// raw_window_handle_ffi:
///         This type is ABI-stable and FFI-compatible analogue for [`raw_window_handle::HaikuWindowHandle`].
///         Can be converted to and from the referenced type.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HaikuWindowHandle {
    /// A pointer to a BWindow object
    pub b_window: NonNull<c_void>,
    /// A pointer to a BDirectWindow object that might be null
    pub b_direct_window: Option<NonNull<c_void>>,
}

unsafe impl Send for HaikuWindowHandle {}
unsafe impl Sync for HaikuWindowHandle {}

impl From<raw_window_handle::HaikuWindowHandle> for HaikuWindowHandle {
    fn from(value: raw_window_handle::HaikuWindowHandle) -> Self {
        Self {
            b_window: value.b_window,
            b_direct_window: value.b_direct_window,
        }
    }
}

impl HaikuWindowHandle {
    /// Converts FFI type back to the original [`raw_window_handle::HaikuWindowHandle`].
    ///
    /// # Safety
    /// Original type [`raw_window_handle::HaikuWindowHandle`] is marked as `non_exaustive`,
    /// and therefore it is impossible to convert to it completely safely.
    pub unsafe fn into(self) -> raw_window_handle::HaikuWindowHandle {
        let mut result = raw_window_handle::HaikuWindowHandle::new(self.b_window);
        result.b_direct_window = self.b_direct_window;
        result
    }
}

impl From<HaikuWindowHandle> for RawWindowHandle {
    fn from(val: HaikuWindowHandle) -> RawWindowHandle {
        RawWindowHandle {
            kind: crate::RawWindowHandleKind::HaikuWindowHandle,
            data: RawWindowHandleData { haiku: val },
        }
    }
}
