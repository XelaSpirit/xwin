//! Window related functions of XWin
//!
//! # Context Guide
//!
//! ## Context Objects
//! A [Window] object encapsulates both a top-level window and an OpenGL or
//! OpenGL ES context. It is created with [Window::create] or
//! [WindowBuilder::create] and destroyed when either the [Window] object drops
//! or XWin is terminated.
//!
//! As the window and context are inseparably linked, the [Window] object also
//! serves as the context handle.
//!
//! ### Note
//! Vulkan does not have a context and the Vulkan instance is created via the
//! Vulkan API itself. If you will be using Vulkan to render a window, disable
//! context creation by setting the [WindowBuilder::client_api] hint
//! to [ClientApi::None].
//!
//! ## Context Creation Hints
//! There are a number of hints, specified using [WindowBuilder], related to
//! what kind of context is created. See [context related
//! hints](#context-related-hints) in the Window Guide below.
//!
//! ## Offscreen Contexts
//! XWin doesn't support creating contexts without an associated window.
//! However, contexts with hidden windows can be created with the
//! [WindowBuilder::visible] window hint.
//!
//! ```
//! # use xwin::core::{Platform, XWin};
//! # use xwin::window::WindowBuilder;
//! # XWin::init(|| {
//! let win = WindowBuilder::new()
//! 	.visible(false)
//! 	.create(1920, 1080, "title", None);
//! # });
//! ```
//!
//! The window never needs to be shown and its context can be used as a plain
//! offscreen context. Depending on the window manager, the size of a hidden
//! window's framebuffer may not be usable or modifiable, so framebuffer objects
//! are recommended for rendering with such contexts.
//!
//! You should still process events as long as you have at least one window,
//! even if none of them are visible.
//!
//! ## Windows Without Contexts
//! You can disable context creation by setting the [WindowBuilder::client_api]
//! hint to [ClientApi::None].
//!
//! Windows without contexts should not call [Window::set_current] or
//! [Window::swap_buffers]. Doing this generates [XErr::NoWindowContext].
//!
//! ## Current Context
//! Before you can make OpenGL or OpenGL ES calls, you need to have a current
//! context of the correct type. A context can only be current for a single
//! thread at a time, and a thread can only have a single context current at a
//! time.
//!
//! The context of a window is made current with [Window::set_current]. Whether
//! a window's context is current can be queried with [Window::is_current].
//!
//! The following XWin function requires a context to be present. Calling this
//! function without a current context will generate [XErr::NoCurrentContext].
//! - [Window::set_swap_interval]
//!
//! ## OpenGL and OpenGL ES Extensions
//! One of the benefits of OpenGL and OpenGL ES is their extensibility. Hardware
//! vendors may include extensions in their implementations that extend the API
//! before that functionality is included in a new version of the OpenGL or
//! OpenGL ES specification, and some extensions are never included and remain
//! as extensions until they become obsolete.
//!
//! An extension is defined by:
//! - An extension name (e.g. `GL_ARB_gl_spirv`)
//! - New OpenGL tokens (e.g. `GL_SPIR_V_BINARY_ARB`)
//! - New OpenGL functions (e.g. `glSpecializeShaderARB`)
//!
//! Note the `ARB` affix, which stands for Architecture Review Board and is used
//! for official extensions. The extension above was created by the ARB, but
//! there are many different affixes, like NV for Nvidia and AMD for, well, AMD.
//! Any group may also use the generic `EXT` affix. Lists of extensions,
//! together with their specifications, can be found at the **OpenGL Registry**
//! and **OpenGL ES Registry**.
//!
//! ## Loading Extensions With A Loader Library
//! An extension loader library is the easiest and best way to access both
//! OpenGL and OpenGL ES extensions and modern versions of the core OpenGL or
//! OpenGL ES APIs. XWin does not provide any extension loading functionality,
//! and expects the user to either implement it themselves or find another crate
//! that provides access to the OpenGL API.
//!
//! # Window Guide
//! TODO -
//! ## Context Related Hints

mod builder;
mod callback;

use std::{
	cell::RefCell,
	sync::mpsc::channel,
};

pub use builder::*;
pub use callback::*;

use crate::{
	bind::{
		glfwSetWindowShouldClose,
		glfwWindowShouldClose,
		GLFWwindow,
		GLFW_DONT_CARE,
		GLFW_FALSE,
		GLFW_TRUE,
	},
	core::{
		exec::XWinMessage,
		image::Image,
		ContentScale,
		ScreenCoordinates,
		XWin,
	},
	err::XErr,
	monitor::Monitor,
};

thread_local! {
	static WINDOW: RefCell<Option<Window>> = RefCell::new(None);
}

pub struct Window(*mut GLFWwindow);

impl Window
{
	/// This function creates a window and its associated OpenGL or OpenGL ES
	/// context. Options controlling how the window and its context should be
	/// created are specified using other functions in [WindowBuilder].
	///
	/// Successful creation does not change which context is current. Before you
	/// can use the newly created context, you need to make it current.
	///
	/// The created window, framebuffer, and context may differ from what you
	/// requested, as not all parameters and hints are hard constraints. This
	/// includes the size of the window, especially for full screen windows. To
	/// query the actual attributes of the created window, framebuffer, and
	/// context, see [Window].
	///
	/// To create a full screen window, you need to specify the monitor the
	/// [Window] will cover. If no monitor is specified, the window will be
	/// windowed mode. Unless you have a way for the user to choose a specified
	/// monitor, it is recommended that you pick the [primary
	/// monitor](Monitor::primary). For more information on how to query
	/// connected monitors, see [retrieving
	/// monitors](crate::monitor#retrieving-monitors).
	///
	/// For full screen windows, the specified size becomes the resolution of
	/// the window's *desired video mode*. As long as a full screen window is
	/// not iconified, the supported video mode most closely matching the
	/// desired video mode is set for the specified monitor. For more
	/// information about full screen windows, including the creation of so
	/// called *windows full screen* or *borderless full screen* windows, see
	/// [Windowed Full Screen
	/// Windows](crate::window#windowed-full-screen-windows).
	///
	/// By default, newly created windows use the placement recommended by the
	/// window system. To create the window at a specific position, use the
	/// [WindowBuilder::position] function before creation.
	///
	/// As long as at least one full screen window is not iconified, the
	/// screensaver is prohibited from starting.
	///
	/// Window systems put limits on window sizes. Very large or very small
	/// window dimensions may be overridden by the window system on creation.
	/// Check the actual size after creation.
	///
	/// The swap interval is not set during window creation and the initial
	/// value may vary depending on driver settings and defaults.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized], [XErr::InvalidEnum],
	/// [XErr::InvalidValue], [XErr::ApiUnavailable],
	/// [XErr::VersionUnavailable], [XErr::FormatUnavailable],
	/// [XErr::NoWindowContext], [XErr::Platform].
	///
	/// # Remarks
	/// **Windows**.
	/// - Window creation will fail if the Microsoft GDI software OpenGL
	///   implementation is the only one available.
	/// - If the executable has an icon resource named `GLFW_ICON`, it will be
	///   set as the initial icon for the window. If no such icon is present,
	///   the `IDI_APPLICATION` icon will be used instead. To set a different
	///   icon, see [Window::set_icon].
	///
	/// **MacOS**.
	/// - The GLFW window has no icon, as it is not a document window, but the
	///   dock icon will be the same as the application bundle's ion. For more
	///   information on bundles, see the **Bundle Programming Guide** in the
	///   Mac Developer Library.
	/// - On OS X 10.10 and later the window frame will not be rendered at full
	///   resolution on Retina displays unless the
	///   [WindowBuilder::scale_framebuffer] hint is `true` and the
	///   `NSHighResolutionCapable` key is enabled in the application bundle's
	///   `Info.plist`. For more information, see **High Resolution Guidelines
	///   for OS X** in the Mac Developer Library.
	/// - When activating frame autosaving with
	///   [WindowBuilder::cocoa_frame_name], the specified window size and
	///   position may be overridden by previously saved values.
	///
	/// **Wayland**.
	/// - XWin uses **libdecor** where available to create its window
	///   decorations. This in turn uses server-side XDG decorations where
	///   available and provides high quality client-side decorations on
	///   compositors like GNOME. If both XDG decorations and libdecor are
	///   unavailable, XWin falls back to a very simple set of window
	///   decorations that only support moving, resizing and the window
	///   manager's right-click menu.
	///
	/// **X11**.
	/// - Some window managers will not respect the placement of initially
	///   hidden windows.
	/// - Due to the asynchronous nature of X11, it may take a moment for a
	///   window to reach its requested state. This means you may not be able to
	///   query the final size, position or other attributes directly after
	///   window creation.
	/// - The class part of the `WM_CLASS` window property will by default be
	///   set to the window title passed to this function. The instance part
	///   will use the contents of the `RESOURCE_NAME` environment variable, if
	///   present and not empty, or fall back to the window title. Set the
	///   [WindowBuilder::x11_class_name] window hint to override this.
	pub fn create(
		width: i32,
		height: i32,
		title: &str,
		monitor: Option<Monitor>,
	) -> Result<Self, XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?
			.post_rcv(
				XWinMessage::CreateWindow {
					width,
					height,
					title: String::from(title),
					monitor,
					builder: None,
					tx,
				},
				rx,
			)?
			.map(|win| Self::from_glfw(win))
	}

	/// Returns the value of the close flag of the window.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized].
	pub fn should_close(&self) -> Result<bool, XErr>
	{
		let close = unsafe { glfwWindowShouldClose(self.0) == GLFW_TRUE as i32 };
		XErr::result(|| close)
	}

	/// Sets the value of the close flag of the window. This can be used to
	/// override the user's attempt to close the window, or to signal that it
	/// should be closed.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized].
	pub fn set_should_close(&mut self, value: bool) -> Result<(), XErr>
	{
		unsafe {
			glfwSetWindowShouldClose(
				self.0,
				if value
				{
					GLFW_TRUE as i32
				}
				else
				{
					GLFW_FALSE as i32
				},
			)
		};

		XErr::result(|| {})
	}

	/// This function returns the title of the window. This is the title set
	/// previously by [Window::create] or [Window::set_title].
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized].
	///
	/// # Remarks
	/// The returned title is currently a copy of the title last set by
	/// [Window::create] or [Window::set_title]. It does not include any
	/// additional text which may be appended by the platform or another
	/// program.
	pub fn title(&self) -> Result<String, XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?.post_rcv(XWinMessage::GetWindowTitle(self.0, tx), rx)?
	}

	/// This function sets the title of the window.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized], [XErr::PlatformError],
	/// and [XErr::InvalidValue].
	///
	/// # Remarks
	/// - **MacOS**: The window title will not be updated until the next time
	///   you process events.
	pub fn set_title(&mut self, title: &str) -> Result<(), XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?.post_rcv(
			XWinMessage::SetWindowTitle(self.0, String::from(title), tx),
			rx,
		)?
	}

	/// This function sets the icon of the window. If passed an array
	/// of candidate images, those of or closest to the sizes desired by the
	/// system are selected. If no images are specified, the window reverts to
	/// its default icon.
	///
	/// The pixels are 32-bit, little-endian, non-premultiplied RGBA, i.e. eight
	/// bits per channel with the red channel first. They are arranged
	/// canonically as packed sequential rows, starting from the top-left
	/// corner.
	///
	/// The desired image sizes varies depending on platform and system
	/// settings. The selected images will be rescaled as needed. Good sizes
	/// include 16x16, 32x32 and 48x48.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized], [XErr::InvalidValue],
	/// [XErr::Platform], and [XErr::FeatureUnavailable].
	///
	/// # Remarks
	/// - **MacOS**: Regular windows do not have icons on macOS. This function
	///   will return [XErr::FeatureUnavailable]. The dock icon will be the same
	///   as the application bundle's icon. For more information on bundles, see
	///   the Bundle Programming Guide in the Mac Developer Library.
	/// - **Wayland**: There is no existing protocol to change an icon, the
	///   window will thus inherit the one defined in the application's desktop
	///   file. This function will return [XErr::FeatureUnavailable].
	pub fn set_icon(&mut self, icons: Vec<Image>) -> Result<(), XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?.post_rcv(XWinMessage::SetWindowIcon(self.0, icons, tx), rx)?
	}

	/// This function retrieves the position, in [ScreenCoordinates], of the
	/// upper-left corner of the content area of the window.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized], [XErr::Platform], and
	/// [XErr::FeatureUnavailable].
	///
	/// # Remarks
	/// - **Wayland**: There is no way for an application to retrieve the global
	///   position of its windows. This function will return
	///   [XErr::FeatureUnavailable].
	pub fn position(&self) -> Result<ScreenCoordinates, XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?.post_rcv(XWinMessage::GetWindowPos(self.0, tx), rx)?
	}

	/// This function sets the position, in [ScreenCoordinates], of the
	/// upper-left corner of the content area of the window. If the window is
	/// a full screen window, this function does nothing.
	///
	/// **Do not use this function** to move an already visible window unless
	/// you have very good reasons for doing so, as it will confuse and annoy
	/// the user.
	///
	/// The window manager may put limits on what positions are allowed. XWin
	/// cannot and should not override these limits.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized], [XErr::Platform],
	/// [XErr::FeatureUnavailable].
	///
	/// # Remarks
	/// - **Wayland**: There is no way for an application to set the global
	///   position of its windows. This function will return
	///   [XErr::FeatureUnavailable].
	pub fn set_position(&mut self, position: ScreenCoordinates) -> Result<(), XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?.post_rcv(XWinMessage::SetWindowPos(self.0, position, tx), rx)?
	}

	/// Returns the size, in [ScreenCoordinates], of the content area of this
	/// window. If you wish to get the size of the framebuffer of the window in
	/// pixels, see [framebuffer_size](Window::framebuffer_size).
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized] and [XErr::Platform].
	pub fn size(&self) -> Result<ScreenCoordinates, XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?.post_rcv(XWinMessage::GetWindowSize(self.0, tx), rx)?
	}

	/// Sets the size limits of the content area of this. If this
	/// window is full screen, the size limits only take effect once it is made
	/// windowed. If the window is not resizable, this function does nothing.
	///
	/// The size limits are applied immediately to a windowed mode window and
	/// may cause it to be resized.
	///
	/// The maximum dimensions must be greater than or equal to the minimum
	/// dimensions and must be greater than or equal to zero.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized], [XErr::InvalidValue],
	/// and [XErr::Platform]
	///
	/// # Remarks
	/// - If you set size limits and an aspect ratio that conflict, the results
	///   are undefined.
	/// - **Wayland**: The size limits will not be applied until the window is
	///   actually resized, either by the user or by the compositor.
	pub fn set_size_limits(
		&mut self,
		min: ScreenCoordinates,
		max: ScreenCoordinates,
	) -> Result<(), XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?.post_rcv(
			XWinMessage::SetWindowSizeLimits {
				window: self.0,
				min,
				max,
				tx,
			},
			rx,
		)?
	}

	/// Sets the required aspect ratio of the content area of the window. If
	/// the window is full screen, the aspect ratio only takes effect once it
	/// is made windowed. If the window is not resizable, this function does
	/// nothing.
	///
	/// The aspect ratio is specified as `(numerator, denominator)` and both
	/// values must be greater than zero. For example, the common 16:9 aspect
	/// ratio is specified as `(16, 9)`.
	///
	/// If the given ratio is `None`, hen the aspect ratio limit is disabled.
	///
	/// The aspect ratio is applied immediately to a windowed mode window and
	/// may cause it to be resized.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized], [XErr::InvalidValue],
	/// and [XErr::Platform].
	///
	/// # Remarks
	/// - If you set size limits and an aspect ratio that conflict, the results
	///   are undefined.
	/// - **Wayland**: The aspect ratio will not be applied until the window is
	///   actually resized, either by the user or by the compositor.
	pub fn set_aspect_ratio(&mut self, ratio: Option<(i32, i32)>) -> Result<(), XErr>
	{
		let (tx, rx) = channel();
		let value = ratio.or(Some((GLFW_DONT_CARE, GLFW_DONT_CARE))).unwrap();
		XWin::get()?.post_rcv(
			XWinMessage::SetWindowAspectRatio {
				window: self.0,
				numerator: value.0,
				denominator: value.1,
				tx,
			},
			rx,
		)?
	}

	/// Sets the size, in [ScreenCoordinates], of the content area
	/// of the window.
	///
	/// For full screen windows, this function updates the resolution of its
	/// desired video mode and switches to the video mode closest to it.
	///
	/// If you wish to update the refresh rate of the desired video mode in
	/// addition to its resolution, see [set_monitor](Window::set_monitor).
	///
	/// The window manager may put limits on what sizes are allowed. XWin cannot
	/// and should not override these limits.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized], [XErr::Platform].
	pub fn set_size(&mut self, size: ScreenCoordinates) -> Result<(), XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?.post_rcv(XWinMessage::SetWindowSize(self.0, size, tx), rx)?
	}

	/// Returns the size, in pixels (`width, height`), of the framebuffer of
	/// the window. If you wish to retrieve the size of the window in
	/// screen coordinates, see [size](Window::size).
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized], [XErr::Platform].
	pub fn framebuffer_size(&self) -> Result<(i32, i32), XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?.post_rcv(XWinMessage::GetFrameBufferSize(self.0, tx), rx)?
	}

	/// Returns the size, in Screen Coordinates, of each edge of
	/// the frame of the specified window. This size includes the title bar, if
	/// the window has one. The size of the frame may vary depending on the
	/// window-related hints used to create it.
	///
	/// The values are returned as `(left, top, right, bottom)`.
	///
	/// Because this function retrieves the size of each window frame edge and
	/// not the offset along a particular coordinate axis, the retrieved values
	/// will always be zero or positive.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized] and [XErr::Platform].
	pub fn frame_size(&self) -> Result<(u32, u32, u32, u32), XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?.post_rcv(XWinMessage::GetWindowFrameSize(self.0, tx), rx)?
	}

	/// Returns the [ContentScale] for the window. The content scale is the
	/// ratio between the current DPI and the platform's default DPI. This is
	/// especially important for text and any UI elements. If the pixel
	/// dimensions of your UI scaled by this look appropriate on your machine
	/// then it should appear at a reasonable size on other machines regardless
	/// of their DPI and scaling settings. This relies on the system DPI and
	/// scaling settings being somewhat correct.
	///
	/// On platforms where each monitors can have its own content scale, the
	/// window content scale will depend on which monitor the system considers
	/// the window to be on.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized] and [XErr::Platform].
	pub fn content_scale(&self) -> Result<ContentScale, XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?.post_rcv(XWinMessage::GetWindowContentScale(self.0, tx), rx)?
	}

	/// Returns the opacity of the window, including any decorations.
	///
	/// The opacity (or alpha) value is a positive finite number between zero
	/// and one, where zero is fully transparent and one is fully opaque. If the
	/// system does not support whole window transparency, this function always
	/// returns one.
	///
	/// The initial opacity value for newly created windows is one.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized] and [XErr::Platform].
	pub fn opacity(&self) -> Result<f32, XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?.post_rcv(XWinMessage::GetWindowOpacity(self.0, tx), rx)?
	}

	/// Sets the opacity of the window, including any decorations.
	///
	/// The opacity (or alpha) value is a positive finite number between zero
	/// and one, where zero is fully transparent and one is fully opaque.
	///
	/// The initial opacity value for newly created windows is one.
	///
	/// A window created with framebuffer transparency may not use whole window
	/// transparency. The results of doing this are undefined.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized], [XErr::Platform] and
	/// [XErr::FeatureUnavailable].
	///
	/// # Remarks
	/// - **Wayland**: There is no way to set an opacity factor for a window.
	///   This function will return [XErr::FeatureUnavailable].
	pub fn set_opacity(&mut self, opacity: f32) -> Result<(), XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?.post_rcv(XWinMessage::SetWindowOpacity(self.0, opacity, tx), rx)?
	}

	/// Iconifies (minimizes) the window if it was previously restored. If the
	/// window is already iconified, this function does nothing.
	///
	/// If the window is a full screen window, XWin restores the original video
	/// mode of the monitor. the window's desired video mode is set again when
	/// the window is restored.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized] and [XErr::Platform].
	///
	/// # Remarks
	/// - **Wayland**: Once a window is iconified, [restore](Window::restore)
	///   won't be able to restore it. This is a design decision of the
	///   xdg-shell protocol.
	pub fn iconify(&self) -> Result<(), XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?.post_rcv(XWinMessage::IconifyWindow(self.0, tx), rx)?
	}

	/// Restores this  window if it was previously iconified (minimized)
	/// or maximized. If the window is already restored, this function does
	/// nothing.
	///
	/// If the window is an iconified full screen window, its desired
	/// video mode is set again for its monitor when this is restored.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized] and [XErr::Platform].
	pub fn restore(&self) -> Result<(), XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?.post_rcv(XWinMessage::RestoreWindow(self.0, tx), rx)?
	}

	/// Maximizes the window if it was previously not maximized. If
	/// this is already maximized, this function does nothing.
	///
	/// If the window is a full screen window, this function does
	/// nothing.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized] and [XErr::Platform].
	pub fn maximize(&self) -> Result<(), XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?.post_rcv(XWinMessage::MaximizeWindow(self.0, tx), rx)?
	}

	/// Makes the window visible if it was previously hidden. If this
	/// window is already visible or is in full screen mode, this function does
	/// nothing.
	///
	/// By default, windowed mode windows are focused when shown. Set the
	/// [WindowBuilder::focus_on_show] window hint to `true` to change this
	/// behavior a newly created window, or change the behavior for an
	/// existing window with [set_attribute](Window::set_attribute).
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized] and [XErr::Platform].
	///
	/// # Remarks
	/// - **Wayland**: Because Wayland wants every frame of the desktop to be
	///   complete, this function dow not immediately make the window visible.
	///   Instead, it will become visible the next time the window framebuffer
	///   is updated after this call.
	pub fn show(&self) -> Result<(), XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?.post_rcv(XWinMessage::ShowWindow(self.0, tx), rx)?
	}

	/// Hides the window if it was previously visible. If the window is already
	/// hidden or is in full screen mode, this function does nothing.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized] and [XErr::Platform].
	pub fn hide(&self) -> Result<(), XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?.post_rcv(XWinMessage::HideWindow(self.0, tx), rx)?
	}

	/// Brings the window to front and sets input focus. The window should
	/// already be visible and not iconified.
	///
	/// By default, both windowed and full screen mode windows are focused when
	/// initially created. Set the [WindowBuilder::focused] window hint to
	/// `false` to disable this behavior.
	///
	/// Also by default, windowed mode windows are focused when shown with
	/// [show](Window::show). Set the [WindowBuilder::focus_on_show] window hint
	/// to `false` to disable this behavior.
	///
	/// **Do not use this function** to steal focus from other applications
	/// unless you are certain that is what the user wants. Focus stealing can
	/// be extremely disruptive.
	///
	/// For a less disruptive way of getting the user's attention, see
	/// [request_attention](Window::request_attention).
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized] and [XErr::Platform].
	///
	/// # Remarks
	/// - **Wayland**: The compositor will likely ignore focus requests unless
	///   another window created by the same application already has input
	///   focus.
	pub fn focus(&self) -> Result<(), XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?.post_rcv(XWinMessage::FocusWindow(self.0, tx), rx)?
	}

	/// Requests user attention to the window. On platforms where this is not
	/// supported, attention is requested to the application as a whole.
	///
	/// Once the user has given attention, usually by focusing the window or
	/// application, the system will end the request automatically.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized] and [XErr::Platform].
	///
	/// # Remarks
	/// - **MacOS**: Attention is requested to the application as a whole not
	///   the specific window.
	pub fn request_attention(&self) -> Result<(), XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?.post_rcv(XWinMessage::RequestWindowAttention(self.0, tx), rx)?
	}

	/// Returns the [Monitor] that the window is full screen on, or `None` if
	/// the window is in windowed mode.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized].
	pub fn monitor(&self) -> Result<Option<Monitor>, XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?.post_rcv(XWinMessage::GetWindowMonitor(self.0, tx), rx)?
	}

	/// Sets the window to fullscreen mode on a given monitor.
	///
	/// This function updates the width, height, and refresh rate of the desired
	/// video mode and switches to the video mode closest to it.
	///
	/// If you only wish to update the resolution of a full screen window, see
	/// [set_size](Window::set_size).
	///
	/// # Parameters
	/// - `monitor`: The desired monitor for full screen mode.
	/// - `size`: The desired size, in [ScreenCoordinates], of the video mode.
	/// - `refresh_hz`: The desired rate, in Hz, of the video mode, or `None`
	///   for no preference.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized] and [XErr::Platform].
	pub fn fullscreen(
		&self,
		monitor: Monitor,
		size: ScreenCoordinates,
		refresh_hz: Option<i32>,
	) -> Result<(), XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?.post_rcv(
			XWinMessage::SetWindowFullscreen {
				window: self.0,
				monitor,
				size,
				refresh_rate: match refresh_hz
				{
					| Some(rate) => rate,
					| None => GLFW_DONT_CARE,
				},
				tx,
			},
			rx,
		)?
	}

	/// Sets the window to windowed mode.
	///
	/// `position` and `size` are used to place the window content area.
	///
	/// If you only wish to update the size of a windowed mode window, see
	/// [set_size](Window::set_size).
	///
	/// # Parameters
	/// - `position`: The desired [ScreenCoordinates] of the upper-left corner
	///   of the content area.
	/// - `size`: The desired size, in [ScreenCoordinates], of the content area.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized] and [XErr::Platform].
	///
	/// # Remarks
	/// - **Wayland**: The desired window position is ignored, as there is no
	///   way for an application to set this property.
	pub fn set_windowed(
		&self,
		position: ScreenCoordinates,
		size: ScreenCoordinates,
	) -> Result<(), XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?.post_rcv(
			XWinMessage::SetWindowWindowed {
				window: self.0,
				position,
				size,
				tx,
			},
			rx,
		)?
	}

	/// Construct a new [Window] from a `GLFWwindow`.
	pub(crate) fn from_glfw(win: *mut GLFWwindow) -> Self
	{
		Window(win)
	}
}

impl Drop for Window
{
	/// Destroys the window. Callbacks on the window may continue to be called
	/// until the window has been fully destroyed.
	///
	/// # Reentrancy
	/// This function must not be called from a callback.
	fn drop(&mut self)
	{
		let (tx, rx) = channel();
		if let Ok(xwin) = XWin::get()
		{
			let _ = xwin.post_rcv(XWinMessage::DestroyWindow(self.0, tx), rx);
		}
	}
}
