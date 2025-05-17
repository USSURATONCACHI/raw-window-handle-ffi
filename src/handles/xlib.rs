use std::ffi::c_ulong;

/// raw_window_handle_ffi:
///         This type is ABI-stable and FFI-compatible analogue for [`raw_window_handle::XlibWindowHandle`].
///         Can be converted to and from the referenced type.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct XlibWindowHandle {
    /// An Xlib `Window`.
    pub window: c_ulong,
    /// An Xlib visual ID, or 0 if unknown.
    pub visual_id: c_ulong,
}

unsafe impl Send for XlibWindowHandle {}
unsafe impl Sync for XlibWindowHandle {}

impl From<raw_window_handle::XlibWindowHandle> for XlibWindowHandle {
    fn from(value: raw_window_handle::XlibWindowHandle) -> Self {
        Self {
            window: value.window,
            visual_id: value.visual_id,
        }
    }
}

impl XlibWindowHandle {
    /// Converts FFI type back to the original [`raw_window_handle::AppKitWindowHandle`].
    /// 
    /// # Safety
    /// Original type [`raw_window_handle::AppKitWindowHandle`] is marked as `non_exaustive`,
    /// and therefore it is impossible to convert to it completely safely.
    pub unsafe fn into(self) -> raw_window_handle::XlibWindowHandle {
        let mut result = raw_window_handle::XlibWindowHandle::new(self.window);
        result.visual_id = self.visual_id;
        result
    }
}
