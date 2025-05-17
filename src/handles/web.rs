/// raw_window_handle_ffi:
///         This type is ABI-stable and FFI-compatible analogue for [`raw_window_handle::WebWindowHandle`].
///         Can be converted to and from the referenced type.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WebWindowHandle {
    /// An ID value inserted into the [data attributes] of the canvas element as '`raw-handle`'.
    ///
    /// When accessing from JS, the attribute will automatically be called `rawHandle`.
    ///
    /// Each canvas created by the windowing system should be assigned their own unique ID.
    ///
    /// [data attributes]: https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/data-*
    pub id: u32,
}

unsafe impl Send for WebWindowHandle {}
unsafe impl Sync for WebWindowHandle {}

impl From<raw_window_handle::WebWindowHandle> for WebWindowHandle {
    fn from(value: raw_window_handle::WebWindowHandle) -> Self {
        Self { id: value.id }
    }
}

impl WebWindowHandle {
    /// Converts FFI type back to the original [`raw_window_handle::AppKitWindowHandle`].
    /// 
    /// # Safety
    /// Original type [`raw_window_handle::AppKitWindowHandle`] is marked as `non_exaustive`,
    /// and therefore it is impossible to convert to it completely safely.
    pub unsafe fn into(self) -> raw_window_handle::WebWindowHandle {
        raw_window_handle::WebWindowHandle::new(self.id)
    }
}
