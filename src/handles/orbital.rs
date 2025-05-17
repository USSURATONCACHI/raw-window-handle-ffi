use std::{ffi::c_void, ptr::NonNull};

/// raw_window_handle_ffi:
///         This type is ABI-stable and FFI-compatible analogue for [`raw_window_handle::OrbitalWindowHandle`].
///         Can be converted to and from the referenced type.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OrbitalWindowHandle {
    /// A pointer to an orbclient window.
    // TODO(madsmtm): I think this is a file descriptor, so perhaps it should
    // actually use `std::os::fd::RawFd`, or some sort of integer instead?
    pub window: NonNull<c_void>,
}

unsafe impl Send for OrbitalWindowHandle {}
unsafe impl Sync for OrbitalWindowHandle {}

impl From<raw_window_handle::OrbitalWindowHandle> for OrbitalWindowHandle {
    fn from(value: raw_window_handle::OrbitalWindowHandle) -> Self {
        Self {
            window: value.window,
        }
    }
}

impl OrbitalWindowHandle {
    /// Converts FFI type back to the original [`raw_window_handle::AppKitWindowHandle`].
    ///
    /// # Safety
    /// Original type [`raw_window_handle::AppKitWindowHandle`] is marked as `non_exaustive`,
    /// and therefore it is impossible to convert to it completely safely.
    pub unsafe fn into(self) -> raw_window_handle::OrbitalWindowHandle {
        raw_window_handle::OrbitalWindowHandle::new(self.window)
    }
}
