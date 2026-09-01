//! XWin core functionality.
//!
//! # Initialization and Termination
//!
//! Before most XWin functions may be called, the library must be initialized.
//! This initialization checks what features are available on the machine,
//! enumerates monitors, initializes the timer and performs any required
//! platform-specific initialization.
//!
//! Only the following functions may be called before the library has been
//! successfully initialized, and only from the main thread.
//!
//! - [glfw_version]
//! - [platform_supported]
//! - [XWin::platform]
//! - [XWin::cocoa_dir_resources]
//! - [XWin::cocoa_menubar]
//! - [XWin::wayland_libdecor]
//!
//! Calling any other function before successful initialization will produce
//! [XErr::NotInitialized].
//!
//! ## Initializing XWin
//! The library can be initialized with [XWin::default], which returns an `XWin`
//! instance on success, or an [XErr] if any errors occurred.
//!
//! ```
//! # use xwin::core::XWin;
//! let xwin = XWin::default().unwrap();
//! ```
//! If any part of initialization fails, any parts that succeeded are terminated
//! as if [XWin::drop] had been called. The library only needs to be
//! initialized once and additional calls to an already initialized library will
//! return `Some(xwin)` immediately.
//!
//! Once the library has been successfully initialized, it should be terminated
//! before the application exits. Modern systems are very good at freeing
//! resources allocated by programs that exit, but XWin sometimes has to change
//! global system settings and these might not be restored without termination.
//!
//! **MacOS:** When the library is initialized the main menu and dock icon are
//! created. These are not desirable for a command-line only program. The
//! creation of the main menu and dock icon can be disabled with the
//! [XWin::cocoa_menubar] function.
//!
//! # Configuring XWin Initialization
//!
//! Use [XWin::new] followed by some number of the functions in this struct to
//! configure XWin before initialization, concluding with [XWin::init]. These
//! functions will affect how the library behaves until termination.
//!
//! ```
//! # use xwin::core::{Platform, XWin};
//! let xwin = XWin::new()
//! 	// ...
//! 	.init();
//! ```
//! The configuration you set is never reset by XWin, but it only takes
//! effect during initialization. Once XWin has been initialized, any
//! configuration you set here will be ignored until the library is terminated
//! and initialized again.
//!
//! Some settings are platform specific. These may be set on any platform, but
//! they will only affect their specific platform. Other platforms will ignore
//! them. Setting these values requires no platform specific headers or
//! functions.
//!
//! See the following functions for more specific information on what they do:
//! - [XWin::platform]
//! - [XWin::cocoa_dir_resources]
//! - [XWin::cocoa_menubar]
//! - [XWin::wayland_libdecor]
//!
//! ## Runtime Platform Selection
//! You can control platform selection via the [XWin::platform] function. By
//! default, this is set to [Platform::Any], which will look for supported
//! window systems in order of priority and select the first one it finds. It
//! can also be set to any specific platform to have XWin only look for that
//! one.
//!
//! ```
//! # use xwin::core::{Platform, XWin};
//! let xwin = XWin::new().platform(Platform::Any).init();
//! ```
//!
//! This mechanism also provides the [Null](Platform::Null) platform, which is
//! always supported but needs to be explicitly requested. This platform is
//! effectively a stub, emulating a window system on a single 1080p monitor, but
//! will not interact with any actual window system.
//!
//! ```
//! # use xwin::core::{Platform, XWin};
//! let xwin = XWin::new().platform(Platform::Null).init();
//! ```
//!
//! You can test whether a library binary was compiled with support for a
//! specific platform with [platform_supported].
//! ```
//! # use xwin::core::{platform_supported, Platform, XWin};
//! if platform_supported(Platform::X11)
//! {
//! 	let xwin = XWin::new().platform(Platform::X11).init();
//! }
//! ```
//!
//! Once XWin has been initialized, you can query which platform was selected
//! with [platform].
//! ```
//! # use xwin::core::{platform, XWin};
//! # let xwin = XWin::default();
//! let platform = platform();
//! ```
//!
//! ## Terminating XWin
//! XWin will be automatically terminated when [XWin::drop] is called (see
//! [XWin::terminate_on_drop] to disable this behavior). For this reason, it is
//! necessary to keep your [XWin] instance alive for as long as you intend to
//! use this library. The easiest way to do this is to initialize XWin at the
//! top of your main function, and save the result to a variable. This way, XWin
//! won't be dropped until the program is terminating.
//!
//! # Guarantees and Limitations
//! This section describes the conditions under which XWin can be expected to
//! function, barring bugs in the operating system or drivers. Use of XWin
//! outside these limits may work on some platforms, or on some machines, or
//! some of the time, or on some versions of XWin, but may break at any time
//! and will not be considered a bug.
//!
//! ## Reentrancy
//! XWin event processing and object destruction are not reentrant. This means
//! that the following functions must not be called from any window function:
//!
//! - TODO link functions here (destroy window, destroy cursor, poll events,
//!   wait events, wait events timeout, terminate)
//!
//! These functions may be made reentrant in future minor or patch releases, but
//! functions not on this list will not be made non-reentrant.
//!
//! ## Thread Safety
//! Most XWin functions must only be called from the main thread (the thread
//! that calls main), but some may be called from any thread once the library
//! has been initialized. Before initialization the whole library is
//! thread-unsafe.
//!
//! The reference documentation for every XWin function states if it is
//! limited to the main thread.
//!
//! Initialization, termination, event processing and the creation and
//! destruction of windows, and cursors are all restricted to the main thread
//! due to limitations of one or several platforms.
//!
//! Because event processing must be performed on the main thread, all callbacks
//! will only be called on that thread.
//!
//! XWin uses synchronization objects internally only to manage the per-thread
//! context and error states. Additional synchronization is left to the
//! application.
//!
//! Functions that may currently be called from any thread will always remain
//! so, but functions that are currently limited to the main thread may be
//! updated to allow calls from any thread in future releases.
//!
//! ## Event Order
//! The order of arrival of related events is not guaranteed to be consistent
//! across platforms. The exception is synthetic key and mouse button release
//! events, which are always delivered after the window defocus event.

use std::{
	cell::Cell,
	marker::PhantomData,
	os::raw::c_int,
};

#[cfg(feature = "tracing")]
use crate::err::set_error_log;
use crate::{
	bind::{
		GLFW_ANY_PLATFORM,
		GLFW_COCOA_CHDIR_RESOURCES,
		GLFW_COCOA_MENUBAR,
		GLFW_FALSE,
		GLFW_PLATFORM,
		GLFW_PLATFORM_COCOA,
		GLFW_PLATFORM_NULL,
		GLFW_PLATFORM_WAYLAND,
		GLFW_PLATFORM_WIN32,
		GLFW_PLATFORM_X11,
		GLFW_TRUE,
		GLFW_WAYLAND_DISABLE_LIBDECOR,
		GLFW_WAYLAND_LIBDECOR,
		GLFW_WAYLAND_PREFER_LIBDECOR,
		glfwGetPlatform,
		glfwGetVersion,
		glfwInit,
		glfwInitHint,
		glfwPlatformSupported,
		glfwTerminate,
	},
	err::XErr,
	monitor::set_monitor_callback,
};

/// XWin has two primary coordinate systems: the **virtual screen** and the
/// window **content area**. Both use the same unit: `virtual screen
/// coordinates`, or just [ScreenCoordinates], which don't
/// necessarily correspond to pixels.
///
/// Both the virtual screen and the content area coordinate systems have the
/// X-axis pointing to the right and the Y-axis pointing down.
///
/// Window and monitor positions are specified as the position of the upper-left
/// corners of their content areas relative to the virtual screen, while cursor
/// positions are specified relative to a window's content area.
///
/// Because the origin of the window's content area coordinate system is also
/// the point from which the window position is specified, you can translate
/// content area coordinates to the virtual screen by adding the window
/// position. The window frame, when present, extends out from the content area
/// but does not affect the window position.
///
/// Almost all positions and sizes in XWin are measured in [ScreenCoordinates]
/// relative to one of the two origins above. This includes cursor positions,
/// window positions and sizes, window frame sizes, monitor positions and video
/// mode resolutions.
///
/// Two exceptions are the **monitor physical size**, which is measured in
/// **millimetres**, and **framebuffer size**, which is measured in **pixels**.
///
/// Pixels and [ScreenCoordinates] may map 1:1 on your machine, but they won't
/// on every other machine, for example on a Mac with a Retina display. The
/// ratio between [ScreenCoordinates] and pixels may also change at run-time
/// depending on which monitor the window is currently considered to be on.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct ScreenCoordinates
{
	pub x: i32,
	pub y: i32,
}

/// The content scale can be thought of as the ratio between the current DPI and
/// the platform's default DPI. It is intended to be a scaling factor to apply
/// to the pixel dimensions of text and other UI elements. If the dimensions
/// scaled by this factor looks appropriate on your machine then it should
/// appear at a reasonable size on other machines with different DPI and scaling
/// settings.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct ContentScale
{
	pub x: f32,
	pub y: f32,
}

impl Default for ScreenCoordinates
{
	fn default() -> ScreenCoordinates
	{
		ScreenCoordinates { x: 0, y: 0 }
	}
}

/// Used to configure [XWin]. Specifies the platform to use for windowing and
/// input.
pub enum Platform
{
	Any,
	Windows,
	Cocoa,
	Wayland,
	X11,
	Null,
}

/// A structure for handling the initialization and termination of the XWin
/// library. For a more complete guide, see [the core module
/// documentation](crate::core)
pub struct XWin(bool, PhantomData<Cell<()>>);

// TODO - glfwInitVulkanLoader

impl XWin
{
	/// Initialize the XWin library with default settings. See [XWin::init] for
	/// a more complete description.
	pub fn default() -> Result<Self, XErr>
	{
		XWin::new().init()
	}

	/// This function initializes the XWin library. Before most XWin functions
	/// can be used, XWin must be initialized. When an [XWin] goes out of scope,
	/// the library is terminated in order to free any resources allocation
	/// during or after initialization.
	///
	/// If this function fails, it terminates XWin before returning and error.
	/// If it succeeds, termination is handled automatically with the [Drop]
	/// trait.
	///
	/// Additional calls to this function after successful initialization but
	/// before termination will succeed and return a new [XWin]. Note that the
	/// XWin library is terminated when *any* [XWin] is dropped. It's
	/// recommended to create only one instance of [XWin] at the top of your
	/// main function and keep it alive for the duration of the program's
	/// runtime.
	///
	/// [XWin::platform] function can be used to control which platforms are
	/// considered during initialization. This also depends on which platforms
	/// the library was compiled to support.
	///
	/// # Returns
	/// A new [XWin] instance if successful, or an error if one occurred
	///
	/// # Errors
	/// Possible errors include [PlatformUnavailable](XErr::PlatformUnavailable)
	/// and [PlatformError](XErr::Platform).
	///
	/// # Remarks
	/// - **macOS:** This function will change the current directory of the
	///   application to the Contents/Resources subdirectory of the
	///   application's bundle, if present. This can be disabled with the
	///   [cocoa_dir_resources](XWin::cocoa_dir_resources) function.
	///
	/// - **macOS:** This function will create the main menu and dock icon for
	///   the application. If XWin finds a `MainMenu.nib` it is loaded and
	///   assumed to contain a menu bar. Otherwise a minimal menu bar is created
	///   manually with common commands like `Hide`, `Quit` and `About`. The
	///   `About` entry opens a minimal about dialog with information from the
	///   application's bundle. The menu bar and dock icon can be disabled
	///   entirely with the [cocoa_menubar](XWin::cocoa_menubar) function.
	///
	/// - **Wayland, X11:** If the library was compiled with support for both
	///   `Wayland` and `X11`, and the [platform](XWin::platform) config is set
	///   to [Platform::Any], the `XDG_SESSION_TYPE` environment variable
	///   affects which platform is picked. If the environment variable is not
	///   set, or is set to something other than `wayland` or `x11`, the regular
	///   detection mechanism will be used instead.
	///
	/// - **X11:** This function will set the `LC_CTYPE` category of the
	///   application locale according to the current environment if that
	///   category is still "C". This is because the "C" locale breaks Unicode
	///   text input.
	pub fn init(&self) -> Result<Self, XErr>
	{
		#[cfg(feature = "tracing")]
		set_error_log();

		if unsafe { glfwInit() } != GLFW_TRUE as i32
		{
			Err(XErr::get())
		}
		else
		{
			set_monitor_callback();
			Ok(XWin(self.0, PhantomData::default()))
		}
	}

	/// Returns an uninitialized XWin that can be used to configure XWin before
	/// initialization.
	///
	/// Configuration you set is never reset by XWin, but it only takes effect
	/// during initialization. Once XWin has been initialized, any further
	/// configuration will be ignored until the library is terminated and
	/// initialized again.
	///
	/// Some configuration is platform specific. These may be set on any
	/// platform, but they will only affect their specific platform. Other
	/// platforms will ignore them. Setting such configuration requires no
	/// platform-specific crates or functions
	///
	/// After configuration is complete, call [XWin::init] to complete
	/// initialization of the library.
	pub fn new() -> Self
	{
		XWin(true, PhantomData::default())
	}

	/// Set the platform to use for windowing and input.
	///
	/// **Default:** [`Platform::Any`]
	///
	/// # Thread Safety
	/// This function must only be called from the main thread.
	pub fn platform(&mut self, platform: Platform) -> &mut Self
	{
		unsafe {
			glfwInitHint(
				GLFW_PLATFORM as c_int,
				match platform
				{
					| Platform::Any => GLFW_ANY_PLATFORM as c_int,
					| Platform::Windows => GLFW_PLATFORM_WIN32 as c_int,
					| Platform::Cocoa => GLFW_PLATFORM_COCOA as c_int,
					| Platform::Wayland => GLFW_PLATFORM_WAYLAND as c_int,
					| Platform::X11 => GLFW_PLATFORM_X11 as c_int,
					| Platform::Null => GLFW_PLATFORM_NULL as c_int,
				},
			);
		}
		self
	}

	/// **MacOS Specific**
	///
	/// Specifies whether to set the current directory to the application to the
	/// `Contents/Resources` subdirectory of the application's bundle, if
	/// present. This is ignored on other platforms.
	///
	/// # Thread Safety
	/// This function must only be called from the main thread.
	pub fn cocoa_dir_resources(&mut self, value: bool) -> &mut Self
	{
		unsafe {
			glfwInitHint(
				GLFW_COCOA_CHDIR_RESOURCES as c_int,
				if value
				{
					GLFW_TRUE as c_int
				}
				else
				{
					GLFW_FALSE as c_int
				},
			)
		};
		self
	}

	/// **MacOS Specific**
	///
	/// Specifies whether to create the menu bar and dock icon when XWin is
	/// initialized. This applies whether the menu bar is created from a nib or
	/// manually by XWin. This is ignored on other platforms.
	///
	/// # Thread Safety
	/// This function must only be called from the main thread.
	pub fn cocoa_menubar(&mut self, value: bool) -> &mut Self
	{
		unsafe {
			glfwInitHint(
				GLFW_COCOA_MENUBAR as c_int,
				if value
				{
					GLFW_TRUE as c_int
				}
				else
				{
					GLFW_FALSE as c_int
				},
			)
		};
		self
	}

	/// **Wayland Specific**
	///
	/// specifies whether to use [libdecor](https://gitlab.freedesktop.org/libdecor/libdecor)
	/// for window decorations where available. This is ignored on other
	/// platforms.
	///
	/// # Thread Safety
	/// This function must only be called from the main thread.
	pub fn wayland_libdecor(&mut self, value: bool) -> &mut Self
	{
		unsafe {
			glfwInitHint(
				GLFW_WAYLAND_LIBDECOR as c_int,
				if value
				{
					GLFW_WAYLAND_PREFER_LIBDECOR as c_int
				}
				else
				{
					GLFW_WAYLAND_DISABLE_LIBDECOR as c_int
				},
			)
		};
		self
	}

	/// Specifies whether to terminate the XWin library when this instance of
	/// [XWin] is dropped.
	///
	/// # Remarks
	/// By default, this value is set to true. Assuming you intend to have XWin
	/// initialized for the duration of your runtime, there should be no need to
	/// change this, simply save the result of calling [XWin::init] in a
	/// variable at the top of your main function so that it won't drop until
	/// your program is terminating.
	///
	/// If you have a specific need to explicitly control the termination /
	/// reinitialization of the XWin library, you can use this function to
	/// prevent auto-termination on drop.
	///
	/// # Thread Safety
	/// This function must only be called from the main thread.
	pub fn terminate_on_drop(&mut self, value: bool) -> &mut Self
	{
		self.0 = value;
		self
	}
}

impl Drop for XWin
{
	/// This function destroys all remaining windows and cursors, restores any
	/// modified gamma ramps and frees any other allocated resources. Once this
	/// function is called, you must again call [XWin::new] successfully before
	/// you will be able to use most XWin functions.
	///
	/// If XWin has been successfully initialized, this function will be
	/// called before the application exits. If initialization fails, there is
	/// no need to call this function, as it is called by [XWin::new] before it
	/// returns failure.
	///
	/// This function has no effect if XWin is not initialized.
	///
	/// # Errors
	/// Possible errors include [PlatformError](XErr::Platform). However,
	/// since it's assumed this will likely be called when an application is
	/// closing, and there's not much value in reporting such an error anyway,
	/// no error checking or handling is done here.
	///
	/// # Thread Safety
	/// This function must only be called from the main thread.
	fn drop(&mut self)
	{
		if self.0
		{
			unsafe { glfwTerminate() };
		}
	}
}

/// This function retrieves the major, minor and revision numbers of the GLFW
/// library. It is intended for when you are using GLFW as a shared library and
/// want to ensure that you are using the minimum required version.
///
/// # Remarks
/// This function may be called before initializing XWin
///
/// # Thread Safety
/// This function may be called from any thread.
pub fn glfw_version() -> (u32, u32, u32)
{
	let mut major: c_int = 0;
	let mut minor: c_int = 0;
	let mut patch: c_int = 0;
	unsafe { glfwGetVersion(&mut major, &mut minor, &mut patch) };

	(major as u32, minor as u32, patch as u32)
}

/// This function returns the platform that was selected during initialization.
///
/// # Returns
/// The currently selected platform, or an error if one occurred.
///
/// # Errors
/// Possible errors include [XErr::NotInitialized].
///
/// # Thread Safety
/// This function may be called from any thread.
///
/// # See Also
/// [platform_supported]
pub fn platform() -> Result<Platform, XErr>
{
	let plat = unsafe { glfwGetPlatform() as u32 };

	match plat
	{
		| 0 => Err(XErr::get()),
		| GLFW_PLATFORM_WIN32 => Ok(Platform::Windows),
		| GLFW_PLATFORM_COCOA => Ok(Platform::Cocoa),
		| GLFW_PLATFORM_WAYLAND => Ok(Platform::Wayland),
		| GLFW_PLATFORM_X11 => Ok(Platform::X11),
		| GLFW_PLATFORM_NULL => Ok(Platform::Null),
		| _ => Err(XErr::Unknown),
	}
}

/// This function returns whether the library was compiled with support for the
/// specified platform.
///
/// # Parameters
/// `platform`: The platform to query
///
/// # Returns
/// `true` if the platform is supported, `false` otherwise.
///
/// # Remarks
/// This function may be called before initializing XWin
///
/// # Thread Safety
/// This function may be called from any thread
///
/// # See Also
/// [platform]
pub fn platform_supported(platform: Platform) -> bool
{
	unsafe {
		match glfwPlatformSupported(match platform
		{
			| Platform::Any => GLFW_ANY_PLATFORM as c_int,
			| Platform::Windows => GLFW_PLATFORM_WIN32 as c_int,
			| Platform::Cocoa => GLFW_PLATFORM_COCOA as c_int,
			| Platform::Wayland => GLFW_PLATFORM_WAYLAND as c_int,
			| Platform::X11 => GLFW_PLATFORM_X11 as c_int,
			| Platform::Null => GLFW_PLATFORM_NULL as c_int,
		}) as u32
		{
			| GLFW_TRUE => true,
			| GLFW_FALSE => false,
			| _ => false,
		}
	}
}
