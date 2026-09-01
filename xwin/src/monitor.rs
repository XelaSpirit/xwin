//! Module containing monitor related functions and types
//!
//! # Monitor Objects
//! A [Monitor] object represents a currently connected monitor. [Monitor]
//! objects cannot be created or destroyed by the application and retain their
//! data until the monitors they represent are disconnected or until the library
//! is terminated.
//!
//! Each monitor has a current video mode, a list of supported video modes, a
//! virtual position, a human-readable name, an estimated physical size and a
//! gamma ramp. One of the monitors is the primary monitor.
//!
//! The virtual position of a monitor is in [ScreenCoordinates] and, together
//! with the current video mode, describes the viewports that the connected
//! monitors provide into the virtual desktop that spans them.
//!
//! # Retrieving Monitors
//! The primary monitor is returned by [Monitor::primary]. It is the user's
//! preferred monitor and is usually the one with global UI elements like task
//! bar or menu bar.
//!
//! ```
//! # use xwin::core::XWin;
//! # use xwin::monitor::Monitor;
//! # let xwin = XWin::default();
//! #
//! let primary = Monitor::primary();
//! ```
//!
//! You can retrieve all currently connected monitors with [Monitor::all].
//!
//! ```
//! # use xwin::core::XWin;
//! # use xwin::monitor::Monitor;
//! # let xwin = XWin::default();
//! #
//! let monitors = Monitor::all();
//! ```
//!
//! The primary monitor is always the first monitor in the returned [Vec], but
//! other monitors may be moved to a different index when a monitor is connected
//! or disconnected.
//!
//! # Monitor Configuration Changes
//! If you wish to be notified when a monitor is connected or disconnected, set
//! a monitor callback.
//!
//! ```
//! # use xwin::core::XWin;
//! # use xwin::monitor::{add_monitor_callback, Monitor, MonitorEvent};
//! # let xwin = XWin::default();
//! #
//! fn monitor_callback(monitor: &Monitor, evt: MonitorEvent)
//! {
//! 	match evt
//! 	{
//! 		| MonitorEvent::Connected =>
//! 		{ /* Monitor was connected */ },
//! 		| MonitorEvent::Disconnected =>
//! 		{ /* Monitor was disconnected */ },
//! 	}
//! }
//! add_monitor_callback(monitor_callback);
//! ```
//!
//! If a monitor is disconnected, all windows that are full screen on it will be
//! switched to windowed mode before the callback is called. Only
//! [Monitor::name] and [Monitor::userdata] will return useful values for a
//! disconnected monitor and only before the monitor callback returns.
//!
//! # Monitor Properties
//! Each monitor has a current video mode, a list of supported video modes, a
//! virtual position, a content scale, a human-readable name, a user pointer, an
//! estimated physical size and a gamma ramp.
//!
//! ## Video Modes
//! XWin generally does a good job selecting a suitable video mode when you
//! create a full screen window, change its video mode or make a windowed one
//! full screen, but it is sometimes useful to know exactly which video modes
//! are supported.
//!
//! Video modes are represented with the [VideoMode] structure. You can get a
//! [Vec] of the video modes supported by a monitor with [Monitor::video_modes].
//!
//! ```
//! # use xwin::core::XWin;
//! # use xwin::monitor::Monitor;
//! # let xwin = XWin::default();
//! #
//! let primary = Monitor::primary();
//! # if false { // GLFW doesn't seem to initialize in doc tests
//! let video_modes = primary.unwrap().video_modes().unwrap();
//! # }
//! ```
//!
//! To get the current video mode of a monitor call [Monitor::video_mode].
//!
//! ```
//! # use xwin::core::XWin;
//! # use xwin::monitor::Monitor;
//! # let xwin = XWin::default();
//! #
//! let primary = Monitor::primary();
//! # if false { // GLFW doesn't seem to initialize in doc tests
//! let video_mode = primary.unwrap().video_mode().unwrap();
//! # }
//! ```
//!
//! The resolution of a video mode is specified in [ScreenCoordinates], not
//! pixels.
//!
//! ## Physical Size
//! The physical size of a monitor in millimeters, or an estimation of it, can
//! be retrieved with [Monitor::physical_size]. This has no relation to its
//! current *resolution*, i.e. the width and height of its current [VideoMode].
//!
//! ```
//! # use xwin::core::XWin;
//! # use xwin::monitor::Monitor;
//! # let xwin = XWin::default();
//! #
//! let primary = Monitor::primary();
//! # if false { // GLFW doesn't seem to initialize in doc tests
//! let (width_mm, height_mm) = primary.unwrap().physical_size().unwrap();
//! # }
//! ```
//!
//! While this can be used to calculate the raw DPI of a monitor, this is often
//! not useful. Instead, use the [monitor content scale](Monitor::content_scale)
//! and [window content scale](xwin::window::Window::content_scale) to scale
//! your content.
//!
//! ## Content scale
//! The content scale for a monitor can be retrieved with
//! [Monitor::content_scale].
//!
//! ```
//! # use xwin::core::XWin;
//! # use xwin::monitor::Monitor;
//! # let xwin = XWin::default();
//! #
//! let primary = Monitor::primary();
//! # if false { // GLFW doesn't seem to initialize in doc tests
//! let content_scale = primary.unwrap().content_scale().unwrap();
//! # }
//! ```
//!
//! For more information on what the content scale is and how to use it, see
//! [window content scale]()
//!
//! TODO - link window content scale
//!
//! ## Virtual Position
//! The position of the monitor on the virtual desktop, in [ScreenCoordinates],
//! can be retrieved with [Monitor::position].
//!
//! ```
//! # use xwin::core::XWin;
//! # use xwin::monitor::Monitor;
//! # let xwin = XWin::default();
//! #
//! let primary = Monitor::primary();
//! # if false { // GLFW doesn't seem to initialize in doc tests
//! let position = primary.unwrap().position().unwrap();
//! # }
//! ```
//!
//! ## Work Area
//! The area of a monitor not occupied by global task bars or menu bars is the
//! work area. This is specified in [ScreenCoordinates] and can be retrieved
//! with [Monitor::work_area].
//!
//! ```
//! # use xwin::core::XWin;
//! # use xwin::monitor::Monitor;
//! # let xwin = XWin::default();
//! #
//! let primary = Monitor::primary();
//! # if false { // GLFW doesn't seem to initialize in doc tests
//! let work_area = primary.unwrap().work_area().unwrap();
//! # }
//! ```
//!
//! ## Human-Readable Name
//! The human-readable name of a monitor is returned by [Monitor::name].
//!
//! ```
//! # use xwin::core::XWin;
//! # use xwin::monitor::Monitor;
//! # let xwin = XWin::default();
//! #
//! let primary = Monitor::primary();
//! # if false { // GLFW doesn't seem to initialize in doc tests
//! let name = primary.unwrap().name().unwrap();
//! # }
//! ```
//!
//! ## User Data
//! Each monitor has a userdata value that can be set with
//! [Monitor::set_userdata] and queried with [Monitor::userdata]. This can be
//! used for any purpose you need and will not be modified by XWin. The value
//! will be kept until the monitor is disconnected or until the library is
//! terminated. This means that even if the [Monitor] is dropped, and later
//! re-retrieved via XWin, the userdata value will still be set so long as the
//! monitor was never disconnected.
//!
//! The initial value of the userdata is `0`.
//!
//! ## Gamma Ramp
//! The gamma ramp of a monitor can be set with [Monitor::set_gamma_ramp], which
//! accepts a monitor handle and a pointer to a [GammaRamp] structure.
//!
//! ```
//! # use xwin::core::XWin;
//! # use xwin::monitor::{GammaRamp, Monitor};
//! # let xwin = XWin::default();
//! #
//! let mut ramp = GammaRamp::new(256, 0);
//!
//! for idx in 0..256
//! {
//! 	// Fill out gamma ramp values as desired
//! }
//!
//! let primary = Monitor::primary();
//! # if false { // GLFW doesn't seem to initialize in doc tests
//! let name = primary.unwrap().set_gamma_ramp(&mut ramp);
//! # }
//! ```
//!
//! It is recommended that your gamma ramp have the same size as the current
//! gamma ramp for that monitor.
//!
//! The current gamma ramp for a monitor is returned by [Monitor::gamma_ramp].
//!
//! ```
//! # use xwin::core::XWin;
//! # use xwin::monitor::Monitor;
//! # let xwin = XWin::default();
//! #
//! let primary = Monitor::primary();
//! # if false { // GLFW doesn't seem to initialize in doc tests
//! let gamma_ramp = primary.unwrap().gamma_ramp().unwrap();
//! # }
//! ```
//!
//! If you wish to set a regular gamma ramp, you can have XWin calculate it for
//! you from the desired exponent with [Monitor::set_gamma], which in turn calls
//! [Monitor::set_gamma_ramp] with the resulting ramp.
//!
//! ```
//! # use xwin::core::XWin;
//! # use xwin::monitor::Monitor;
//! # let xwin = XWin::default();
//! #
//! let primary = Monitor::primary();
//! # if false { // GLFW doesn't seem to initialize in doc tests
//! let gamma_ramp = primary.unwrap().set_gamma(1.0);
//! # }
//! ```

mod callback;
mod gamma_ramp;
mod video_mode;
mod work_area;

use std::{
	ffi::CStr,
	os::raw::c_void,
};

pub use callback::*;
pub use gamma_ramp::*;
pub use video_mode::*;
pub use work_area::*;

use crate::{
	bind::{
		glfwGetGammaRamp,
		glfwGetMonitorContentScale,
		glfwGetMonitorName,
		glfwGetMonitorPhysicalSize,
		glfwGetMonitorPos,
		glfwGetMonitorUserPointer,
		glfwGetMonitorWorkarea,
		glfwGetMonitors,
		glfwGetPrimaryMonitor,
		glfwGetVideoMode,
		glfwGetVideoModes,
		glfwSetGamma,
		glfwSetGammaRamp,
		glfwSetMonitorUserPointer,
		GLFWmonitor,
	},
	core::ScreenCoordinates,
	err::XErr,
};

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Monitor(*mut GLFWmonitor);

impl Monitor
{
	/// Returns a [Vec] containing all currently connected monitors. The primary
	/// monitor is always first in the returned list.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized].
	///
	/// # Thread Safety
	/// This function must only be called from the main thread.
	///
	/// # See Also
	/// - [Monitor::primary]
	pub fn all() -> Result<Vec<Monitor>, XErr>
	{
		let mut count = 0i32;
		let monitors: *mut *mut GLFWmonitor = unsafe { glfwGetMonitors(&mut count) };
		XErr::result(|| {
			let mut arr = Vec::<Monitor>::with_capacity(count as usize);
			for idx in 0..count as usize
			{
				arr.push(Monitor(unsafe { *monitors.add(idx) }));
			}
			arr
		})
	}

	/// Returns the primary monitor. This is usually the monitor where elements
	/// like the task bar or global menu bar are located.
	///
	/// # Errors
	/// Returns [XErr::None] if no monitors were found. Other possible errors
	/// include [XErr::NotInitialized].
	///
	/// # Thread Safety
	/// This function must only be called from the main thread.
	///
	/// # Remarks
	/// The primary monitor is always first in the [Vec] returned by
	/// [Monitor::all]
	pub fn primary() -> Result<Monitor, XErr>
	{
		let monitor = Monitor(unsafe { glfwGetPrimaryMonitor() });

		// Null may mean no monitor was found, but we'll still report it as XErr::None
		if monitor.0.is_null()
		{
			Err(XErr::get())
		}
		else
		{
			Ok(monitor)
		}
	}

	/// Returns the position `(x, y)`, in **screen coordinates**, of the
	/// upper-left corner of this monitor.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized] and [XErr::Platform].
	///
	/// # Thread Safety
	/// This function must only be called from the main thread.
	pub fn position(&self) -> Result<ScreenCoordinates, XErr>
	{
		let mut pos = ScreenCoordinates::default();
		unsafe { glfwGetMonitorPos(self.0, &mut pos.x, &mut pos.y) };
		XErr::result(|| pos)
	}

	/// Returns the position, in screen coordinates, of the upper-left corner of
	/// the work area of the specified monitor along with the work area size in
	/// screen coordinates. The work area is defined as the area of the monitor
	/// not occluded by the window system task bar where present. If no task
	/// bar exists then the work area is the monitor resolution in screen
	/// coordinates.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized] and [XErr::Platform].
	///
	/// # Thread Safety
	/// This function must only be called from the main thread.
	///
	/// # See Also
	/// - [WorkArea]
	pub fn work_area(&self) -> Result<WorkArea, XErr>
	{
		let mut area = WorkArea::default();

		unsafe {
			glfwGetMonitorWorkarea(
				self.0,
				&mut area.pos.x,
				&mut area.pos.y,
				&mut area.size.x,
				&mut area.size.y,
			)
		};

		XErr::result(|| area)
	}

	/// Returns the size `(width, height)`, in millimetres, of the display area
	/// of this monitor.
	///
	/// Some platforms do not provide accurate monitor size information, either
	/// because the monitor EDID data is incorrect or because the driver does
	/// not report it accurately.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized].
	///
	/// # Remarks
	/// **Windows**: On Windows 8 and earlier the physical size is calculated
	/// from the current resolution and system DPI instead of querying the
	/// monitor EDID data.
	///
	/// # Thread safety
	/// This function must only be called from the main thread.
	pub fn physical_size(&self) -> Result<(i32, i32), XErr>
	{
		let mut width = 0i32;
		let mut height = 0i32;

		unsafe { glfwGetMonitorPhysicalSize(self.0, &mut width, &mut height) };
		XErr::result(|| (width, height))
	}

	/// Returns the content scale `(xscale, yscale)` for the specified monitor.
	/// The content scale is the ratio between the current DPI and the
	/// platform's default DPI. This is especially important for text and any
	/// UI elements. If the pixel dimensions of your UI scaled by this look
	/// appropriate on your machine then it should appear at a reasonable size
	/// on other machines regardless of their DPI and scaling settings. This
	/// relies on the system DPI and scaling settings being somewhat correct.
	///
	/// The content scale may depend on both the monitor resolution and pixel
	/// density and on user settings. It may be very different from the raw DPI
	/// calculated from the physical size and current resolution.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized] and [XErr::Platform].
	///
	/// # Remarks
	/// **Wayland**: Fractional scaling information is not yet available for
	/// monitors, so this function only returns integer content scales.
	///
	/// # Thread Safety
	/// This function must only be called from the main thread.
	pub fn content_scale(&self) -> Result<(f32, f32), XErr>
	{
		let mut xscale = 0.0f32;
		let mut yscale = 0.0f32;

		unsafe { glfwGetMonitorContentScale(self.0, &mut xscale, &mut yscale) };
		XErr::result(|| (xscale, yscale))
	}

	/// Returns a human-readable name, encoded as UTF-8, of this monitor. The
	/// name typically reflects the make and model of the monitor and is not
	/// guaranteed to be unique among the connected monitors.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized].
	///
	/// # Thread Safety
	/// This function must only be called from the main thread.
	pub fn name(&self) -> Result<String, XErr>
	{
		let title = unsafe { glfwGetMonitorName(self.0) };

		if title.is_null()
		{
			Err(XErr::get())
		}
		else
		{
			Ok(unsafe { CStr::from_ptr(title) }
				.to_str()
				.unwrap_or_else(|_| "")
				.to_owned())
		}
	}

	/// Sets the user-defined pointer of this monitor. The current value is
	/// retained until the monitor is disconnected. The initial value is `0`.
	///
	/// This function may be called from the monitor callback, even for a
	/// monitor that is being disconnected.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized].
	///
	/// # Thread Safety
	/// This function may be called from any thread. Access to userdata is not
	/// synchronized.
	///
	/// # See Also
	/// - [Monitor::userdata]
	pub fn set_userdata(&self, userdata: usize) -> Result<(), XErr>
	{
		let data = userdata as *mut c_void;
		unsafe { glfwSetMonitorUserPointer(self.0, data) };
		XErr::result(|| ())
	}

	/// This function returns the current userdata of this monitor. The initial
	/// value is 0.
	///
	/// This function may be called from the monitor callback, even for a
	/// monitor that is being disconnected.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized].
	///
	/// # Thread Safety
	/// This function may be called from any thread. Access to userdata is not
	/// synchronized.
	///
	/// # See Also
	/// - [Monitor::set_userdata]
	pub fn userdata(&self) -> Result<usize, XErr>
	{
		let data = unsafe { glfwGetMonitorUserPointer(self.0) };
		XErr::result(|| data as usize)
	}

	/// This function returns a [Vec] of all video modes supported by this
	/// monitor. The returned list is sorted in ascending order, first by color
	/// bit depth (the sum of all channel depths), then by resolution area (the
	/// product of width and height), then resolution width and finally by
	/// refresh rate.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized] and [XErr::Platform].
	///
	/// # Thread Safety
	/// This function must only be called from the main thread.
	///
	/// # See Also
	/// - [Monitor::video_mode]
	pub fn video_modes(&self) -> Result<Vec<VideoMode>, XErr>
	{
		let mut count = 0i32;
		let vms = unsafe { glfwGetVideoModes(self.0, &mut count) };

		if vms.is_null()
		{
			Err(XErr::get())
		}
		else
		{
			let mut arr = Vec::<VideoMode>::with_capacity(count as usize);
			for idx in 0..count as usize
			{
				arr.push(VideoMode::from_glfw(unsafe { &*vms.add(idx) }));
			}
			Ok(arr)
		}
	}

	/// This function returns the current video mode of this monitor. If you
	/// have created a full screen window for that monitor, the return value
	/// will depend on whether that window is iconified.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized] and [XErr::Platform].
	///
	/// # Thread Safety
	/// This function must only be called from the main thread.
	///
	/// # See Also
	/// - [Monitor::video_modes]
	pub fn video_mode(&self) -> Result<VideoMode, XErr>
	{
		let vm_ptr = unsafe { glfwGetVideoMode(self.0) };

		match unsafe { vm_ptr.as_ref() }
		{
			| None => Err(XErr::get()),
			| Some(vm) => Ok(VideoMode::from_glfw(vm)),
		}
	}

	/// This function generates an appropriately sized gamma ramp from the
	/// specified exponent and then sets the gamma ramp of this monitor to it.
	/// The value must be a finite number greater than zero.
	///
	/// The software controlled gamma ramp is applied in addition to the
	/// hardware gamma correction, which today is usually an approximation of
	/// sRGB gamma. This means that setting a perfectly linear ramp, or gamma
	/// `1.0`, will produce the default (usually sRGB-like) behavior.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized], [XErr::InvalidValue],
	/// [XErr::Platform], and [XErr::FeatureUnavailable] (see remarks).
	///
	/// # Remarks
	/// **Wayland**: Gamma handling is a privileged protocol, this function will
	/// thus never be implemented and returns [XErr::FeatureUnavailable].
	///
	/// # Thread Safety
	/// This function must only be called from the main thread.
	pub fn set_gamma(&self, gamma: f32) -> Result<(), XErr>
	{
		unsafe { glfwSetGamma(self.0, gamma) };
		XErr::result(|| ())
	}

	/// Returns the current gamma ramp of this monitor.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized], [XErr::Platform],
	/// [XErr::FeatureUnavailable] (see remarks).
	///
	/// # Remarks
	/// **Wayland**: Gamma handling is a privileged protocol, this function will
	/// thus never be implemented and returns [XErr::FeatureUnavailable].
	///
	/// # Thread Safety
	/// This function must only be called from the main thread.
	pub fn gamma_ramp(&self) -> Result<GammaRamp, XErr>
	{
		let ramp = unsafe { glfwGetGammaRamp(self.0) };

		match unsafe { ramp.as_ref() }
		{
			| None => Err(XErr::get()),
			| Some(gr) => Ok(GammaRamp::from_glfw(gr)),
		}
	}

	/// Sets the current gamma ramp for this monitor. The original gamma ramp
	/// for that monitor is saved by XWin the first time this function is called
	/// and is restored by [XWin::drop](crate::core::XWin::drop).
	///
	/// The software controlled gamma ramp is applied in addition to the
	/// hardware gamma correction, which today is usually an approximation of
	/// sRGB gamma. This means that setting a perfectly linear ramp, or gamma
	/// `1.0`, will produce the default (usually sRGB-like) behavior.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized], [XErr::Platform], and
	/// [XErr::FeatureUnavailable] (see remarks).
	///
	/// # Remarks
	/// The size of the specified gamma ramp should match the size of the
	/// current ramp for that monitor.
	///
	/// **Windows**: The gamma ramp size must be 256.
	///
	/// **Wayland**: Gamma handling is a privileged protocol, this function will
	/// thus never be implemented and returns [XErr::FeatureUnavailable].
	///
	/// # Thread Safety
	/// This function must only be called from the main thread.
	pub fn set_gamma_ramp(&self, ramp: &mut GammaRamp) -> Result<(), XErr>
	{
		ramp.with_glfw(|ramp| unsafe { glfwSetGammaRamp(self.0, ramp) });
		XErr::result(|| ())
	}

	fn from_glfw(monitor: *mut GLFWmonitor) -> Self
	{
		Monitor(monitor)
	}
}
