//! XWin error handling.
//!
//! Some XWin functions return a [Result] which may contain an [XErr]. The enum
//! value indicates the general category of the error, while the [String] it
//! contains is set to a more human-readable description of the error.
//!
//! If XWin was built with the `tracing` feature (enabled by default), XWin will
//! also report errors as they occur using the [Tracing](https://crates.io/crates/tracing)
//! crate as warnings.
//!
//! **Reported errors are never fatal.** As long as XWin was successfully
//! initialized, it will remain initialized and in a safe state until terminated
//! regardless of how many errors occur. If an error occurs during
//! initialization that causes initialization to fail, any part of the library
//! that was initialized will be safely terminated.
//!
//! Do not rely on a currently invalid call to generate a specific error, as in
//! the future that same call may generate a different error or become valid.

use std::{
	ffi::{
		CStr,
		CString,
	},
	os::raw::{
		c_char,
		c_int,
	},
	ptr::null,
};

#[cfg(feature = "tracing")]
use tracing::{
	instrument,
	warn,
};

#[cfg(feature = "tracing")]
use crate::bind::glfwSetErrorCallback;
use crate::bind::{
	GLFW_API_UNAVAILABLE,
	GLFW_CURSOR_UNAVAILABLE,
	GLFW_FEATURE_UNAVAILABLE,
	GLFW_FEATURE_UNIMPLEMENTED,
	GLFW_FORMAT_UNAVAILABLE,
	GLFW_INVALID_ENUM,
	GLFW_INVALID_VALUE,
	GLFW_NO_CURRENT_CONTEXT,
	GLFW_NO_ERROR,
	GLFW_NO_WINDOW_CONTEXT,
	GLFW_NOT_INITIALIZED,
	GLFW_OUT_OF_MEMORY,
	GLFW_PLATFORM_ERROR,
	GLFW_PLATFORM_UNAVAILABLE,
	GLFW_VERSION_UNAVAILABLE,
	glfwGetError,
};

/// Error codes used throughout the XWin library. See [crate::error] for more
/// information.
#[repr(u32)]
#[derive(Debug, PartialEq, Eq)]
pub enum XErr
{
	/// No error has occurred. Yay
	None(String)         = GLFW_NO_ERROR,
	/// XWin has not been initialized.
	///
	/// This occurs if an XWin function was called that must nor be called
	/// unless the library is initialized.
	///
	/// **Analysis**. Application programmer error. Initialize XWin before
	/// calling any function that requires initialization.
	NotInitialized(String) = GLFW_NOT_INITIALIZED,
	/// No context is current for this thread.
	///
	/// XWin does not support OpenGL contexts, but this may occur if the
	/// [bindings feature](crate#crate-features) was used to create a window
	/// with context.
	///
	/// **Analysis**. A bug in XWin.
	NoCurrentContext(String) = GLFW_NO_CURRENT_CONTEXT,
	/// One of the arguments to a native GLFW function was an invalid enum
	/// value.
	///
	/// XWin has defined enums for its API, unlike GLFW which used `#define` for
	/// many of their constants. This should be sufficient to prevent this error
	/// from ever occuring, but may occur if the [bindings
	/// feature](crate#crate_features) is being used to directly call GLFW
	/// functions.
	///
	/// **Analysis**. A bug in XWin.
	InvalidEnum(String)  = GLFW_INVALID_ENUM,
	/// One of the arguments to the function was an invalid value.
	///
	/// **Analysis**. Application programmer error. Fix the offending call.
	InvalidValue(String) = GLFW_INVALID_VALUE,
	/// A memory allocation failed.
	///
	/// **Analysis**. A bug in XWin, GLFW or the underlying operating system.
	OutOfMemory(String)  = GLFW_OUT_OF_MEMORY,
	/// GLFW could not find support for the requested API on the system.
	///
	/// **Analysis**. The installed graphics driver does not support the
	/// requested API, or does not support it via the chosen context creation
	/// API. Below are a few examples.
	///
	/// Some pre-installed Windows graphics drivers do not support OpenGL. AMD
	/// only supports OpenGL ES via EGL, while Nvidia and Intel only support it
	/// via a WGL or GLX extension. macOS does not provide OpenGL ES at all.
	/// The Mesa EGL, OpenGL and OpenGL ES libraries do not interface
	/// with the Nvidia binary driver. Older graphics drivers do not support
	/// Vulkan.
	ApiUnavailable(String) = GLFW_API_UNAVAILABLE,
	/// The requested OpenGL or OpenGL ES version is not available.
	///
	/// XWin does not support OpenGL contexts, but this may occur if the
	/// [bindings feature](crate#crate-features) was used to create a window
	/// with context.
	///
	/// **Analysis.** A bug in XWin.
	VersionUnavailable(String) = GLFW_VERSION_UNAVAILABLE,
	/// A platform-specific error occurred that does not match any of the more
	/// specific categories.
	///
	/// **Analysis**. A bug or configuration error in XWin, GLFW, the underlying
	/// operating system or its drivers, or a lack of required resources.
	Platform(String)     = GLFW_PLATFORM_ERROR,
	/// The requested format is not supported or available.
	///
	/// If emitted during window creation, the requested pixel format is not
	/// supported.
	///
	/// If emitted when querying the clipboard, the contents of the clipboard
	/// could not be converted to the requested format.
	///
	/// **Analysis**. If emitted during window creation, one or more hard
	/// constraints did not  match any of the available pixel formats. If your
	/// application is sufficiently flexible, downgrade your requirements and
	/// try again. Otherwise, inform the user that their machine does not match
	/// your requirements. If emitted when querying the clipboard, ignore the
	/// error or report it to the user, as appropriate.
	FormatUnavailable(String) = GLFW_FORMAT_UNAVAILABLE,
	/// The specified window does not have an OpenGL or OpenGL ES context.
	///
	/// XWin does not support OpenGL contexts, but this may occur if the
	/// [bindings feature](crate#crate-features) was used to create a window
	/// with context.
	///
	/// **Analysis**. A bug in XWin.
	NoWindowContext(String) = GLFW_NO_WINDOW_CONTEXT,
	/// The specified cursor shape is not available.
	///
	/// The specified standard cursor shape is not available, either because the
	/// current platform cursor theme does not provide it or because it is not
	/// available on the platform.
	///
	/// **Analysis**. Platform or system settings limitation. Pick another
	/// standard cursor shape or create a custom cursor.
	CursorUnavailable(String) = GLFW_CURSOR_UNAVAILABLE,
	/// The requested feature is not provided by the platform.
	///
	/// The requested feature is not provided by the platform, so GLFW is unable
	/// to implement it. The documentation for each function notes if it could
	/// emit this error.
	///
	/// **Analysis**. Platform or platform version limitation. The error can
	/// be ignored unless the feature is critical to the application.
	///
	/// A function call that emits this error has no effect other than the error
	/// and updating any existing out parameters.
	FeatureUnavailable(String) = GLFW_FEATURE_UNAVAILABLE,
	/// The requested feature is not implemented for the platform.
	///
	/// **Analysis**. An incomplete implementation of XWin or GLFW for this
	/// platform, hopefully fixed in a future release. The error can be ignored
	/// unless the feature is critical to the application.
	///
	/// A function call that emits this error has no effect other than the error
	/// and updating any existing out parameters.
	FeatureUnimplemented(String) = GLFW_FEATURE_UNIMPLEMENTED,
	/// Platform unavailable or no matching platform was found.
	///
	/// If emitted during initialization, no matching platform was found. If the
	/// GLFW_PLATFORM init hint was set to GLFW_ANY_PLATFORM, XWin could not
	/// detect any of the platforms supported by this library binary, except for
	/// the Null platform. If the init hint was set to a specific platform, it
	/// is either not supported by this library binary or XWin was not able to
	/// detect it.
	///
	/// If emitted by a native access function, XWin was initialized for a
	/// different platform than the function is for.
	///
	/// **Analysis**. Failure to detect any platform usually only happens on
	/// non-macOS Unix systems, either when no window system is running or the
	/// program was run from a terminal that does not have the necessary
	/// environment variables. Fall back to a different platform if possible or
	/// notify the user that no usable platform was detected.
	///
	/// Failure to detect a specific platform may have the same cause as above
	/// or be because support for that platform was not compiled in. Call
	/// glfwPlatformSupported to check whether a specific platform is supported
	/// by a library binary.
	///
	/// TODO link hints and functions
	PlatformUnavailable(String) = GLFW_PLATFORM_UNAVAILABLE,
	/// An unknown error occurred that XWin did not expect
	///
	/// **Analysis**. A bug in XWin.
	Unknown(String),
	/// Attempted to reinitialize XWin after termination.
	///
	/// **Analysis**. Currently, XWin may only be initialized/terminated once.
	/// Future versions may remove this limitation.
	Reinitialized(String),
	// TODO - GLFW doesn't have this limitation, this is only here because of not being able to
	//        update tx in XWIN without using an unsafe mutable static. A different solution for
	//        this could be found, I just haven't done it yet.
}

impl XErr
{
	/// Returns both the GLFW error code and description of the error. Values
	/// that are not GLFW errors will all return 0.
	#[cfg(feature = "glfw")]
	pub fn to_glfw(self) -> (u32, *const c_char)
	{
		match self
		{
			| XErr::None(str) =>
			{
				(
					GLFW_NO_ERROR,
					CString::new(str)
						.expect("Unable to convert to CString")
						.into_raw(),
				)
			},
			| XErr::NotInitialized(str) =>
			{
				(
					GLFW_NOT_INITIALIZED,
					CString::new(str)
						.expect("Unable to convert to CString")
						.into_raw(),
				)
			},
			| XErr::NoCurrentContext(str) =>
			{
				(
					GLFW_NO_CURRENT_CONTEXT,
					CString::new(str)
						.expect("Unable to convert to CString")
						.into_raw(),
				)
			},
			| XErr::InvalidEnum(str) =>
			{
				(
					GLFW_INVALID_ENUM,
					CString::new(str)
						.expect("Unable to convert to CString")
						.into_raw(),
				)
			},
			| XErr::InvalidValue(str) =>
			{
				(
					GLFW_INVALID_VALUE,
					CString::new(str)
						.expect("Unable to convert to CString")
						.into_raw(),
				)
			},
			| XErr::OutOfMemory(str) =>
			{
				(
					GLFW_OUT_OF_MEMORY,
					CString::new(str)
						.expect("Unable to convert to CString")
						.into_raw(),
				)
			},
			| XErr::ApiUnavailable(str) =>
			{
				(
					GLFW_API_UNAVAILABLE,
					CString::new(str)
						.expect("Unable to convert to CString")
						.into_raw(),
				)
			},
			| XErr::VersionUnavailable(str) =>
			{
				(
					GLFW_VERSION_UNAVAILABLE,
					CString::new(str)
						.expect("Unable to convert to CString")
						.into_raw(),
				)
			},
			| XErr::Platform(str) =>
			{
				(
					GLFW_PLATFORM_ERROR,
					CString::new(str)
						.expect("Unable to convert to CString")
						.into_raw(),
				)
			},
			| XErr::FormatUnavailable(str) =>
			{
				(
					GLFW_FORMAT_UNAVAILABLE,
					CString::new(str)
						.expect("Unable to convert to CString")
						.into_raw(),
				)
			},
			| XErr::NoWindowContext(str) =>
			{
				(
					GLFW_NO_WINDOW_CONTEXT,
					CString::new(str)
						.expect("Unable to convert to CString")
						.into_raw(),
				)
			},
			| XErr::CursorUnavailable(str) =>
			{
				(
					GLFW_CURSOR_UNAVAILABLE,
					CString::new(str)
						.expect("Unable to convert to CString")
						.into_raw(),
				)
			},
			| XErr::FeatureUnavailable(str) =>
			{
				(
					GLFW_FEATURE_UNAVAILABLE,
					CString::new(str)
						.expect("Unable to convert to CString")
						.into_raw(),
				)
			},
			| XErr::FeatureUnimplemented(str) =>
			{
				(
					GLFW_FEATURE_UNIMPLEMENTED,
					CString::new(str)
						.expect("Unable to convert to CString")
						.into_raw(),
				)
			},
			| XErr::PlatformUnavailable(str) =>
			{
				(
					GLFW_PLATFORM_UNAVAILABLE,
					CString::new(str)
						.expect("Unable to convert to CString")
						.into_raw(),
				)
			},
			| XErr::Unknown(str) =>
			{
				(
					0,
					CString::new(str)
						.expect("Unable to convert to CString")
						.into_raw(),
				)
			},
			| XErr::Reinitialized(str) =>
			{
				(
					0,
					CString::new(str)
						.expect("Unable to convert to CString")
						.into_raw(),
				)
			},
		}
	}

	#[cfg(feature = "glfw")]
	pub fn from_glfw(code: c_int, msg: *const c_char) -> Self
	{
		Self::from_code(code, msg)
	}

	fn from_code(code: c_int, msg: *const c_char) -> XErr
	{
		let str = if !msg.is_null()
		{
			unsafe { CStr::from_ptr(msg) }
				.to_str()
				.unwrap_or_else(|_| "")
				.to_owned()
		}
		else
		{
			String::default()
		};

		match code as u32
		{
			| GLFW_NO_ERROR => XErr::None(str),
			| GLFW_NOT_INITIALIZED => XErr::NotInitialized(str),
			| GLFW_NO_CURRENT_CONTEXT => XErr::NoCurrentContext(str),
			| GLFW_INVALID_ENUM => XErr::InvalidEnum(str),
			| GLFW_INVALID_VALUE => XErr::InvalidValue(str),
			| GLFW_OUT_OF_MEMORY => XErr::OutOfMemory(str),
			| GLFW_API_UNAVAILABLE => XErr::ApiUnavailable(str),
			| GLFW_VERSION_UNAVAILABLE => XErr::VersionUnavailable(str),
			| GLFW_PLATFORM_ERROR => XErr::Platform(str),
			| GLFW_FORMAT_UNAVAILABLE => XErr::FormatUnavailable(str),
			| GLFW_NO_WINDOW_CONTEXT => XErr::NoWindowContext(str),
			| GLFW_CURSOR_UNAVAILABLE => XErr::CursorUnavailable(str),
			| GLFW_FEATURE_UNAVAILABLE => XErr::FeatureUnavailable(str),
			| GLFW_FEATURE_UNIMPLEMENTED => XErr::FeatureUnimplemented(str),
			| GLFW_PLATFORM_UNAVAILABLE => XErr::PlatformUnavailable(str),
			| _ => XErr::Unknown(String::from("Unknown error")),
		}
	}

	/// Returns the latest GLFW error. May be [XErr::None] if no error has
	/// occurred.
	pub(crate) fn get() -> XErr
	{
		let mut desc: *const c_char = null();
		let code = unsafe { glfwGetError(&mut desc) };

		XErr::from_code(code, desc)
	}

	/// Returns `Err` if any GLFW errors have occurred. Otherwise, returns
	/// `Ok(f())`.
	pub(crate) fn result<T, F>(f: F) -> Result<T, XErr>
	where
		F: FnOnce() -> T,
	{
		match XErr::get()
		{
			| XErr::None(_) => Ok(f()),
			| err => Err(err),
		}
	}
}

#[cfg(feature = "tracing")]
#[instrument(level = "warn", skip_all)]
extern "C" fn glfw_error_handler(code: c_int, desc: *const c_char)
{
	warn!(
		"XWin encountered an error: {:?}",
		XErr::from_code(code, desc)
	);
}

#[cfg(feature = "tracing")]
pub(crate) fn set_error_log()
{
	unsafe { glfwSetErrorCallback(Some(glfw_error_handler)) };
}

#[cfg(test)]
mod tests
{
	use std::{
		ffi::CString,
		os::raw::c_int,
	};

	use crate::{
		bind::{
			GLFW_API_UNAVAILABLE,
			GLFW_CURSOR_UNAVAILABLE,
			GLFW_FEATURE_UNAVAILABLE,
			GLFW_FEATURE_UNIMPLEMENTED,
			GLFW_FORMAT_UNAVAILABLE,
			GLFW_INVALID_ENUM,
			GLFW_INVALID_VALUE,
			GLFW_NO_CURRENT_CONTEXT,
			GLFW_NO_ERROR,
			GLFW_NO_WINDOW_CONTEXT,
			GLFW_NOT_INITIALIZED,
			GLFW_OUT_OF_MEMORY,
			GLFW_PLATFORM_ERROR,
			GLFW_PLATFORM_UNAVAILABLE,
			GLFW_VERSION_UNAVAILABLE,
		},
		error::XErr,
	};

	#[test]
	fn no_error()
	{
		let str = CString::new("").unwrap();
		assert_eq!(
			XErr::from_code(GLFW_NO_ERROR as c_int, str.as_ptr()),
			XErr::None(String::default())
		);
	}

	#[test]
	fn not_initialized()
	{
		let str = CString::new("").unwrap();
		assert_eq!(
			XErr::from_code(GLFW_NOT_INITIALIZED as c_int, str.as_ptr()),
			XErr::NotInitialized(String::default())
		);
	}

	#[test]
	fn no_current_context()
	{
		let str = CString::new("").unwrap();
		assert_eq!(
			XErr::from_code(GLFW_NO_CURRENT_CONTEXT as c_int, str.as_ptr()),
			XErr::NoCurrentContext(String::default())
		);
	}

	#[test]
	fn invalid_enum()
	{
		let str = CString::new("").unwrap();
		assert_eq!(
			XErr::from_code(GLFW_INVALID_ENUM as c_int, str.as_ptr()),
			XErr::InvalidEnum(String::default())
		);
	}

	#[test]
	fn invalid_value()
	{
		let str = CString::new("").unwrap();
		assert_eq!(
			XErr::from_code(GLFW_INVALID_VALUE as c_int, str.as_ptr()),
			XErr::InvalidValue(String::default())
		);
	}

	#[test]
	fn out_of_memory()
	{
		let str = CString::new("").unwrap();
		assert_eq!(
			XErr::from_code(GLFW_OUT_OF_MEMORY as c_int, str.as_ptr()),
			XErr::OutOfMemory(String::default())
		);
	}

	#[test]
	fn api_unavailable()
	{
		let str = CString::new("").unwrap();
		assert_eq!(
			XErr::from_code(GLFW_API_UNAVAILABLE as c_int, str.as_ptr()),
			XErr::ApiUnavailable(String::default())
		);
	}

	#[test]
	fn version_unavailable()
	{
		let str = CString::new("").unwrap();
		assert_eq!(
			XErr::from_code(GLFW_VERSION_UNAVAILABLE as c_int, str.as_ptr()),
			XErr::VersionUnavailable(String::default())
		);
	}

	#[test]
	fn platform_error()
	{
		let str = CString::new("").unwrap();
		assert_eq!(
			XErr::from_code(GLFW_PLATFORM_ERROR as c_int, str.as_ptr()),
			XErr::Platform(String::default())
		);
	}

	#[test]
	fn format_unavailable()
	{
		let str = CString::new("").unwrap();
		assert_eq!(
			XErr::from_code(GLFW_FORMAT_UNAVAILABLE as c_int, str.as_ptr()),
			XErr::FormatUnavailable(String::default())
		);
	}

	#[test]
	fn no_window_context()
	{
		let str = CString::new("").unwrap();
		assert_eq!(
			XErr::from_code(GLFW_NO_WINDOW_CONTEXT as c_int, str.as_ptr()),
			XErr::NoWindowContext(String::default())
		);
	}

	#[test]
	fn cursor_unavailable()
	{
		let str = CString::new("").unwrap();
		assert_eq!(
			XErr::from_code(GLFW_CURSOR_UNAVAILABLE as c_int, str.as_ptr()),
			XErr::CursorUnavailable(String::default())
		);
	}

	#[test]
	fn feature_unavailable()
	{
		let str = CString::new("").unwrap();
		assert_eq!(
			XErr::from_code(GLFW_FEATURE_UNAVAILABLE as c_int, str.as_ptr()),
			XErr::FeatureUnavailable(String::default())
		);
	}

	#[test]
	fn feature_unimplemented()
	{
		let str = CString::new("").unwrap();
		assert_eq!(
			XErr::from_code(GLFW_FEATURE_UNIMPLEMENTED as c_int, str.as_ptr()),
			XErr::FeatureUnimplemented(String::default())
		);
	}

	#[test]
	fn platform_unavailable()
	{
		let str = CString::new("").unwrap();
		assert_eq!(
			XErr::from_code(GLFW_PLATFORM_UNAVAILABLE as c_int, str.as_ptr()),
			XErr::PlatformUnavailable(String::default())
		);
	}
}
