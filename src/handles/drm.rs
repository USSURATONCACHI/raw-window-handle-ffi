/// raw_window_handle_ffi:
///         This type is ABI-stable and FFI-compatible analogue for [`raw_window_handle::DrmWindowHandle`].
///         Can be converted to and from the referenced type.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DrmWindowHandle {
    /// The primary drm plane handle.
    pub plane: u32,
}

unsafe impl Send for DrmWindowHandle {}
unsafe impl Sync for DrmWindowHandle {}

impl From<raw_window_handle::DrmWindowHandle> for DrmWindowHandle {
    fn from(value: raw_window_handle::DrmWindowHandle) -> Self {
        Self { plane: value.plane }
    }
}

impl DrmWindowHandle {
    /// Converts FFI type back to the original [`raw_window_handle::DrmWindowHandle`].
    /// 
    /// # Safety
    /// Original type [`raw_window_handle::DrmWindowHandle`] is marked as `non_exaustive`,
    /// and therefore it is impossible to convert to it completely safely.
    pub unsafe fn into(self) -> raw_window_handle::DrmWindowHandle {
        raw_window_handle::DrmWindowHandle::new(self.plane)
    }
}
