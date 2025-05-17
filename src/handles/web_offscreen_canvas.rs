use std::{ffi::c_void, ptr::NonNull};

/// raw_window_handle_ffi:
///         This type is ABI-stable and FFI-compatible analogue for [`raw_window_handle::WebOffscreenCanvasWindowHandle`].
///         Can be converted to and from the referenced type.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WebOffscreenCanvasWindowHandle {
    /// A pointer to the [`JsValue`] of an [`OffscreenCanvas`].
    ///
    /// Note: This uses [`c_void`] to avoid depending on `wasm-bindgen`
    /// directly.
    ///
    /// [`JsValue`]: https://docs.rs/wasm-bindgen/latest/wasm_bindgen/struct.JsValue.html
    /// [`OffscreenCanvas`]: https://docs.rs/web-sys/latest/web_sys/struct.OffscreenCanvas.html
    //
    // SAFETY: See WebCanvasWindowHandle.
    pub obj: NonNull<c_void>,
}

unsafe impl Send for WebOffscreenCanvasWindowHandle {}
unsafe impl Sync for WebOffscreenCanvasWindowHandle {}

impl From<raw_window_handle::WebOffscreenCanvasWindowHandle> for WebOffscreenCanvasWindowHandle {
    fn from(value: raw_window_handle::WebOffscreenCanvasWindowHandle) -> Self {
        Self { obj: value.obj }
    }
}

impl WebOffscreenCanvasWindowHandle {
    /// Converts FFI type back to the original [`raw_window_handle::AppKitWindowHandle`].
    ///
    /// # Safety
    /// Original type [`raw_window_handle::AppKitWindowHandle`] is marked as `non_exaustive`,
    /// and therefore it is impossible to convert to it completely safely.
    pub unsafe fn into(self) -> raw_window_handle::WebOffscreenCanvasWindowHandle {
        raw_window_handle::WebOffscreenCanvasWindowHandle::new(self.obj)
    }
}
