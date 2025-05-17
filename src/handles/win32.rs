use std::num::NonZero;

/// raw_window_handle_ffi:
///         This type is ABI-stable and FFI-compatible analogue for [`raw_window_handle::Win32WindowHandle`].
///         Can be converted to and from the referenced type.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Win32WindowHandle {
    /// A Win32 `HWND` handle.
    pub hwnd: NonZero<isize>,
    /// The `GWLP_HINSTANCE` associated with this type's `HWND`.
    pub hinstance: Option<NonZero<isize>>,
}

unsafe impl Send for Win32WindowHandle {}
unsafe impl Sync for Win32WindowHandle {}

impl From<raw_window_handle::Win32WindowHandle> for Win32WindowHandle {
    fn from(value: raw_window_handle::Win32WindowHandle) -> Self {
        Self {
            hwnd: value.hwnd,
            hinstance: value.hinstance,
        }
    }
}

impl Win32WindowHandle {
    /// Converts FFI type back to the original [`raw_window_handle::AppKitWindowHandle`].
    ///
    /// # Safety
    /// Original type [`raw_window_handle::AppKitWindowHandle`] is marked as `non_exaustive`,
    /// and therefore it is impossible to convert to it completely safely.
    pub unsafe fn into(self) -> raw_window_handle::Win32WindowHandle {
        let mut result = raw_window_handle::Win32WindowHandle::new(self.hwnd);
        result.hinstance = self.hinstance;
        result
    }
}
