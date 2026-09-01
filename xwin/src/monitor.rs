//! Monitor related functionality of XWin
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
//! # let xwin = XWin::init(|| {
//! let primary = Monitor::primary();
//! # });
//! ```
//!
//! You can retrieve all currently connected monitors with [Monitor::all].
//!
//! ```
//! # use xwin::core::XWin;
//! # use xwin::monitor::Monitor;
//! # let xwin = XWin::init(|| {
//! let monitors = Monitor::all();
//! # });
//! ```
//!
//! The primary monitor is always the first monitor in the returned [Vec], but
//! other monitors may be moved to a different index when a monitor is connected
//! or disconnected.
//!
//! # Monitor Configuration Changes
//! If you wish to be notified when a monitor is connected or disconnected, use
//! the monitor configuration event channel.
//!
//! ```
//! # use xwin::core::XWin;
//! # use xwin::monitor::{monitor_event_rx, Monitor, MonitorEvent};
//! # let xwin = XWin::init(|| {
//! let rx = monitor_event_rx().unwrap();
//! # return;
//! while let Ok(ev) = rx.recv()
//! {
//! 	match ev
//! 	{
//! 		| MonitorEvent::Connected =>
//! 		{ /* ... */ },
//! 		| MonitorEvent::Disconnected =>
//! 		{ /* ... */ },
//! 	}
//! }
//! # });
//! ```
//!
//! If a monitor is disconnected, all windows that are full screen on it will be
//! switched to windowed mode before the event is sent.
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
//! # let xwin = XWin::init(|| {
//! let primary = Monitor::primary();
//! let video_modes = primary.unwrap().video_modes().unwrap();
//! # });
//! ```
//!
//! To get the current video mode of a monitor call [Monitor::video_mode].
//!
//! ```
//! # use xwin::core::XWin;
//! # use xwin::monitor::Monitor;
//! # let xwin = XWin::init(|| {
//! let primary = Monitor::primary();
//! let video_mode = primary.unwrap().video_mode().unwrap();
//! # });
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
//! # XWin::init(|| {
//! let primary = Monitor::primary();
//! let size_mm = primary.unwrap().physical_size().unwrap();
//! # });
//! ```
//!
//! While this can be used to calculate the raw DPI of a monitor, this is often
//! not useful. Instead, use the [monitor content scale](Monitor::content_scale)
//! and [window content scale](crate::window::Window::content_scale) to scale
//! your content.
//!
//! ## Content scale
//! The content scale for a monitor can be retrieved with
//! [Monitor::content_scale].
//!
//! ```
//! # use xwin::core::XWin;
//! # use xwin::monitor::Monitor;
//! # let xwin = XWin::init(|| {
//! let primary = Monitor::primary();
//! let content_scale = primary.unwrap().content_scale().unwrap();
//! # });
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
//! # let xwin = XWin::init(|| {
//! let primary = Monitor::primary();
//! let position = primary.unwrap().position().unwrap();
//! # });
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
//! # let xwin = XWin::init(|| {
//! let primary = Monitor::primary();
//! let work_area = primary.unwrap().work_area().unwrap();
//! # });
//! ```
//!
//! ## Human-Readable Name
//! The human-readable name of a monitor is returned by [Monitor::name].
//!
//! ```
//! # use xwin::core::XWin;
//! # use xwin::monitor::Monitor;
//! # let xwin = XWin::init(|| {
//! let primary = Monitor::primary();
//! let name = primary.unwrap().name().unwrap();
//! # });
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
//! # let xwin = XWin::init(|| {
//! let mut ramp = GammaRamp::new(256, 0);
//!
//! for idx in 0..256
//! {
//! 	// Fill out gamma ramp values as desired
//! }
//!
//! let primary = Monitor::primary();
//! let name = primary.unwrap().set_gamma_ramp(ramp);
//! # });
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
//! # let xwin = XWin::init(|| {
//! let primary = Monitor::primary().unwrap();
//! let gamma_ramp = primary.gamma_ramp().unwrap();
//! # });
//! ```
//!
//! If you wish to set a regular gamma ramp, you can have XWin calculate it for
//! you from the desired exponent with [Monitor::set_gamma], which in turn calls
//! [Monitor::set_gamma_ramp] with the resulting ramp.
//!
//! ```
//! # use xwin::core::XWin;
//! # use xwin::monitor::Monitor;
//! # let xwin = XWin::init(|| {
//! let primary = Monitor::primary();
//! let gamma_ramp = primary.unwrap().set_gamma(1.0);
//! # });
//! ```

mod events;
mod gamma_ramp;
mod video_mode;
mod work_area;

use std::sync::mpsc::channel;

pub use events::*;
pub use gamma_ramp::*;
pub use video_mode::*;
pub use work_area::*;

use crate::{
	bind::GLFWmonitor,
	core::{
		ContentScale,
		ScreenCoordinates,
		XWin,
		exec::XWinMessage,
	},
	err::XErr,
};

/// Almost all positions and sizes in XWin are measured in
/// [ScreenCoordinates]. However, a monitor's
/// physical size is measured in millimeters
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Millimeters
{
	pub x: i32,
	pub y: i32,
}

impl Default for Millimeters
{
	/// Construct a new [Millimeters] with `x` and `y` set to `0`.
	fn default() -> Millimeters
	{
		Millimeters { x: 0, y: 0 }
	}
}

#[derive(Debug)]
pub struct Monitor(*mut GLFWmonitor);
unsafe impl Send for Monitor {}
unsafe impl Sync for Monitor {}

impl PartialEq for Monitor
{
	fn eq(&self, other: &Self) -> bool
	{
		self.0 == other.0
	}
}

impl Monitor
{
	/// Returns a [Vec] containing all currently connected monitors. The primary
	/// monitor is always first in the returned list.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized].
	///
	/// # See Also
	/// - [Monitor::primary]
	pub fn try_all() -> Result<Vec<Monitor>, XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?
			.read()
			.unwrap()
			.post_rcv(XWinMessage::GetMonitors(tx), rx)?
			.map(|vec| {
				vec.iter()
					.map(|monitor| Self::from_glfw(*monitor))
					.collect()
			})
	}

	/// See [Monitor::try_all].
	pub fn all() -> Vec<Monitor>
	{
		Self::try_all().unwrap_or_default()
	}

	/// Returns the primary monitor. This is usually the monitor where elements
	/// like the task bar or global menu bar are located.
	///
	/// # Errors
	/// Returns [XErr::None] if no monitors were found. Other possible errors
	/// include [XErr::NotInitialized].
	///
	/// # Remarks
	/// The primary monitor is always first in the [Vec] returned by
	/// [Monitor::all]
	pub fn try_primary() -> Result<Monitor, XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?
			.read()
			.unwrap()
			.post_rcv(XWinMessage::GetPrimaryMonitor(tx), rx)?
			.map(|monitor| Self::from_glfw(monitor))
	}

	/// Returns the position `(x, y)`, in **screen coordinates**, of the
	/// upper-left corner of the monitor.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized] and [XErr::Platform].
	pub fn try_position(&self) -> Result<ScreenCoordinates, XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?
			.read()
			.unwrap()
			.post_rcv(XWinMessage::GetMonitorPos(self.0, tx), rx)?
	}

	/// See [Monitor::try_position].
	pub fn position(&self) -> ScreenCoordinates
	{
		self.try_position().unwrap_or_default()
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
	/// # See Also
	/// - [WorkArea]
	pub fn try_work_area(&self) -> Result<WorkArea, XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?
			.read()
			.unwrap()
			.post_rcv(XWinMessage::GetMonitorWorkArea(self.0, tx), rx)?
	}

	/// See [Monitor::try_work_area].
	pub fn work_area(&self) -> WorkArea
	{
		self.try_work_area().unwrap_or_default()
	}

	/// Returns the size, in millimetres, of the display area
	/// of the monitor.
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
	pub fn try_physical_size(&self) -> Result<Millimeters, XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?
			.read()
			.unwrap()
			.post_rcv(XWinMessage::GetMonitorPhysicalSize(self.0, tx), rx)?
	}

	/// See [Monitor::physical_size].
	pub fn physical_size(&self) -> Millimeters
	{
		self.try_physical_size().unwrap_or_default()
	}

	/// Returns the content scale for the specified monitor.
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
	pub fn try_content_scale(&self) -> Result<ContentScale, XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?
			.read()
			.unwrap()
			.post_rcv(XWinMessage::GetMonitorContentScale(self.0, tx), rx)?
	}

	/// See [Monitor::try_content_scale].
	pub fn content_scale(&self) -> ContentScale
	{
		self.try_content_scale().unwrap_or_default()
	}

	/// Returns a human-readable name, encoded as UTF-8, of the monitor. The
	/// name typically reflects the make and model of the monitor and is not
	/// guaranteed to be unique among the connected monitors.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized].
	pub fn try_name(&self) -> Result<String, XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?
			.read()
			.unwrap()
			.post_rcv(XWinMessage::GetMonitorName(self.0, tx), rx)?
	}

	/// See [Monitor::try_name].
	pub fn name(&self) -> String
	{
		self.try_name().unwrap_or_default()
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
	/// # See Also
	/// - [Monitor::video_mode]
	pub fn try_video_modes(&self) -> Result<Vec<VideoMode>, XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?
			.read()
			.unwrap()
			.post_rcv(XWinMessage::GetMonitorVideoModes(self.0, tx), rx)?
	}

	/// See [Monitor::try_video_modes].
	pub fn video_modes(&self) -> Vec<VideoMode>
	{
		self.try_video_modes().unwrap_or_default()
	}

	/// This function returns the current video mode of the monitor. If you
	/// have created a full screen window for that monitor, the return value
	/// will depend on whether that window is iconified.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized] and [XErr::Platform].
	///
	/// # See Also
	/// - [Monitor::video_modes]
	pub fn try_video_mode(&self) -> Result<VideoMode, XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?
			.read()
			.unwrap()
			.post_rcv(XWinMessage::GetMonitorVideoMode(self.0, tx), rx)?
	}

	/// See [Monitor::try_video_mode].
	pub fn video_mode(&self) -> VideoMode
	{
		self.try_video_mode().unwrap_or_default()
	}

	/// This function generates an appropriately sized gamma ramp from the
	/// specified exponent and then sets the gamma ramp of the monitor to it.
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
	pub fn try_set_gamma(&mut self, gamma: f32) -> Result<(), XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?
			.read()
			.unwrap()
			.post_rcv(XWinMessage::SetGamma(self.0, gamma, tx), rx)?
	}

	/// See [Monitor::try_set_gamma].
	pub fn set_gamma(&mut self, gamma: f32)
	{
		self.try_set_gamma(gamma).unwrap_or_default()
	}

	/// Returns the current gamma ramp of the monitor.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized], [XErr::Platform],
	/// [XErr::FeatureUnavailable] (see remarks).
	///
	/// # Remarks
	/// **Wayland**: Gamma handling is a privileged protocol, this function will
	/// thus never be implemented and returns [XErr::FeatureUnavailable].
	pub fn try_gamma_ramp(&self) -> Result<GammaRamp, XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?
			.read()
			.unwrap()
			.post_rcv(XWinMessage::GammaRamp(self.0, tx), rx)?
	}

	/// See [Monitor::GammaRamp]
	pub fn gamma_ramp(&self) -> GammaRamp
	{
		self.try_gamma_ramp().unwrap_or_default()
	}

	/// Sets the current gamma ramp for the monitor. The original gamma ramp
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
	pub fn try_set_gamma_ramp(&mut self, ramp: GammaRamp) -> Result<(), XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?
			.read()
			.unwrap()
			.post_rcv(XWinMessage::SetGammaRamp(self.0, ramp, tx), rx)?
	}

	/// See [Monitor::try_set_gamma_ramp].
	pub fn set_gamma_ramp(&mut self, ramp: GammaRamp)
	{
		self.try_set_gamma_ramp(ramp).unwrap_or_default()
	}

	/// Construct a new [Monitor] from a `GLFWmonitor`.
	pub(crate) fn from_glfw(monitor: *mut GLFWmonitor) -> Self
	{
		Monitor(monitor)
	}

	/// Return the `GLFWmonitor` held by this [Monitor].
	pub(crate) fn as_glfw(&self) -> *mut GLFWmonitor
	{
		self.0
	}
}
