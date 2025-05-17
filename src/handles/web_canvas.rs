use std::{ffi::c_void, ptr::NonNull};

/// raw_window_handle_ffi:
///         This type is ABI-stable and FFI-compatible analogue for [`raw_window_handle::WebCanvasWindowHandle`].
///         Can be converted to and from the referenced type.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WebCanvasWindowHandle {
    /// A pointer to the [`JsValue`] of an [`HtmlCanvasElement`].
    ///
    /// Note: This uses [`c_void`] to avoid depending on `wasm-bindgen`
    /// directly.
    ///
    /// [`JsValue`]: https://docs.rs/wasm-bindgen/latest/wasm_bindgen/struct.JsValue.html
    /// [`HtmlCanvasElement`]: https://docs.rs/web-sys/latest/web_sys/struct.HtmlCanvasElement.html
    //
    // SAFETY: Not using `JsValue` is sound because `wasm-bindgen` guarantees
    // that there's only one version of itself in any given binary, and hence
    // we can't have a type-confusion where e.g. one library used `JsValue`
    // from `v0.2` of `wasm-bindgen`, and another used `JsValue` from `v1.0`;
    // the binary will simply fail to compile!
    //
    // Reference: TODO
    pub obj: NonNull<c_void>,
}

unsafe impl Send for WebCanvasWindowHandle {}
unsafe impl Sync for WebCanvasWindowHandle {}

impl From<raw_window_handle::WebCanvasWindowHandle> for WebCanvasWindowHandle {
    fn from(value: raw_window_handle::WebCanvasWindowHandle) -> Self {
        Self { obj: value.obj }
    }
}

impl WebCanvasWindowHandle {
    /// Converts FFI type back to the original [`raw_window_handle::AppKitWindowHandle`].
    ///
    /// # Safety
    /// Original type [`raw_window_handle::AppKitWindowHandle`] is marked as `non_exaustive`,
    /// and therefore it is impossible to convert to it completely safely.
    pub unsafe fn into(self) -> raw_window_handle::WebCanvasWindowHandle {
        raw_window_handle::WebCanvasWindowHandle::new(self.obj)
    }
}
