use std::{ffi::c_void, ptr::NonNull};

/// raw_window_handle_ffi:
///         This type is ABI-stable and FFI-compatible analogue for [`raw_window_handle::WaylandWindowHandle`].
///         Can be converted to and from the referenced type.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WaylandWindowHandle {
    /// A pointer to a `wl_surface`.
    pub surface: NonNull<c_void>,
}

unsafe impl Send for WaylandWindowHandle {}
unsafe impl Sync for WaylandWindowHandle {}

impl From<raw_window_handle::WaylandWindowHandle> for WaylandWindowHandle {
    fn from(value: raw_window_handle::WaylandWindowHandle) -> Self {
        Self {
            surface: value.surface,
        }
    }
}

impl WaylandWindowHandle {
    /// Converts FFI type back to the original [`raw_window_handle::AppKitWindowHandle`].
    /// 
    /// # Safety
    /// Original type [`raw_window_handle::AppKitWindowHandle`] is marked as `non_exaustive`,
    /// and therefore it is impossible to convert to it completely safely.
    pub unsafe fn into(self) -> raw_window_handle::WaylandWindowHandle {
        raw_window_handle::WaylandWindowHandle::new(self.surface)
    }
}
