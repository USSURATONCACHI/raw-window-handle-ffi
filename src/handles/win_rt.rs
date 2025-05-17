use std::{ffi::c_void, ptr::NonNull};

/// raw_window_handle_ffi:
///         This type is ABI-stable and FFI-compatible analogue for [`raw_window_handle::WinRtWindowHandle`].
///         Can be converted to and from the referenced type.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WinRtWindowHandle {
    /// A WinRT `CoreWindow` handle.
    pub core_window: NonNull<c_void>,
}

unsafe impl Send for WinRtWindowHandle {}
unsafe impl Sync for WinRtWindowHandle {}

impl From<raw_window_handle::WinRtWindowHandle> for WinRtWindowHandle {
    fn from(value: raw_window_handle::WinRtWindowHandle) -> Self {
        Self {
            core_window: value.core_window,
        }
    }
}

impl WinRtWindowHandle {
    /// Converts FFI type back to the original [`raw_window_handle::AppKitWindowHandle`].
    /// 
    /// # Safety
    /// Original type [`raw_window_handle::AppKitWindowHandle`] is marked as `non_exaustive`,
    /// and therefore it is impossible to convert to it completely safely.
    pub unsafe fn into(self) -> raw_window_handle::WinRtWindowHandle {
        raw_window_handle::WinRtWindowHandle::new(self.core_window)
    }
}
