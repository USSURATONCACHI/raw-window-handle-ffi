use std::{ffi::c_void, ptr::NonNull};

/// raw_window_handle_ffi:
///         This type is ABI-stable and FFI-compatible analogue for [`raw_window_handle::UiKitWindowHandle`].
///         Can be converted to and from the referenced type.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UiKitWindowHandle {
    /// A pointer to an `UIView` object.
    pub ui_view: NonNull<c_void>,
    /// A pointer to an `UIViewController` object, if the view has one.
    pub ui_view_controller: Option<NonNull<c_void>>,
}

unsafe impl Send for UiKitWindowHandle {}
unsafe impl Sync for UiKitWindowHandle {}

impl From<raw_window_handle::UiKitWindowHandle> for UiKitWindowHandle {
    fn from(value: raw_window_handle::UiKitWindowHandle) -> Self {
        Self {
            ui_view: value.ui_view,
            ui_view_controller: value.ui_view_controller,
        }
    }
}

impl UiKitWindowHandle {
    /// Converts FFI type back to the original [`raw_window_handle::AppKitWindowHandle`].
    /// 
    /// # Safety
    /// Original type [`raw_window_handle::AppKitWindowHandle`] is marked as `non_exaustive`,
    /// and therefore it is impossible to convert to it completely safely.
    pub unsafe fn into(self) -> raw_window_handle::UiKitWindowHandle {
        let mut result = raw_window_handle::UiKitWindowHandle::new(self.ui_view);
        result.ui_view_controller = self.ui_view_controller;
        result
    }
}
