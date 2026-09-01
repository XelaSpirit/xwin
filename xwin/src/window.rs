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
//! ## Context Object Sharing
//! When creating a window and its OpenGL or OpenGL ES context, you can specify
//! another window whose context the new one should share its objects (textures,
//! vertex, and element buffers, etc.) with.
//!
//! Object sharing is implemented by the operating system and graphics driver.
//! On platforms where it is possible to choose which types of object are
//! shared, XWin requests that all types are shared.
//!
//! ```
//! # use xwin::window::{Window, WindowBuilder};
//! # if false { // GLFW doesn't seem to initialize in doc tests
//! # let other_window: Window = WindowBuilder::new().create(1920, 1080, "Title", None, None).unwrap();
//! let win = WindowBuilder::new().create(1920, 1080, "title", None, Some(other_window));
//! # }
//! ```
//!
//! See the relevant chapter of the OpenGL or OpenGL ES reference documents for
//! more information. The name and number of this chapter unfortunately varies
//! between versions and APIs, but has at times been named *Shared Object and
//! Multiple Contexts*.
//!
//! ## Offscreen Contexts
//! XWin doesn't support creating contexts without an associated window.
//! However, contexts with hidden windows can be created with the
//! [WindowBuilder::visible] window hint.
//!
//! ```
//! # use xwin::window::{Window, WindowBuilder};
//! # if false { // GLFW doesn't seem to initialize in doc tests
//! let win = WindowBuilder::new()
//! 	.visible(false)
//! 	.create(1920, 1080, "title", None, None);
//! # }
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
//! When moving a [Window] between threads, the context is automatically
//! detached from its current thread before moving.
//!
//! TODO - implement above
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
	ptr::null_mut,
	sync::{
		Mutex,
		mpsc::channel,
	},
};

pub use builder::*;
pub use callback::*;

use crate::{
	bind::{
		GLFW_FALSE,
		GLFW_TRUE,
		GLFWwindow,
		glfwGetCurrentContext,
		glfwMakeContextCurrent,
		glfwSetWindowShouldClose,
		glfwSwapInterval,
		glfwWindowShouldClose,
	},
	core::{
		XWin,
		exec::XWinMessage,
	},
	err::XErr,
	monitor::Monitor,
};

pub struct Window
{
	window: *mut GLFWwindow,
	lock:   Mutex<()>,
}

unsafe impl Send for Window {}
unsafe impl Sync for Window {}

impl Window
{
	/// This function creates a window and its associated OpenGL or OpenGL ES
	/// context. Options controlling how the window and its context should be
	/// created are specified using other functions in [WindowBuilder].
	///
	/// Successful creation does not change which context is current. Before you
	/// can use the newly created context, you need to make it current.
	///
	/// For information about the `share` parameter, see [Context Object
	/// Sharing](crate::window#context-object-sharing).
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
	/// # Panics
	/// Will panic if `title` contains any null bytes (`\0`).
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
		share: Option<Window>,
	) -> Result<Self, XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?.post_rcv(
			XWinMessage::CreateWindow {
				width,
				height,
				title: String::from(title),
				monitor,
				share,
				builder: None,
				tx,
			},
			rx,
		)?
	}

	/// If this window is the current context, detaches it.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized], [XErr::NoWindowContext],
	/// [XErr::Platform].
	pub fn detach(&self) -> Result<(), XErr>
	{
		if self.is_current()?
		{
			unsafe { glfwMakeContextCurrent(null_mut()) };
		}
		XErr::result(|| {})
	}

	/// Sets the OpenGL or OpenGL ES context of this window as the current
	/// context on the calling thread.
	///
	/// A context may only be made current on a single thread at a time and each
	/// thread can have only a single current context at a time. Making a
	/// context current detaches any previously current context on the calling
	/// thread.
	///
	/// By default, making a context non-current implicitly forces a pipeline
	/// flush. On machines that support `GL_KHR_context_flush_control`, you can
	/// control whether a context performs this flush by setting the
	/// [WindowBuilder::context_release_behavior](WindowBuilder::context_release_behavior) hint.
	///
	/// This window must have an OpenGL or OpenGL ES context. Using this without
	/// a context will return [XErr::NoWindowContext].
	///
	/// # Remarks
	/// If the previously current context was created via a different context
	/// creation API than this window, XWin will still detach the previous one
	/// from its API before making this one current.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized], [XErr::NoWindowContext],
	/// and [XErr::Platform].
	pub fn set_current(&self) -> Result<(), XErr>
	{
		unsafe { glfwMakeContextCurrent(self.window) };
		XErr::result(|| {})
	}

	/// Returns whether this window is the current context.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized].
	pub fn is_current(&self) -> Result<bool, XErr>
	{
		let res = unsafe { glfwGetCurrentContext() == self.window };
		XErr::result(|| res)
	}

	/// Sets the swap interval for this window's OpenGL or OpenGL ES
	/// context, i.e. the number of screen updates to wait from the time
	/// [Window::swap_buffers] was called before swapping the buffers and
	/// returning. This is sometimes called vertical synchronization, vertical
	/// retrace synchronization or just vsync.
	///
	/// A context that supports either of the `WGL_EXT_swap_control_tear` and
	/// `GLX_EXT_swap_control_tear` extensions also accepts negative swap
	/// intervals, which allows the driver to swap immediately even if a frame
	/// arrives a little late.
	///
	/// This function does not apply to Vulkan. If you are rendering with
	/// Vulkan, see the present mode of your swapchain instead.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized],
	/// [XErr::NoCurrentContext], and [XErr::Platform].
	///
	/// # Remarks
	/// This function is not called during context creation, leaving the swap
	/// interval set to whatever is the default for that API. This is done
	/// because some swap interval extensions used by XWin do not allow the swap
	/// interval to be reset to zero once it has been set to a non-zero value.
	///
	/// Some GPU drivers do not honor the requested swap interval, either
	/// because of a user setting that overrides the application's request or
	/// due to bugs in the driver.
	pub fn set_swap_interval(&self, swap_interval: i32) -> Result<(), XErr>
	{
		let ctx = unsafe { glfwGetCurrentContext() };
		XErr::result(|| {})?;

		self.set_current()?;

		unsafe {
			glfwSwapInterval(swap_interval);
			XErr::result(|| {})?;

			glfwMakeContextCurrent(ctx);
		};
		XErr::result(|| {})
	}

	/// Returns the value of the close flag of this window.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized].
	pub fn should_close(&self) -> Result<bool, XErr>
	{
		let lock = self.lock.lock().unwrap();
		let close = unsafe { glfwWindowShouldClose(self.window) == GLFW_TRUE as i32 };
		drop(lock);

		XErr::result(|| close)
	}

	/// Sets the value of the close flag of this window. This can be used to
	/// override the user's attempt to close the window, or to signal that it
	/// should be closed.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized].
	///
	/// # Thread Safety
	/// This function may be called from any thread. Access is not synchronized.
	pub fn set_should_close(&self, value: bool) -> Result<(), XErr>
	{
		let lock = self.lock.lock().unwrap();
		unsafe {
			glfwSetWindowShouldClose(
				self.window,
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
		drop(lock);

		XErr::result(|| {})
	}

	/// Construct a new [Window] from a `GLFWwindow`.
	pub(crate) fn from_glfw(win: *mut GLFWwindow) -> Self
	{
		Window {
			window: win,
			lock:   Mutex::new(()),
		}
	}

	/// Return the `GLFWwindow` held by this [Window].
	pub(crate) fn get_glfw(&self) -> *mut GLFWwindow
	{
		self.window
	}
}

impl Drop for Window
{
	/// Destroys the window and its context. On calling this function, no
	/// further callbacks will be called for this window.
	///
	/// If the context of this window is set as current, it is detached before
	/// being destroyed.
	///
	/// # Reentrancy
	/// This function must not be called from a callback.
	fn drop(&mut self)
	{
		let _ = self.detach();
		let (tx, rx) = channel();
		match XWin::get()
		{
			| Err(_) =>
			{},
			| Ok(xwin) =>
			{
				let _ = xwin.post_rcv(
					XWinMessage::DestroyWindow(Window::from_glfw(self.window), tx),
					rx,
				);
			},
		};
	}
}
