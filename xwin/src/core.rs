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
//! - [platform]
//! - [cocoa_dir_resources]
//! - [cocoa_menubar]
//! - [wayland_libdecor]
//!
//! Calling any other function before successful initialization will produce
//! [NotInitialized].
//!
//! ## Initializing XWin
//! The library can be initialized with [init], which returns an [XErr] if
//! any errors occurred. If successful, this function will block until XWin is
//! terminated.
//!
//! ```
//! let xwin = xwin::init(|| {});
//! ```
//! The closure passed to [init] will be called on a new thread before the
//! function blocks. It is intended that all code dealing with XWin aside from
//! initial configuration will occur on other threads, while XWin will control
//! the main thread. Some functions within XWin require some of their code to be
//! run on the main thread. When such functions are called, XWin will handle
//! moving data to the main thread, and may block the calling thread until the
//! function is completed.
//!
//! If any part of initialization fails, any parts that succeeded are terminated
//! as if [terminate] had been called.
//!
//! **MacOS:** When the library is initialized the main menu and dock icon are
//! created. These are not desirable for a command-line only program. The
//! creation of the main menu and dock icon can be disabled with the
//! [cocoa_menubar] function.
//!
//! # Configuring XWin Initialization
//!
//! Use the functions in this struct to configure XWin before initialization,
//! concluding with [init]. These functions will affect how the library
//! behaves until termination.
//!
//! ```
//! # use xwin::Platform;
//! xwin::set_platform(Platform::Any);
//! xwin::init(|| {});
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
//! - [platform]
//! - [cocoa_dir_resources]
//! - [cocoa_menubar]
//! - [wayland_libdecor]
//!
//! ## Runtime Platform Selection
//! You can control platform selection via the [platform] function. By
//! default, this is set to [Platform::Any], which will look for supported
//! window systems in order of priority and select the first one it finds. It
//! can also be set to any specific platform to have XWin only look for that
//! one.
//!
//! ```
//! # use xwin::core::Platform;
//! # #[cfg(windows)]
//! xwin::set_platform(Platform::Windows);
//! xwin::init(|| {});
//! ```
//!
//! This mechanism also provides the [Null](Platform::Null) platform, which is
//! always supported but needs to be explicitly requested. This platform is
//! effectively a stub, emulating a window system on a single 1080p monitor, but
//! will not interact with any actual window system.
//!
//! ```
//! # use xwin::core::Platform;
//! xwin::set_platform(Platform::Null);
//! xwin::init(|| {});
//! ```
//!
//! You can test whether a library binary was compiled with support for a
//! specific platform with [XWin::platform_supported].
//! ```
//! # use xwin::core::Platform;
//! if xwin::platform_supported(Platform::X11)
//! {
//! 	xwin::set_platform(Platform::X11);
//! 	xwin::init(|| {});
//! }
//! ```
//!
//! Once XWin has been initialized, you can query which platform was selected
//! with [XWin::platform].
//! ```
//! # xwin::init(|| {
//! let platform = xwin::platform();
//! # });
//! ```
//!
//! ## Terminating XWin
//! XWin will be automatically terminated when the closure passed to
//! [XWin::init] terminates. XWin may also be terminated early by calling
//! [XWin::terminate], which will immediately close all windows and cause
//! [XWin::init] to return.
//!
//! # Guarantees and Limitations
//! This section describes the conditions under which XWin can be expected to
//! function, barring bugs in the operating system or drivers. Use of XWin
//! outside these limits may work on some platforms, or on some machines, or
//! some of the time, or on some versions of XWin, but may break at any time
//! and will not be considered a bug.
//!
//! ## Thread Safety
//! Most XWin functions require some amount of code to be called on the main
//! thread. XWin will mostly handle moving execution between threads as
//! necessary for you. The only exceptions being XWin configuration functions
//! under this module; [set_platform], [cocoa_dir_resources], [cocoa_menubar],
//! [wayland_libdecor], [platform], and [platform_supported]. Such functions
//! explicitly state in their documentation that they must be called from the
//! main thread under 'Thread Safety'
//!
//! ## Event Handling
//! XWin events are handled primarily using message passing through synchronous
//! channels. These channels are not created until you subscribe to them, at
//! which time you may specify their bound and a [Receiver] will be returned
//! which will receive the relevant events. The bound you specify is used for
//! the internal buffer of the channel. When this channel's buffer is full, any
//! subsequent event will cause the event handling thread to block until space
//! becomes available.
//!
//! This blocking may or may not also block handling of other types of events
//! (XWin makes no guarantees about which thread any events are sent from).
//! Therefore, it is highly recommended to ensure you have a large enough bound
//! and are pulling events from the receivers frequently enough to prevent such
//! blocking from occuring.
//!
//! Disconnecting the receiver (such as by dropping), or calling the related
//! unsubscribe function, will prevent XWin from attempting to send more events
//! on a given channel. This can be used to prevent blocking if you know you
//! won't be handling those events.
//!
//! Note also that only one subscription per event type may exist at a time. If
//! an event's subcribe function is called while a subscription already exists,
//! the existing channel will be quietly disconnected and a new one created.
//! This may be changed in the future, but for now XWin will only allow one
//! channel to exist for a given event type at a time.
//!
//! ## Event Order
//! The order of arrival of related events is not guaranteed to be consistent
//! across platforms. The exception is synthetic key and mouse button release
//! events, which are always delivered after the window defocus event.

pub(crate) mod exec;
pub mod image;

use std::{
	os::raw::c_int,
	panic,
	panic::{
		resume_unwind,
		UnwindSafe,
	},
	sync::{
		mpsc::{
			channel,
			Sender,
			SyncSender,
		},
		OnceLock,
		RwLock,
	},
	thread,
};

#[cfg(feature = "tracing")]
use crate::err::set_error_log;
use crate::{
	bind::{
		glfwGetPlatform,
		glfwInit,
		glfwInitHint,
		glfwPlatformSupported,
		glfwTerminate,
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
	},
	core::exec::XWinMessage,
	err::XErr,
	monitor::MonitorEvent,
};

/// Used internally by XWin for managing global state
pub(crate) static XWIN: OnceLock<RwLock<XWin>> = OnceLock::new();

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
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
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
#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct ContentScale
{
	pub x: f32,
	pub y: f32,
}

pub(crate) struct XWin
{
	xwin_tx:    Sender<XWinMessage>,
	monitor_tx: Option<SyncSender<MonitorEvent>>,
}

// TODO - glfwInitVulkanLoader

impl XWin
{
	pub(crate) fn get() -> Result<&'static RwLock<XWin>, XErr>
	{
		XWIN.get()
			.ok_or_else(|| XErr::NotInitialized(String::from("XWin has not been initialized")))
	}

	pub(crate) fn set_monitor_tx(&mut self, tx: Option<SyncSender<MonitorEvent>>)
	{
		self.monitor_tx = tx;
	}

	pub(crate) fn monitor_tx(&self) -> &Option<SyncSender<MonitorEvent>>
	{
		&self.monitor_tx
	}
}

/// Initializes the XWin library. Before most XWin functions can be used,
/// XWin must be initialized. If this function fails, it terminates XWin
/// before returning an error.
///
/// The [platform] function can be used to control which platforms are
/// considered during initialization. This also depends on which platforms
/// the library was compiled to support.
///
/// The closure passed to this function will be run on a new thread, while
/// this function enters a loop to process events generated by other parts
/// of the XWin library that must be processed on the main thread. This function
/// will not return until XWin is terminated, which happens automatically after
/// the closure passed to this function terminates. XWin may also be terminated
/// early by calling [terminate], which will immediately close all windows and
/// cause this function to return.
///
/// # Returns
/// An error if one occurs during initialization. If initialization
/// succeeds, returns `()` once XWin is terminated.
///
/// # Panics
/// If the closure passed to this function panics, XWin is immediately
/// terminated, and this function will panic.
///
/// # Errors
/// Possible errors include
/// [XErr::PlatformUnavailable], [XErr::Platform] and [XErr::Reinitialized].
///
/// # Remarks
/// - **macOS:** This function will change the current directory of the
///   application to the Contents/Resources subdirectory of the application's
///   bundle, if present. This can be disabled with the
///   [cocoa_dir_resources](cocoa_dir_resources) function.
///
/// - **macOS:** This function will create the main menu and dock icon for the
///   application. If XWin finds a `MainMenu.nib` it is loaded and assumed to
///   contain a menu bar. Otherwise a minimal menu bar is created manually with
///   common commands like `Hide`, `Quit` and `About`. The `About` entry opens a
///   minimal about dialog with information from the application's bundle. The
///   menu bar and dock icon can be disabled entirely with the
///   [cocoa_menubar](cocoa_menubar) function.
///
/// - **Wayland, X11:** If the library was compiled with support for both
///   `Wayland` and `X11`, and the [platform](platform) config is set to
///   [Platform::Any], the `XDG_SESSION_TYPE` environment variable affects which
///   platform is picked. If the environment variable is not set, or is set to
///   something other than `wayland` or `x11`, the regular detection mechanism
///   will be used instead.
///
/// - **X11:** This function will set the `LC_CTYPE` category of the application
///   locale according to the current environment if that category is still "C".
///   This is because the "C" locale breaks Unicode text input.
///
/// # Thread Safety
/// This function must be called from the main thread.
pub fn init<F>(f: F) -> Result<(), XErr>
where
	F: 'static + Send + FnOnce() + UnwindSafe,
{
	let (tx, rx) = channel();
	if let Err(_) = XWIN.set(RwLock::new(XWin {
		xwin_tx:    tx,
		monitor_tx: None,
	}))
	{
		return Err(XErr::Reinitialized);
	}

	#[cfg(feature = "tracing")]
	set_error_log();

	if unsafe { glfwInit() } != GLFW_TRUE as i32
	{
		return Err(XErr::get());
	}

	let handle = thread::Builder::new()
		.name("XWin Thread".to_string())
		.spawn(move || {
			let result = panic::catch_unwind(move || {
				f();
			});
			terminate();
			if let Err(err) = result
			{
				resume_unwind(err);
			}
		})
		.map_err(|err| XErr::Platform(err.to_string()))?;

	exec::run(rx);
	unsafe { glfwTerminate() };

	if handle.is_finished()
	{
		if let Err(err) = handle.join()
		{
			resume_unwind(err);
		}
	}

	Ok(())
}

/// Destroys all remaining windows and cursors, restores any modified gamma
/// ramps and frees any other allocated resources. Once this function is
/// called, most XWin functions will no longer be useful.
///
/// This function will not stop you from continuing to attempt to use other
/// XWin objects (windows, monitors, etc), but most will begin returning
/// errors after this is called.
///
/// This function has no effect if XWin is not initialized.
///
/// Unlike most functions which send a command to the main thread, this
/// function will not wait for a response from the main thread before
/// returning. This function may, as a result, return before XWin has
/// actually been terminated.
pub fn terminate()
{
	if let Some(xwin) = XWIN.get()
	{
		let _ = xwin.read().unwrap().post(XWinMessage::Terminate);
	}
}

/// Set the platform to use for windowing and input.
///
/// **Default:** [`Platform::Any`]
///
/// # Thread Safety
/// This function must only be called from the main thread.
pub fn set_platform(platform: Platform)
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
}

/// **MacOS Specific**
///
/// Specifies whether to set the current directory to the application to the
/// `Contents/Resources` subdirectory of the application's bundle, if
/// present. This is ignored on other platforms.
///
/// # Thread Safety
/// This function must only be called from the main thread.
pub fn cocoa_dir_resources(value: bool)
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
}

/// **MacOS Specific**
///
/// Specifies whether to create the menu bar and dock icon when XWin is
/// initialized. This applies whether the menu bar is created from a nib or
/// manually by XWin. This is ignored on other platforms.
///
/// # Thread Safety
/// This function must only be called from the main thread.
pub fn cocoa_menubar(value: bool)
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
}

/// **Wayland Specific**
///
/// specifies whether to use [libdecor](https://gitlab.freedesktop.org/libdecor/libdecor)
/// for window decorations where available. This is ignored on other
/// platforms.
///
/// # Thread Safety
/// This function must only be called from the main thread.
pub fn wayland_libdecor(value: bool)
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
}

/// This function returns the platform that was selected during
/// initialization.
///
/// # Returns
/// The currently selected platform, or an error if one occurred.
///
/// # Errors
/// Possible errors include [XErr::NotInitialized].
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

/// This function returns whether the library was compiled with support for
/// the specified platform.
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
