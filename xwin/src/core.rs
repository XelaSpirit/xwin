//! # Core Functionality
//!
//! This covers the core functionality of XWin, primarily initialization and
//! termination of the XWin library.

use std::{
	ffi::CStr,
	os::raw::c_char,
	ptr::null,
};

use crate::{
	bind::{
		GLFW_TRUE,
		glfwGetError,
		glfwInit,
		glfwTerminate,
	},
	err::XErr,
};

pub struct XWin(());

impl XWin
{
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
	pub fn new() -> Result<XWin, XErr>
	{
		let init = unsafe { glfwInit() };
		if init != GLFW_TRUE as i32
		{
			let mut desc: *const c_char = null();
			let code = unsafe { glfwGetError(&mut desc) };

			return Err(XErr::from_code(
				code as u32,
				if !desc.is_null()
				{
					unsafe { CStr::from_ptr(desc) }
						.to_str()
						.unwrap_or_else(|_| "")
						.to_string()
				}
				else
				{
					String::default()
				},
			));
		}

		Ok(XWin(()))
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
