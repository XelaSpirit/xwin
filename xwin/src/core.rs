//! # Core Functionality
//!
//! This covers the core functionality of XWin, primarily initialization and
//! termination of the XWin library.

use std::os::raw::c_int;

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
};

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

pub struct XWin(());

impl XWin
{
	/// Initialize the XWin library with default configuration. See [XWin::init]
	/// for a full description.
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
	/// The GLFW_PLATFORM init hint controls which platforms are considered
	/// during initialization. This also depends on which platforms the library
	/// was compiled to support.
	///
	/// TODO add link to platform init hint
	///
	/// # Returns
	/// A new XWin instance if successful, or an error if one occurred
	///
	/// # Errors
	/// Possible errors include [PlatformUnavailable](XErr::PlatformUnavailable)
	/// and [PlatformError](XErr::PlatformError).
	///
	/// # Remarks
	/// - **macOS:** This function will change the current directory of the
	///   application to the Contents/Resources subdirectory of the
	///   application's bundle, if present. This can be disabled with the
	///   GLFW_COCOA_CHDIR_RESOURCES init hint. TODO link init hint
	///
	/// - **macOS:** This function will create the main menu and dock icon for
	///   the application. If XWin finds a MainMenu.nib it is loaded and assumed
	///   to contain a menu bar. Otherwise a minimal menu bar is created
	///   manually with common commands like Hide, Quit and About. The About
	///   entry opens a minimal about dialog with information from the
	///   application's bundle. The menu bar and dock icon can be disabled
	///   entirely with the GLFW_COCOA_MENUBAR init hint. TODO link init hint
	///
	/// - **Wayland, X11:** If the library was compiled with support for both
	///   Wayland and X11, and the GLFW_PLATFORM init hint is set to
	///   GLFW_ANY_PLATFORM, the XDG_SESSION_TYPE environment variable affects
	///   which platform is picked. If the environment variable is not set, or
	///   is set to something other than wayland or x11, the regular detection
	///   mechanism will be used instead. TODO link hints
	///
	/// - **X11:** This function will set the LC_CTYPE category of the
	///   application locale according to the current environment if that
	///   category is still "C". This is because the "C" locale breaks Unicode
	///   text input.
	///
	/// # Thread Safety
	/// This function must only be called from the main thread.
	pub fn init(&self) -> Result<Self, XErr>
	{
		#[cfg(feature = "tracing")]
		set_error_log();

		let init = unsafe { glfwInit() };

		if init != GLFW_TRUE as i32
		{
			Err(XErr::get())
		}
		else
		{
			Ok(XWin(()))
		}
	}

	/// Returns an uninitialized XWin that can be used to configure XWin before
	/// initialization.
	///
	/// Configuration you set is never reset by XWin, but it only takes effect
	/// during initialization. Once XWin has been initialized, any further
	/// configuration will be ignored until the library is terminated and
	/// initialized again (this would require dropping the XWin instance and
	/// creating a new one).
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
		XWin(())
	}

	/// Set the platform to use for windowing and input.
	///
	/// **Default:** [`Platform::Any`]
	///
	/// # Thread Safety
	/// This function must only be called from the main thread.
	pub fn platform(&self, platform: Platform) -> &Self
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
	pub fn cocoa_dir_resources(&self, value: bool) -> &Self
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
	pub fn cocoa_menubar(&self, value: bool) -> &Self
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
	pub fn wayland_libdecor(&self, value: bool) -> &Self
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
	/// Possible errors include [PlatformError](XErr::PlatformError). However,
	/// since it's assumed this will likely be called when an application is
	/// closing, and there's not much value in reporting such an error anyway,
	/// no error checking or handling is done here.
	///
	/// # Thread Safety
	/// This function must only be called from the main thread.
	fn drop(&mut self)
	{
		unsafe { glfwTerminate() };
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
