//! XWin core functionality.
//!
//! TODO documentation

pub(crate) mod exec;
pub mod image;

use std::{
	os::raw::c_int,
	panic,
	panic::{
		UnwindSafe,
		resume_unwind,
	},
	sync::{
		OnceLock,
		RwLock,
		mpsc,
		mpsc::channel,
	},
	thread,
};

use xch::Sender;

#[cfg(feature = "tracing")]
use crate::error::set_error_log;
use crate::{
	bind::{
		GLFW_ANY_PLATFORM,
		GLFW_COCOA_CHDIR_RESOURCES,
		GLFW_COCOA_MENUBAR,
		GLFW_FALSE,
		GLFW_JOYSTICK_HAT_BUTTONS,
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
		glfwGetTime,
		glfwGetTimerFrequency,
		glfwGetTimerValue,
		glfwInit,
		glfwInitHint,
		glfwPlatformSupported,
		glfwSetTime,
		glfwTerminate,
	},
	core::exec::XWinMessage,
	error::XErr,
	input::event::{
		JoystickConfigEvent,
		set_joystick_callback,
	},
	monitor::{
		MonitorEvent,
		set_monitor_callback,
	},
};

/// Used internally by XWin for managing global state
pub(crate) static XWIN: OnceLock<RwLock<XWin>> = OnceLock::new();

/// Used to configure XWin. Specifies the platform to use for windowing and
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
pub struct ScreenCoordinates<T>
{
	pub x: T,
	pub y: T,
}

/// Almost all positions and sizes in XWin are measured in
/// [ScreenCoordinates]. However, framebuffer sizes
/// are measured in pixels.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct Pixels
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
	joystick_tx: Option<Box<dyn Sender<JoystickConfigEvent> + Send + Sync>>,
	monitor_tx:  Option<Box<dyn Sender<MonitorEvent> + Send + Sync>>,
	xwin_tx:     mpsc::Sender<XWinMessage>,
}

// TODO - glfwInitVulkanLoader

impl XWin
{
	pub(crate) fn get() -> Result<&'static RwLock<XWin>, XErr>
	{
		XWIN.get()
			.ok_or_else(|| XErr::NotInitialized(String::from("XWin has not been initialized")))
	}

	pub(crate) fn set_joystick_tx<T>(&mut self, tx: T)
	where
		T: Sender<JoystickConfigEvent> + Send + Sync + 'static,
	{
		self.joystick_tx = Some(Box::new(tx));
	}

	pub(crate) fn remove_joystick_tx(&mut self)
	{
		self.joystick_tx = None;
	}

	pub(crate) fn joystick_tx(&self)
	-> Option<&Box<dyn Sender<JoystickConfigEvent> + Send + Sync>>
	{
		self.joystick_tx.as_ref()
	}

	pub(crate) fn set_monitor_tx<T>(&mut self, tx: T)
	where
		T: Sender<MonitorEvent> + Send + Sync + 'static,
	{
		self.monitor_tx = Some(Box::new(tx));
	}

	pub(crate) fn remove_monitor_tx(&mut self)
	{
		self.monitor_tx = None;
	}

	pub(crate) fn monitor_tx(&self) -> Option<&Box<dyn Sender<MonitorEvent> + Send + Sync>>
	{
		self.monitor_tx.as_ref()
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
///   bundle, if present. This can be disabled with the [cocoa_dir_resources]
///   function.
///
/// - **macOS:** This function will create the main menu and dock icon for the
///   application. If XWin finds a `MainMenu.nib` it is loaded and assumed to
///   contain a menu bar. Otherwise a minimal menu bar is created manually with
///   common commands like `Hide`, `Quit` and `About`. The `About` entry opens a
///   minimal about dialog with information from the application's bundle. The
///   menu bar and dock icon can be disabled entirely with the [cocoa_menubar]
///   function.
///
/// - **Wayland, X11:** If the library was compiled with support for both
///   `Wayland` and `X11`, and the [platform] config is set to [Platform::Any],
///   the `XDG_SESSION_TYPE` environment variable affects which platform is
///   picked. If the environment variable is not set, or is set to something
///   other than `wayland` or `x11`, the regular detection mechanism will be
///   used instead.
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
		joystick_tx: None,
		monitor_tx:  None,
		xwin_tx:     tx,
	}))
	{
		return Err(XErr::Reinitialized);
	}

	#[cfg(feature = "tracing")]
	set_error_log();

	unsafe { glfwInitHint(GLFW_JOYSTICK_HAT_BUTTONS as c_int, GLFW_FALSE as c_int) };

	if unsafe { glfwInit() } != GLFW_TRUE as i32
	{
		return Err(XErr::get());
	}

	set_monitor_callback();
	set_joystick_callback();

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

/// See [try_clipboard_string].
pub fn clipboard_string() -> String
{
	try_clipboard_string().unwrap_or_default()
}

/// See [try_set_clipboard_string].
pub fn set_clipboard_string(value: String)
{
	let _ = try_set_clipboard_string(value);
}

/// See [try_set_time].
pub fn set_time(value: f64)
{
	let _ = try_set_time(value);
}

/// See [try_time].
pub fn time() -> f64
{
	try_time().unwrap_or_default()
}

/// See [try_timer_frequency].
pub fn timer_frequency() -> u64
{
	try_timer_frequency().unwrap_or_default()
}

/// See [try_timer_value].
pub fn timer_value() -> u64
{
	try_timer_value().unwrap_or_default()
}

/// Returns the contents of the system clipboard, if it contains or is
/// convertible to a UTF-8 encoded string. If the clipboard is empty or if its
/// contents cannot be converted, [XErr::FormatUnavailable] is returned.
///
/// # Errors
/// Possible errors include [XErr::NotInitialized], [XErr::FormatUnavailable],
/// and [XErr::Platform].
///
/// # Remarks
/// **Win32**: The clipboard on Windows has a single global lock for reading and
/// writing. XWin tries to acquire it a few times, which is almost always
/// enough. If it cannot acquire the lock then this function returns
/// [XErr::Platform]. It is safe to try this multiple times.
pub fn try_clipboard_string() -> Result<String, XErr>
{
	let (tx, rx) = channel();
	XWin::get()?
		.read()
		.unwrap()
		.post_rcv(XWinMessage::GetClipboardString(tx), rx)?
}

/// Sets the system clipboard to the specified [String].
///
/// # Errors
/// Possible errors include [XErr::NotInitialized].
///
/// # Remarks
/// **Win32**: The clipboard on Windows has a single global lock for reading and
/// writing. XWin tries to acquire it a few times, which is almost always
/// enough. If it cannot acquire the lock then this function returns
/// [XErr::Platform]. It is safe to try this multiple times.
pub fn try_set_clipboard_string(value: String) -> Result<(), XErr>
{
	let (tx, rx) = channel();
	XWin::get()?
		.read()
		.unwrap()
		.post_rcv(XWinMessage::SetClipboardString(value, tx), rx)?
}

/// Sets the current XWin time, in seconds. The value must be a positive finite
/// number less than or equal to `18446744073.0`, which is approximately 584.5
/// yearsa.
///
/// This function and [try_get_time] are helper functions on top of
/// [try_timer_frequency] and [try_timer_value].
///
/// # Thread Safety
/// Reading and writing of the internal base time is not atomic, so it needs to
/// be externally synchronized with calls to [try_set_time].
///
/// # Errors
/// Possible errors include [XErr::NotInitialized].
///
/// # Remarks
/// The upper limit of XWin time is calculated as `floor((2^64 - 1) / 10^9)` and
/// is due to implementations storing nanoseconds in 64 bits. The limit may be
/// increased in the future.
pub fn try_set_time(value: f64) -> Result<(), XErr>
{
	unsafe { glfwSetTime(value) };
	XErr::result(|| ())
}

/// Returns the current XWin time, in seconds. Unless the time has been set
/// using [try_set_time] it measures the time elapsed since XWin was
/// initialized.
///
/// This function and [try_set_time] are helper functions on top of
/// [try_timer_frequency] and [try_timer_value].
///
/// The resolution of the timer is system dependent, but is usually on the order
/// of a few micro- or nanoseconds. It uses the highest-resolution monotonic
/// time source on each operating system.
///
/// # Thread Safety
/// Reading and writing of the internal base time is not atomic, so it needs to
/// be externally synchronized with calls to [try_set_time].
///
/// # Errors
/// Possible errors include [XErr::NotInitialized].
pub fn try_time() -> Result<f64, XErr>
{
	let time = unsafe { glfwGetTime() };
	XErr::result(|| time)
}

/// Returns the frequency, in Hz, of the raw timer.
///
/// # Errors
/// Possible errors include [XErr::NotInitialized].
pub fn try_timer_frequency() -> Result<u64, XErr>
{
	let time = unsafe { glfwGetTimerFrequency() };
	XErr::result(|| time)
}

/// Returns the current value of the raw timer, measured in `1/frequency`
/// seconds. To get the frequency, call [try_timer_frequency].
///
/// # Errors
/// Possible errors include [XErr::NotInitialized].
pub fn try_timer_value() -> Result<u64, XErr>
{
	let time = unsafe { glfwGetTimerValue() };
	XErr::result(|| time)
}
