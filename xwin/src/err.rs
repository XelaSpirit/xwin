//! # Error Handling
//! Some GLFW functions have return values that indicate an error, but this is
//! often not very helpful when trying to figure out what happened or why it
//! occurred. Other functions have no return value reserved for errors, so error
//! notification needs a separate channel. Finally, far from all GLFW functions
//! have return values.
//!
//! When XWin encounters one of these errors, it will grab the last error from
//! GLFW and return it using [XErr]. XWin will also grab a human-readable string
//! describing the error.
//!
//! The error code indicates the general category of the error. Some error
//! codes, such as [XErr::NotInitialized] have only a single meaning, whereas
//! others like [XErr::PlatformError] are used for many different errors.
//!
//! **Reported errors are never fatal.** As long as XWin was successfully
//! initialized, it will remain initialized and in a safe state until terminated
//! regardless of how many errors occur. If an error occurs during
//! initialization that causes [XWin::new](crate::core::XWin::new) to fail, any
//! part of the library that was initialized will be safely terminated.
//!
//! Do not rely on a currently invalid call to generate a specific error, as in
//! the future that same call may generate a different error or become valid.

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
};

/// Error codes used throughout the XWin API.
#[repr(u32)]
#[derive(Debug, PartialEq, Eq)]
pub enum XErr
{
	/// No error has occurred.
	///
	/// Yay
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
	/// This occurs if an XWin function was called that needs and operates on
	/// the current OpenGL or OpenGL ES context but no context is current on
	/// the calling thread. One such function is glfwSwapInterval.
	/// TODO add link to XWin function here.
	///
	/// **Analysis**. Application programmer error. Ensure a context is current
	/// before calling functions that require a current context.
	NoCurrentContext(String) = GLFW_NO_CURRENT_CONTEXT,
	/// One of the arguments to the function was an invalid enum value.
	///
	/// For example, requesting GLFW_RED_BITS with glfwGetWindowAttrib.
	/// TODO Add link to XWin function here.
	///
	/// **Analysis**. Application programmer error. Fix the offending call.
	InvalidEnum(String)  = GLFW_INVALID_ENUM,
	/// One of the arguments to the function was an invalid value.
	///
	/// For example, requesting a non-existent OpenGL or OpenGL ES version like
	/// 2.7.
	///
	/// Requesting a valid but unavailable OpenGL or OpenGL ES version will
	/// instead result in a [VersionUnavailable](XErr::VersionUnavailable)
	/// error.
	///
	/// **Analysis**. Application programmer error. Fix the offending call.
	InvalidValue(String) = GLFW_INVALID_VALUE,
	/// A memory allocation failed.
	///
	/// **Analysis**. A bug in XWin, GLFW or the underlying operating system.
	/// Report the bug to our issue tracker.
	/// TODO where to report bugs?
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
	/// The requested OpenGL or OpenGL ES version (including any requested
	/// context or framebuffer hints) is not available on this machine.
	///
	/// **Analysis.** The machine does not support your requirements. If your
	/// application is sufficiently flexible, downgrade your requirements and
	/// try again. Otherwise, inform the user that their machine does not match
	/// your requirements.
	///
	/// Future invalid OpenGL and OpenGL ES versions, for example OpenGL 4.8 if
	/// 5.0 comes out before the 4.x series gets that far, also fail with this
	/// error and not GLFW_INVALID_VALUE, because GLFW cannot know what future
	/// versions will exist.
	VersionUnavailable(String) = GLFW_VERSION_UNAVAILABLE,
	/// A platform-specific error occurred that does not match any of the more
	/// specific categories.
	///
	/// **Analysis**. A bug or configuration error in XWin, GLFW, the underlying
	/// operating system or its drivers, or a lack of required resources.
	/// Report the issue to our issue tracker.
	/// TODO where to report bugs?
	PlatformError(String) = GLFW_PLATFORM_ERROR,
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
	/// A window that does not have an OpenGL or OpenGL ES context was passed to
	/// a function that requires it to have one.
	///
	/// **Analysis**. Application programmer error. Fix the offending call.
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
	/// **Application**. Platform or platform version limitation. The error can
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
	/// **Analysis**. A bug in XWin, report the issue to our issue tracker
	/// TODO where to report bugs?
	Unknown,
}

impl XErr
{
	pub(crate) fn from_code(code: u32, msg: String) -> XErr
	{
		match code
		{
			| GLFW_NO_ERROR => XErr::None(msg),
			| GLFW_NOT_INITIALIZED => XErr::NotInitialized(msg),
			| GLFW_NO_CURRENT_CONTEXT => XErr::NoCurrentContext(msg),
			| GLFW_INVALID_ENUM => XErr::InvalidEnum(msg),
			| GLFW_INVALID_VALUE => XErr::InvalidValue(msg),
			| GLFW_OUT_OF_MEMORY => XErr::OutOfMemory(msg),
			| GLFW_API_UNAVAILABLE => XErr::ApiUnavailable(msg),
			| GLFW_VERSION_UNAVAILABLE => XErr::VersionUnavailable(msg),
			| GLFW_PLATFORM_ERROR => XErr::PlatformError(msg),
			| GLFW_FORMAT_UNAVAILABLE => XErr::FormatUnavailable(msg),
			| GLFW_NO_WINDOW_CONTEXT => XErr::NoWindowContext(msg),
			| GLFW_CURSOR_UNAVAILABLE => XErr::CursorUnavailable(msg),
			| GLFW_FEATURE_UNAVAILABLE => XErr::FeatureUnavailable(msg),
			| GLFW_FEATURE_UNIMPLEMENTED => XErr::FeatureUnimplemented(msg),
			| GLFW_PLATFORM_UNAVAILABLE => XErr::PlatformUnavailable(msg),
			| _ => XErr::Unknown,
		}
	}
}

#[cfg(test)]
mod tests
{
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
		err::XErr,
	};

	#[test]
	fn no_error()
	{
		assert_eq!(
			XErr::from_code(GLFW_NO_ERROR, String::default()),
			XErr::None(String::default())
		);
	}

	#[test]
	fn not_initialized()
	{
		assert_eq!(
			XErr::from_code(GLFW_NOT_INITIALIZED, String::default()),
			XErr::NotInitialized(String::default())
		);
	}

	#[test]
	fn no_current_context()
	{
		assert_eq!(
			XErr::from_code(GLFW_NO_CURRENT_CONTEXT, String::default()),
			XErr::NoCurrentContext(String::default())
		);
	}

	#[test]
	fn invalid_enum()
	{
		assert_eq!(
			XErr::from_code(GLFW_INVALID_ENUM, String::default()),
			XErr::InvalidEnum(String::default())
		);
	}

	#[test]
	fn invalid_value()
	{
		assert_eq!(
			XErr::from_code(GLFW_INVALID_VALUE, String::default()),
			XErr::InvalidValue(String::default())
		);
	}

	#[test]
	fn out_of_memory()
	{
		assert_eq!(
			XErr::from_code(GLFW_OUT_OF_MEMORY, String::default()),
			XErr::OutOfMemory(String::default())
		);
	}

	#[test]
	fn api_unavailable()
	{
		assert_eq!(
			XErr::from_code(GLFW_API_UNAVAILABLE, String::default()),
			XErr::ApiUnavailable(String::default())
		);
	}

	#[test]
	fn version_unavailable()
	{
		assert_eq!(
			XErr::from_code(GLFW_VERSION_UNAVAILABLE, String::default()),
			XErr::VersionUnavailable(String::default())
		);
	}

	#[test]
	fn platform_error()
	{
		assert_eq!(
			XErr::from_code(GLFW_PLATFORM_ERROR, String::default()),
			XErr::PlatformError(String::default())
		);
	}

	#[test]
	fn format_unavailable()
	{
		assert_eq!(
			XErr::from_code(GLFW_FORMAT_UNAVAILABLE, String::default()),
			XErr::FormatUnavailable(String::default())
		);
	}

	#[test]
	fn no_window_context()
	{
		assert_eq!(
			XErr::from_code(GLFW_NO_WINDOW_CONTEXT, String::default()),
			XErr::NoWindowContext(String::default())
		);
	}

	#[test]
	fn cursor_unavailable()
	{
		assert_eq!(
			XErr::from_code(GLFW_CURSOR_UNAVAILABLE, String::default()),
			XErr::CursorUnavailable(String::default())
		);
	}

	#[test]
	fn feature_unavailable()
	{
		assert_eq!(
			XErr::from_code(GLFW_FEATURE_UNAVAILABLE, String::default()),
			XErr::FeatureUnavailable(String::default())
		);
	}

	#[test]
	fn feature_unimplemented()
	{
		assert_eq!(
			XErr::from_code(GLFW_FEATURE_UNIMPLEMENTED, String::default()),
			XErr::FeatureUnimplemented(String::default())
		);
	}

	#[test]
	fn platform_unavailable()
	{
		assert_eq!(
			XErr::from_code(GLFW_PLATFORM_UNAVAILABLE, String::default()),
			XErr::PlatformUnavailable(String::default())
		);
	}
}
