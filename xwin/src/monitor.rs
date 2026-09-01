//! Monitor related functionality of XWin
//!
//! TODO documentation

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
	error::XErr,
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
	/// - [Monitor::try_primary]
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
	pub fn try_position(&self) -> Result<ScreenCoordinates<i32>, XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?
			.read()
			.unwrap()
			.post_rcv(XWinMessage::GetMonitorPos(self.0, tx), rx)?
	}

	/// See [Monitor::try_position].
	pub fn position(&self) -> ScreenCoordinates<i32>
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

	/// See [GammaRamp]
	pub fn gamma_ramp(&self) -> GammaRamp
	{
		self.try_gamma_ramp().unwrap_or_default()
	}

	/// Sets the current gamma ramp for the monitor. The original gamma ramp
	/// for that monitor is saved by XWin the first time this function is called
	/// and is restored when XWin is terminated.
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
