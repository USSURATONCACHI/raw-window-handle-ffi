use std::{ffi::c_void, ptr::NonNull};

/// raw_window_handle_ffi:
///         This type is ABI-stable and FFI-compatible analogue for [`raw_window_handle::GbmWindowHandle`].
///         Can be converted to and from the referenced type.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GbmWindowHandle {
    /// The gbm surface.
    pub gbm_surface: NonNull<c_void>,
}

unsafe impl Send for GbmWindowHandle {}
unsafe impl Sync for GbmWindowHandle {}

impl From<raw_window_handle::GbmWindowHandle> for GbmWindowHandle {
    fn from(value: raw_window_handle::GbmWindowHandle) -> Self {
        Self {
            gbm_surface: value.gbm_surface,
        }
    }
}

impl GbmWindowHandle {
    /// Converts FFI type back to the original [`raw_window_handle::GbmWindowHandle`].
    ///
    /// # Safety
    /// Original type [`raw_window_handle::GbmWindowHandle`] is marked as `non_exaustive`,
    /// and therefore it is impossible to convert to it completely safely.
    pub unsafe fn into(self) -> raw_window_handle::GbmWindowHandle {
        raw_window_handle::GbmWindowHandle::new(self.gbm_surface)
    }
}
