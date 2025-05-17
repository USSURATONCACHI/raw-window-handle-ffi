use std::num::NonZeroU32;

/// raw_window_handle_ffi:
///         This type is ABI-stable and FFI-compatible analogue for [`raw_window_handle::XcbWindowHandle`].
///         Can be converted to and from the referenced type.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct XcbWindowHandle {
    /// An X11 `xcb_window_t`.
    pub window: NonZeroU32, // Based on xproto.h
    /// An X11 `xcb_visualid_t`.
    pub visual_id: Option<NonZeroU32>,
}

unsafe impl Send for XcbWindowHandle {}
unsafe impl Sync for XcbWindowHandle {}

impl From<raw_window_handle::XcbWindowHandle> for XcbWindowHandle {
    fn from(value: raw_window_handle::XcbWindowHandle) -> Self {
        Self {
            window: value.window,
            visual_id: value.visual_id,
        }
    }
}

impl XcbWindowHandle {
    /// Converts FFI type back to the original [`raw_window_handle::AppKitWindowHandle`].
    ///
    /// # Safety
    /// Original type [`raw_window_handle::AppKitWindowHandle`] is marked as `non_exaustive`,
    /// and therefore it is impossible to convert to it completely safely.
    pub unsafe fn into(self) -> raw_window_handle::XcbWindowHandle {
        let mut result = raw_window_handle::XcbWindowHandle::new(self.window);
        result.visual_id = self.visual_id;
        result
    }
}
