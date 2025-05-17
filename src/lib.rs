mod handles;

pub use handles::*;
use thiserror::Error;

//Debug,  PartialEq, Eq, Hash

#[repr(C)]
#[derive(Clone, Copy)]
pub struct RawWindowHandle {
    pub kind: RawWindowHandleKind,
    pub data: RawWindowHandleData,
}

#[repr(u8)]
#[derive(Debug, Error)]
pub enum Error {
    #[error("unknown window handle variant was passed")]
    UnknownUnexaustiveVariant,
}

impl TryFrom<raw_window_handle::RawWindowHandle> for RawWindowHandle {
    type Error = Error;

    fn try_from(value: raw_window_handle::RawWindowHandle) -> Result<Self, Self::Error> {
        match value {
            raw_window_handle::RawWindowHandle::UiKit(ui_kit_window_handle) => Ok(Self {
                kind: RawWindowHandleKind::UiKitWindowHandle,
                data: RawWindowHandleData {
                    ui_kit: UiKitWindowHandle::from(ui_kit_window_handle),
                },
            }),
            raw_window_handle::RawWindowHandle::AppKit(app_kit_window_handle) => Ok(Self {
                kind: RawWindowHandleKind::AppKitWindowHandle,
                data: RawWindowHandleData {
                    app_kit: AppKitWindowHandle::from(app_kit_window_handle),
                },
            }),
            raw_window_handle::RawWindowHandle::Orbital(orbital_window_handle) => Ok(Self {
                kind: RawWindowHandleKind::OrbitalWindowHandle,
                data: RawWindowHandleData {
                    orbital: OrbitalWindowHandle::from(orbital_window_handle),
                },
            }),
            raw_window_handle::RawWindowHandle::OhosNdk(ohos_ndk_window_handle) => Ok(Self {
                kind: RawWindowHandleKind::OhosNdkWindowHandle,
                data: RawWindowHandleData {
                    ohos_ndk: OhosNdkWindowHandle::from(ohos_ndk_window_handle),
                },
            }),
            raw_window_handle::RawWindowHandle::Xlib(xlib_window_handle) => Ok(Self {
                kind: RawWindowHandleKind::XlibWindowHandle,
                data: RawWindowHandleData {
                    xlib: XlibWindowHandle::from(xlib_window_handle),
                },
            }),
            raw_window_handle::RawWindowHandle::Xcb(xcb_window_handle) => Ok(Self {
                kind: RawWindowHandleKind::XcbWindowHandle,
                data: RawWindowHandleData {
                    xcb: XcbWindowHandle::from(xcb_window_handle),
                },
            }),
            raw_window_handle::RawWindowHandle::Wayland(wayland_window_handle) => Ok(Self {
                kind: RawWindowHandleKind::WaylandWindowHandle,
                data: RawWindowHandleData {
                    wayland: WaylandWindowHandle::from(wayland_window_handle),
                },
            }),
            raw_window_handle::RawWindowHandle::Drm(drm_window_handle) => Ok(Self {
                kind: RawWindowHandleKind::DrmWindowHandle,
                data: RawWindowHandleData {
                    drm: DrmWindowHandle::from(drm_window_handle),
                },
            }),
            raw_window_handle::RawWindowHandle::Gbm(gbm_window_handle) => Ok(Self {
                kind: RawWindowHandleKind::GbmWindowHandle,
                data: RawWindowHandleData {
                    gbm: GbmWindowHandle::from(gbm_window_handle),
                },
            }),
            raw_window_handle::RawWindowHandle::Win32(win32_window_handle) => Ok(Self {
                kind: RawWindowHandleKind::Win32WindowHandle,
                data: RawWindowHandleData {
                    win32: Win32WindowHandle::from(win32_window_handle),
                },
            }),
            raw_window_handle::RawWindowHandle::WinRt(win_rt_window_handle) => Ok(Self {
                kind: RawWindowHandleKind::WinRtWindowHandle,
                data: RawWindowHandleData {
                    win_rt: WinRtWindowHandle::from(win_rt_window_handle),
                },
            }),
            raw_window_handle::RawWindowHandle::Web(web_window_handle) => Ok(Self {
                kind: RawWindowHandleKind::WebWindowHandle,
                data: RawWindowHandleData {
                    web: WebWindowHandle::from(web_window_handle),
                },
            }),
            raw_window_handle::RawWindowHandle::WebCanvas(web_canvas_window_handle) => Ok(Self {
                kind: RawWindowHandleKind::WebCanvasWindowHandle,
                data: RawWindowHandleData {
                    web_canvas: WebCanvasWindowHandle::from(web_canvas_window_handle),
                },
            }),
            raw_window_handle::RawWindowHandle::WebOffscreenCanvas(
                web_offscreen_canvas_window_handle,
            ) => Ok(Self {
                kind: RawWindowHandleKind::WebOffscreenCanvasWindowHandle,
                data: RawWindowHandleData {
                    web_offscreen_canvas: WebOffscreenCanvasWindowHandle::from(
                        web_offscreen_canvas_window_handle,
                    ),
                },
            }),
            raw_window_handle::RawWindowHandle::AndroidNdk(android_ndk_window_handle) => Ok(Self {
                kind: RawWindowHandleKind::AndroidNdkWindowHandle,
                data: RawWindowHandleData {
                    android_ndk: AndroidNdkWindowHandle::from(android_ndk_window_handle),
                },
            }),
            raw_window_handle::RawWindowHandle::Haiku(haiku_window_handle) => Ok(Self {
                kind: RawWindowHandleKind::HaikuWindowHandle,
                data: RawWindowHandleData {
                    haiku: HaikuWindowHandle::from(haiku_window_handle),
                },
            }),
            _ => Err(Error::UnknownUnexaustiveVariant),
        }
    }
}

impl RawWindowHandle {
    /// Converts FFI type back to the [`raw_window_handle::RawWindowHandle`].
    ///
    /// Unsafe because all non-FFI values are `#[non_exaustive]` and its impossible to safely convert to those.
    pub unsafe fn into_handle(self) -> raw_window_handle::RawWindowHandle {
        unsafe {
            match self.kind {
                RawWindowHandleKind::UiKitWindowHandle => {
                    raw_window_handle::RawWindowHandle::UiKit(self.data.ui_kit.into())
                }
                RawWindowHandleKind::AppKitWindowHandle => {
                    raw_window_handle::RawWindowHandle::AppKit(self.data.app_kit.into())
                }
                RawWindowHandleKind::OrbitalWindowHandle => {
                    raw_window_handle::RawWindowHandle::Orbital(self.data.orbital.into())
                }
                RawWindowHandleKind::OhosNdkWindowHandle => {
                    raw_window_handle::RawWindowHandle::OhosNdk(self.data.ohos_ndk.into())
                }
                RawWindowHandleKind::XlibWindowHandle => {
                    raw_window_handle::RawWindowHandle::Xlib(self.data.xlib.into())
                }
                RawWindowHandleKind::XcbWindowHandle => {
                    raw_window_handle::RawWindowHandle::Xcb(self.data.xcb.into())
                }
                RawWindowHandleKind::WaylandWindowHandle => {
                    raw_window_handle::RawWindowHandle::Wayland(self.data.wayland.into())
                }
                RawWindowHandleKind::DrmWindowHandle => {
                    raw_window_handle::RawWindowHandle::Drm(self.data.drm.into())
                }
                RawWindowHandleKind::GbmWindowHandle => {
                    raw_window_handle::RawWindowHandle::Gbm(self.data.gbm.into())
                }
                RawWindowHandleKind::Win32WindowHandle => {
                    raw_window_handle::RawWindowHandle::Win32(self.data.win32.into())
                }
                RawWindowHandleKind::WinRtWindowHandle => {
                    raw_window_handle::RawWindowHandle::WinRt(self.data.win_rt.into())
                }
                RawWindowHandleKind::WebWindowHandle => {
                    raw_window_handle::RawWindowHandle::Web(self.data.web.into())
                }
                RawWindowHandleKind::WebCanvasWindowHandle => {
                    raw_window_handle::RawWindowHandle::WebCanvas(self.data.web_canvas.into())
                }
                RawWindowHandleKind::WebOffscreenCanvasWindowHandle => {
                    raw_window_handle::RawWindowHandle::WebOffscreenCanvas(
                        self.data.web_offscreen_canvas.into(),
                    )
                }
                RawWindowHandleKind::AndroidNdkWindowHandle => {
                    raw_window_handle::RawWindowHandle::AndroidNdk(self.data.android_ndk.into())
                }
                RawWindowHandleKind::HaikuWindowHandle => {
                    raw_window_handle::RawWindowHandle::Haiku(self.data.haiku.into())
                }
            }
        }
    }
}

impl std::fmt::Debug for RawWindowHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe {
            let handle: raw_window_handle::RawWindowHandle = self.into_handle();
            handle.fmt(f)
        }
    }
}

impl PartialEq for RawWindowHandle {
    fn eq(&self, other: &Self) -> bool {
        if self.kind != other.kind {
            return false;
        }

        unsafe {
            match self.kind {
                RawWindowHandleKind::UiKitWindowHandle => self.data.ui_kit == other.data.ui_kit,
                RawWindowHandleKind::AppKitWindowHandle => self.data.app_kit == other.data.app_kit,
                RawWindowHandleKind::OrbitalWindowHandle => self.data.orbital == other.data.orbital,
                RawWindowHandleKind::OhosNdkWindowHandle => {
                    self.data.ohos_ndk == other.data.ohos_ndk
                }
                RawWindowHandleKind::XlibWindowHandle => self.data.xlib == other.data.xlib,
                RawWindowHandleKind::XcbWindowHandle => self.data.xcb == other.data.xcb,
                RawWindowHandleKind::WaylandWindowHandle => self.data.wayland == other.data.wayland,
                RawWindowHandleKind::DrmWindowHandle => self.data.drm == other.data.drm,
                RawWindowHandleKind::GbmWindowHandle => self.data.gbm == other.data.gbm,
                RawWindowHandleKind::Win32WindowHandle => self.data.win32 == other.data.win32,
                RawWindowHandleKind::WinRtWindowHandle => self.data.win_rt == other.data.win_rt,
                RawWindowHandleKind::WebWindowHandle => self.data.web == other.data.web,
                RawWindowHandleKind::WebCanvasWindowHandle => {
                    self.data.web_canvas == other.data.web_canvas
                }
                RawWindowHandleKind::WebOffscreenCanvasWindowHandle => {
                    self.data.web_offscreen_canvas == other.data.web_offscreen_canvas
                }
                RawWindowHandleKind::AndroidNdkWindowHandle => {
                    self.data.android_ndk == other.data.android_ndk
                }
                RawWindowHandleKind::HaikuWindowHandle => self.data.haiku == other.data.haiku,
            }
        }
    }
}

impl Eq for RawWindowHandle {}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum RawWindowHandleKind {
    /// A raw window handle for UIKit (Apple's non-macOS windowing library).
    ///
    /// ## Availability Hints
    /// This variant is likely to be used on iOS, tvOS, (in theory) watchOS, and
    /// Mac Catalyst (`$arch-apple-ios-macabi` targets, which can notably use
    /// UIKit *or* AppKit), as these are the targets that (currently) support
    /// UIKit.
    UiKitWindowHandle,
    /// A raw window handle for AppKit.
    ///
    /// ## Availability Hints
    /// This variant is likely to be used on macOS, although Mac Catalyst
    /// (`$arch-apple-ios-macabi` targets, which can notably use UIKit *or*
    /// AppKit) can also use it despite being `target_os = "ios"`.
    AppKitWindowHandle,
    /// A raw window handle for the Redox operating system.
    ///
    /// ## Availability Hints
    /// This variant is used by the Orbital Windowing System in the Redox
    /// operating system.
    OrbitalWindowHandle,
    /// A raw window handle for the OpenHarmony OS NDK
    ///
    /// ## Availability Hints
    /// This variant is used on OpenHarmony OS (`target_env = "ohos"`).
    OhosNdkWindowHandle,
    /// A raw window handle for Xlib.
    ///
    /// ## Availability Hints
    /// This variant is likely to show up anywhere someone manages to get X11
    /// working that Xlib can be built for, which is to say, most (but not all)
    /// Unix systems.
    XlibWindowHandle,
    /// A raw window handle for Xcb.
    ///
    /// ## Availability Hints
    /// This variant is likely to show up anywhere someone manages to get X11
    /// working that XCB can be built for, which is to say, most (but not all)
    /// Unix systems.
    XcbWindowHandle,
    /// A raw window handle for Wayland.
    ///
    /// ## Availability Hints
    /// This variant should be expected anywhere Wayland works, which is
    /// currently some subset of unix systems.
    WaylandWindowHandle,
    /// A raw window handle for the Linux Kernel Mode Set/Direct Rendering Manager
    ///
    /// ## Availability Hints
    /// This variant is used on Linux when neither X nor Wayland are available
    DrmWindowHandle,
    /// A raw window handle for the Linux Generic Buffer Manager.
    ///
    /// ## Availability Hints
    /// This variant is present regardless of windowing backend and likely to be used with
    /// EGL_MESA_platform_gbm or EGL_KHR_platform_gbm.
    GbmWindowHandle,
    /// A raw window handle for Win32.
    ///
    /// ## Availability Hints
    /// This variant is used on Windows systems.
    Win32WindowHandle,
    /// A raw window handle for WinRT.
    ///
    /// ## Availability Hints
    /// This variant is used on Windows systems.
    WinRtWindowHandle,
    /// A raw window handle for the Web.
    ///
    /// ## Availability Hints
    /// This variant is used on Wasm or asm.js targets when targeting the Web/HTML5.
    WebWindowHandle,
    /// A raw window handle for a Web canvas registered via [`wasm-bindgen`].
    ///
    /// ## Availability Hints
    /// This variant is used on Wasm or asm.js targets when targeting the Web/HTML5.
    ///
    /// [`wasm-bindgen`]: https://crates.io/crates/wasm-bindgen
    WebCanvasWindowHandle,
    /// A raw window handle for a Web offscreen canvas registered via [`wasm-bindgen`].
    ///
    /// ## Availability Hints
    /// This variant is used on Wasm or asm.js targets when targeting the Web/HTML5.
    ///
    /// [`wasm-bindgen`]: https://crates.io/crates/wasm-bindgen
    WebOffscreenCanvasWindowHandle,
    /// A raw window handle for Android NDK.
    ///
    /// ## Availability Hints
    /// This variant is used on Android targets.
    AndroidNdkWindowHandle,
    /// A raw window handle for Haiku.
    ///
    /// ## Availability Hints
    /// This variant is used on HaikuOS.
    HaikuWindowHandle,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union RawWindowHandleData {
    /// A raw window handle for UIKit (Apple's non-macOS windowing library).
    ///
    /// ## Availability Hints
    /// This variant is likely to be used on iOS, tvOS, (in theory) watchOS, and
    /// Mac Catalyst (`$arch-apple-ios-macabi` targets, which can notably use
    /// UIKit *or* AppKit), as these are the targets that (currently) support
    /// UIKit.
    ui_kit: UiKitWindowHandle,
    /// A raw window handle for AppKit.
    ///
    /// ## Availability Hints
    /// This variant is likely to be used on macOS, although Mac Catalyst
    /// (`$arch-apple-ios-macabi` targets, which can notably use UIKit *or*
    /// AppKit) can also use it despite being `target_os = "ios"`.
    app_kit: AppKitWindowHandle,
    /// A raw window handle for the Redox operating system.
    ///
    /// ## Availability Hints
    /// This variant is used by the Orbital Windowing System in the Redox
    /// operating system.
    orbital: OrbitalWindowHandle,
    /// A raw window handle for the OpenHarmony OS NDK
    ///
    /// ## Availability Hints
    /// This variant is used on OpenHarmony OS (`target_env = "ohos"`).
    ohos_ndk: OhosNdkWindowHandle,
    /// A raw window handle for Xlib.
    ///
    /// ## Availability Hints
    /// This variant is likely to show up anywhere someone manages to get X11
    /// working that Xlib can be built for, which is to say, most (but not all)
    /// Unix systems.
    xlib: XlibWindowHandle,
    /// A raw window handle for Xcb.
    ///
    /// ## Availability Hints
    /// This variant is likely to show up anywhere someone manages to get X11
    /// working that XCB can be built for, which is to say, most (but not all)
    /// Unix systems.
    xcb: XcbWindowHandle,
    /// A raw window handle for Wayland.
    ///
    /// ## Availability Hints
    /// This variant should be expected anywhere Wayland works, which is
    /// currently some subset of unix systems.
    wayland: WaylandWindowHandle,
    /// A raw window handle for the Linux Kernel Mode Set/Direct Rendering Manager
    ///
    /// ## Availability Hints
    /// This variant is used on Linux when neither X nor Wayland are available
    drm: DrmWindowHandle,
    /// A raw window handle for the Linux Generic Buffer Manager.
    ///
    /// ## Availability Hints
    /// This variant is present regardless of windowing backend and likely to be used with
    /// EGL_MESA_platform_gbm or EGL_KHR_platform_gbm.
    gbm: GbmWindowHandle,
    /// A raw window handle for Win32.
    ///
    /// ## Availability Hints
    /// This variant is used on Windows systems.
    win32: Win32WindowHandle,
    /// A raw window handle for WinRT.
    ///
    /// ## Availability Hints
    /// This variant is used on Windows systems.
    win_rt: WinRtWindowHandle,
    /// A raw window handle for the Web.
    ///
    /// ## Availability Hints
    /// This variant is used on Wasm or asm.js targets when targeting the Web/HTML5.
    web: WebWindowHandle,
    /// A raw window handle for a Web canvas registered via [`wasm-bindgen`].
    ///
    /// ## Availability Hints
    /// This variant is used on Wasm or asm.js targets when targeting the Web/HTML5.
    ///
    /// [`wasm-bindgen`]: https://crates.io/crates/wasm-bindgen
    web_canvas: WebCanvasWindowHandle,
    /// A raw window handle for a Web offscreen canvas registered via [`wasm-bindgen`].
    ///
    /// ## Availability Hints
    /// This variant is used on Wasm or asm.js targets when targeting the Web/HTML5.
    ///
    /// [`wasm-bindgen`]: https://crates.io/crates/wasm-bindgen
    web_offscreen_canvas: WebOffscreenCanvasWindowHandle,
    /// A raw window handle for Android NDK.
    ///
    /// ## Availability Hints
    /// This variant is used on Android targets.
    android_ndk: AndroidNdkWindowHandle,
    /// A raw window handle for Haiku.
    ///
    /// ## Availability Hints
    /// This variant is used on HaikuOS.
    haiku: HaikuWindowHandle,
}
