/* Auto-generated header by cbindgen. Do not edit directly. */

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

enum rwhf_Error {
  RWHF_ERROR_UNKNOWN_UNEXAUSTIVE_VARIANT,
};
typedef uint8_t rwhf_Error;

enum rwhf_RawWindowHandleKind {
  /**
   * A raw window handle for UIKit (Apple's non-macOS windowing library).
   *
   * ## Availability Hints
   * This variant is likely to be used on iOS, tvOS, (in theory) watchOS, and
   * Mac Catalyst (`$arch-apple-ios-macabi` targets, which can notably use
   * UIKit *or* AppKit), as these are the targets that (currently) support
   * UIKit.
   */
  RWHF_RAW_WINDOW_HANDLE_KIND_UI_KIT_WINDOW_HANDLE,
  /**
   * A raw window handle for AppKit.
   *
   * ## Availability Hints
   * This variant is likely to be used on macOS, although Mac Catalyst
   * (`$arch-apple-ios-macabi` targets, which can notably use UIKit *or*
   * AppKit) can also use it despite being `target_os = "ios"`.
   */
  RWHF_RAW_WINDOW_HANDLE_KIND_APP_KIT_WINDOW_HANDLE,
  /**
   * A raw window handle for the Redox operating system.
   *
   * ## Availability Hints
   * This variant is used by the Orbital Windowing System in the Redox
   * operating system.
   */
  RWHF_RAW_WINDOW_HANDLE_KIND_ORBITAL_WINDOW_HANDLE,
  /**
   * A raw window handle for the OpenHarmony OS NDK
   *
   * ## Availability Hints
   * This variant is used on OpenHarmony OS (`target_env = "ohos"`).
   */
  RWHF_RAW_WINDOW_HANDLE_KIND_OHOS_NDK_WINDOW_HANDLE,
  /**
   * A raw window handle for Xlib.
   *
   * ## Availability Hints
   * This variant is likely to show up anywhere someone manages to get X11
   * working that Xlib can be built for, which is to say, most (but not all)
   * Unix systems.
   */
  RWHF_RAW_WINDOW_HANDLE_KIND_XLIB_WINDOW_HANDLE,
  /**
   * A raw window handle for Xcb.
   *
   * ## Availability Hints
   * This variant is likely to show up anywhere someone manages to get X11
   * working that XCB can be built for, which is to say, most (but not all)
   * Unix systems.
   */
  RWHF_RAW_WINDOW_HANDLE_KIND_XCB_WINDOW_HANDLE,
  /**
   * A raw window handle for Wayland.
   *
   * ## Availability Hints
   * This variant should be expected anywhere Wayland works, which is
   * currently some subset of unix systems.
   */
  RWHF_RAW_WINDOW_HANDLE_KIND_WAYLAND_WINDOW_HANDLE,
  /**
   * A raw window handle for the Linux Kernel Mode Set/Direct Rendering Manager
   *
   * ## Availability Hints
   * This variant is used on Linux when neither X nor Wayland are available
   */
  RWHF_RAW_WINDOW_HANDLE_KIND_DRM_WINDOW_HANDLE,
  /**
   * A raw window handle for the Linux Generic Buffer Manager.
   *
   * ## Availability Hints
   * This variant is present regardless of windowing backend and likely to be used with
   * EGL_MESA_platform_gbm or EGL_KHR_platform_gbm.
   */
  RWHF_RAW_WINDOW_HANDLE_KIND_GBM_WINDOW_HANDLE,
  /**
   * A raw window handle for Win32.
   *
   * ## Availability Hints
   * This variant is used on Windows systems.
   */
  RWHF_RAW_WINDOW_HANDLE_KIND_WIN32_WINDOW_HANDLE,
  /**
   * A raw window handle for WinRT.
   *
   * ## Availability Hints
   * This variant is used on Windows systems.
   */
  RWHF_RAW_WINDOW_HANDLE_KIND_WIN_RT_WINDOW_HANDLE,
  /**
   * A raw window handle for the Web.
   *
   * ## Availability Hints
   * This variant is used on Wasm or asm.js targets when targeting the Web/HTML5.
   */
  RWHF_RAW_WINDOW_HANDLE_KIND_WEB_WINDOW_HANDLE,
  /**
   * A raw window handle for a Web canvas registered via [`wasm-bindgen`].
   *
   * ## Availability Hints
   * This variant is used on Wasm or asm.js targets when targeting the Web/HTML5.
   *
   * [`wasm-bindgen`]: https://crates.io/crates/wasm-bindgen
   */
  RWHF_RAW_WINDOW_HANDLE_KIND_WEB_CANVAS_WINDOW_HANDLE,
  /**
   * A raw window handle for a Web offscreen canvas registered via [`wasm-bindgen`].
   *
   * ## Availability Hints
   * This variant is used on Wasm or asm.js targets when targeting the Web/HTML5.
   *
   * [`wasm-bindgen`]: https://crates.io/crates/wasm-bindgen
   */
  RWHF_RAW_WINDOW_HANDLE_KIND_WEB_OFFSCREEN_CANVAS_WINDOW_HANDLE,
  /**
   * A raw window handle for Android NDK.
   *
   * ## Availability Hints
   * This variant is used on Android targets.
   */
  RWHF_RAW_WINDOW_HANDLE_KIND_ANDROID_NDK_WINDOW_HANDLE,
  /**
   * A raw window handle for Haiku.
   *
   * ## Availability Hints
   * This variant is used on HaikuOS.
   */
  RWHF_RAW_WINDOW_HANDLE_KIND_HAIKU_WINDOW_HANDLE,
};
typedef uint8_t rwhf_RawWindowHandleKind;

/**
 * raw_window_handle_ffi:
 *         This type is ABI-stable and FFI-compatible analogue for [`raw_window_handle::UiKitWindowHandle`].
 *         Can be converted to and from the referenced type.
 */
typedef struct rwhf_UiKitWindowHandle {
  /**
   * A pointer to an `UIView` object.
   */
  void *ui_view;
  /**
   * A pointer to an `UIViewController` object, if the view has one.
   */
  void *ui_view_controller;
} rwhf_UiKitWindowHandle;

/**
 * raw_window_handle_ffi:
 *         This type is ABI-stable and FFI-compatible analogue for [`raw_window_handle::WinRtWindowHandle`].
 *         Can be converted to and from the referenced type.
 */
typedef struct rwhf_WinRtWindowHandle {
  /**
   * A WinRT `CoreWindow` handle.
   */
  void *core_window;
} rwhf_WinRtWindowHandle;

/**
 * raw_window_handle_ffi:
 *         This type is ABI-stable and FFI-compatible analogue for [`raw_window_handle::HaikuWindowHandle`].
 *         Can be converted to and from the referenced type.
 */
typedef struct rwhf_HaikuWindowHandle {
  /**
   * A pointer to a BWindow object
   */
  void *b_window;
  /**
   * A pointer to a BDirectWindow object that might be null
   */
  void *b_direct_window;
} rwhf_HaikuWindowHandle;

/**
 * raw_window_handle_ffi:
 *         This type is ABI-stable and FFI-compatible analogue for [`raw_window_handle::AppKitWindowHandle`].
 *         Can be converted to and from the referenced type.
 */
typedef struct rwhf_AppKitWindowHandle {
  /**
   * A pointer to an `NSView` object.
   */
  void *ns_view;
} rwhf_AppKitWindowHandle;

/**
 * raw_window_handle_ffi:
 *         This type is ABI-stable and FFI-compatible analogue for [`raw_window_handle::OrbitalWindowHandle`].
 *         Can be converted to and from the referenced type.
 */
typedef struct rwhf_OrbitalWindowHandle {
  /**
   * A pointer to an orbclient window.
   */
  void *window;
} rwhf_OrbitalWindowHandle;

/**
 * raw_window_handle_ffi:
 *         This type is ABI-stable and FFI-compatible analogue for [`raw_window_handle::OhosNdkWindowHandle`].
 *         Can be converted to and from the referenced type.
 *
 * Raw window handle for Ohos NDK.
 */
typedef struct rwhf_OhosNdkWindowHandle {
  void *native_window;
} rwhf_OhosNdkWindowHandle;

/**
 * raw_window_handle_ffi:
 *         This type is ABI-stable and FFI-compatible analogue for [`raw_window_handle::XlibWindowHandle`].
 *         Can be converted to and from the referenced type.
 */
typedef struct rwhf_XlibWindowHandle {
  /**
   * An Xlib `Window`.
   */
  unsigned long window;
  /**
   * An Xlib visual ID, or 0 if unknown.
   */
  unsigned long visual_id;
} rwhf_XlibWindowHandle;

/**
 * raw_window_handle_ffi:
 *         This type is ABI-stable and FFI-compatible analogue for [`raw_window_handle::XcbWindowHandle`].
 *         Can be converted to and from the referenced type.
 */
typedef struct rwhf_XcbWindowHandle {
  /**
   * An X11 `xcb_window_t`.
   */
  uint32_t window;
  /**
   * An X11 `xcb_visualid_t`.
   */
  uint32_t visual_id;
} rwhf_XcbWindowHandle;

/**
 * raw_window_handle_ffi:
 *         This type is ABI-stable and FFI-compatible analogue for [`raw_window_handle::WaylandWindowHandle`].
 *         Can be converted to and from the referenced type.
 */
typedef struct rwhf_WaylandWindowHandle {
  /**
   * A pointer to a `wl_surface`.
   */
  void *surface;
} rwhf_WaylandWindowHandle;

/**
 * raw_window_handle_ffi:
 *         This type is ABI-stable and FFI-compatible analogue for [`raw_window_handle::DrmWindowHandle`].
 *         Can be converted to and from the referenced type.
 */
typedef struct rwhf_DrmWindowHandle {
  /**
   * The primary drm plane handle.
   */
  uint32_t plane;
} rwhf_DrmWindowHandle;

/**
 * raw_window_handle_ffi:
 *         This type is ABI-stable and FFI-compatible analogue for [`raw_window_handle::GbmWindowHandle`].
 *         Can be converted to and from the referenced type.
 */
typedef struct rwhf_GbmWindowHandle {
  /**
   * The gbm surface.
   */
  void *gbm_surface;
} rwhf_GbmWindowHandle;

/**
 * raw_window_handle_ffi:
 *         This type is ABI-stable and FFI-compatible analogue for [`raw_window_handle::Win32WindowHandle`].
 *         Can be converted to and from the referenced type.
 */
typedef struct rwhf_Win32WindowHandle {
  /**
   * A Win32 `HWND` handle.
   */
  intptr_t hwnd;
  /**
   * The `GWLP_HINSTANCE` associated with this type's `HWND`.
   */
  intptr_t hinstance;
} rwhf_Win32WindowHandle;

/**
 * raw_window_handle_ffi:
 *         This type is ABI-stable and FFI-compatible analogue for [`raw_window_handle::WebWindowHandle`].
 *         Can be converted to and from the referenced type.
 */
typedef struct rwhf_WebWindowHandle {
  /**
   * An ID value inserted into the [data attributes] of the canvas element as '`raw-handle`'.
   *
   * When accessing from JS, the attribute will automatically be called `rawHandle`.
   *
   * Each canvas created by the windowing system should be assigned their own unique ID.
   *
   * [data attributes]: https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/data-*
   */
  uint32_t id;
} rwhf_WebWindowHandle;

/**
 * raw_window_handle_ffi:
 *         This type is ABI-stable and FFI-compatible analogue for [`raw_window_handle::WebCanvasWindowHandle`].
 *         Can be converted to and from the referenced type.
 */
typedef struct rwhf_WebCanvasWindowHandle {
  /**
   * A pointer to the [`JsValue`] of an [`HtmlCanvasElement`].
   *
   * Note: This uses [`c_void`] to avoid depending on `wasm-bindgen`
   * directly.
   *
   * [`JsValue`]: https://docs.rs/wasm-bindgen/latest/wasm_bindgen/struct.JsValue.html
   * [`HtmlCanvasElement`]: https://docs.rs/web-sys/latest/web_sys/struct.HtmlCanvasElement.html
   */
  void *obj;
} rwhf_WebCanvasWindowHandle;

/**
 * raw_window_handle_ffi:
 *         This type is ABI-stable and FFI-compatible analogue for [`raw_window_handle::WebOffscreenCanvasWindowHandle`].
 *         Can be converted to and from the referenced type.
 */
typedef struct rwhf_WebOffscreenCanvasWindowHandle {
  /**
   * A pointer to the [`JsValue`] of an [`OffscreenCanvas`].
   *
   * Note: This uses [`c_void`] to avoid depending on `wasm-bindgen`
   * directly.
   *
   * [`JsValue`]: https://docs.rs/wasm-bindgen/latest/wasm_bindgen/struct.JsValue.html
   * [`OffscreenCanvas`]: https://docs.rs/web-sys/latest/web_sys/struct.OffscreenCanvas.html
   */
  void *obj;
} rwhf_WebOffscreenCanvasWindowHandle;

/**
 * raw_window_handle_ffi:
 *         This type is ABI-stable and FFI-compatible analogue for [`raw_window_handle::AndroidNdkWindowHandle`].
 *         Can be converted to and from the referenced type.
 */
typedef struct rwhf_AndroidNdkWindowHandle {
  /**
   * A pointer to an `ANativeWindow`.
   */
  void *a_native_window;
} rwhf_AndroidNdkWindowHandle;

typedef union rwhf_RawWindowHandleData {
  /**
   * A raw window handle for UIKit (Apple's non-macOS windowing library).
   *
   * ## Availability Hints
   * This variant is likely to be used on iOS, tvOS, (in theory) watchOS, and
   * Mac Catalyst (`$arch-apple-ios-macabi` targets, which can notably use
   * UIKit *or* AppKit), as these are the targets that (currently) support
   * UIKit.
   */
  struct rwhf_UiKitWindowHandle ui_kit;
  /**
   * A raw window handle for AppKit.
   *
   * ## Availability Hints
   * This variant is likely to be used on macOS, although Mac Catalyst
   * (`$arch-apple-ios-macabi` targets, which can notably use UIKit *or*
   * AppKit) can also use it despite being `target_os = "ios"`.
   */
  struct rwhf_AppKitWindowHandle app_kit;
  /**
   * A raw window handle for the Redox operating system.
   *
   * ## Availability Hints
   * This variant is used by the Orbital Windowing System in the Redox
   * operating system.
   */
  struct rwhf_OrbitalWindowHandle orbital;
  /**
   * A raw window handle for the OpenHarmony OS NDK
   *
   * ## Availability Hints
   * This variant is used on OpenHarmony OS (`target_env = "ohos"`).
   */
  struct rwhf_OhosNdkWindowHandle ohos_ndk;
  /**
   * A raw window handle for Xlib.
   *
   * ## Availability Hints
   * This variant is likely to show up anywhere someone manages to get X11
   * working that Xlib can be built for, which is to say, most (but not all)
   * Unix systems.
   */
  struct rwhf_XlibWindowHandle xlib;
  /**
   * A raw window handle for Xcb.
   *
   * ## Availability Hints
   * This variant is likely to show up anywhere someone manages to get X11
   * working that XCB can be built for, which is to say, most (but not all)
   * Unix systems.
   */
  struct rwhf_XcbWindowHandle xcb;
  /**
   * A raw window handle for Wayland.
   *
   * ## Availability Hints
   * This variant should be expected anywhere Wayland works, which is
   * currently some subset of unix systems.
   */
  struct rwhf_WaylandWindowHandle wayland;
  /**
   * A raw window handle for the Linux Kernel Mode Set/Direct Rendering Manager
   *
   * ## Availability Hints
   * This variant is used on Linux when neither X nor Wayland are available
   */
  struct rwhf_DrmWindowHandle drm;
  /**
   * A raw window handle for the Linux Generic Buffer Manager.
   *
   * ## Availability Hints
   * This variant is present regardless of windowing backend and likely to be used with
   * EGL_MESA_platform_gbm or EGL_KHR_platform_gbm.
   */
  struct rwhf_GbmWindowHandle gbm;
  /**
   * A raw window handle for Win32.
   *
   * ## Availability Hints
   * This variant is used on Windows systems.
   */
  struct rwhf_Win32WindowHandle win32;
  /**
   * A raw window handle for WinRT.
   *
   * ## Availability Hints
   * This variant is used on Windows systems.
   */
  struct rwhf_WinRtWindowHandle win_rt;
  /**
   * A raw window handle for the Web.
   *
   * ## Availability Hints
   * This variant is used on Wasm or asm.js targets when targeting the Web/HTML5.
   */
  struct rwhf_WebWindowHandle web;
  /**
   * A raw window handle for a Web canvas registered via [`wasm-bindgen`].
   *
   * ## Availability Hints
   * This variant is used on Wasm or asm.js targets when targeting the Web/HTML5.
   *
   * [`wasm-bindgen`]: https://crates.io/crates/wasm-bindgen
   */
  struct rwhf_WebCanvasWindowHandle web_canvas;
  /**
   * A raw window handle for a Web offscreen canvas registered via [`wasm-bindgen`].
   *
   * ## Availability Hints
   * This variant is used on Wasm or asm.js targets when targeting the Web/HTML5.
   *
   * [`wasm-bindgen`]: https://crates.io/crates/wasm-bindgen
   */
  struct rwhf_WebOffscreenCanvasWindowHandle web_offscreen_canvas;
  /**
   * A raw window handle for Android NDK.
   *
   * ## Availability Hints
   * This variant is used on Android targets.
   */
  struct rwhf_AndroidNdkWindowHandle android_ndk;
  /**
   * A raw window handle for Haiku.
   *
   * ## Availability Hints
   * This variant is used on HaikuOS.
   */
  struct rwhf_HaikuWindowHandle haiku;
} rwhf_RawWindowHandleData;

typedef struct rwhf_RawWindowHandle {
  rwhf_RawWindowHandleKind kind;
  union rwhf_RawWindowHandleData data;
} rwhf_RawWindowHandle;
