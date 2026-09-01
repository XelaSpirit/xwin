//! Window related functions of XWin
//!
//! TODO documentation

mod builder;
pub mod config;
pub(crate) mod context;
pub mod control;
pub(crate) mod event;
pub mod input;

use std::sync::mpsc::channel;

pub use builder::*;

use crate::{
	bind::{
		GLFW_FALSE,
		GLFW_TRUE,
		GLFWwindow,
	},
	core::{
		XWin,
		exec::XWinMessage,
	},
	error::XErr,
	monitor::Monitor,
	window::context::WindowContext,
};

pub struct Window(*mut GLFWwindow);
unsafe impl Send for Window {}
unsafe impl Sync for Window {}

impl Window
{
	// =======================
	//       CONSTRUCTOR
	// =======================

	/// This function creates a window. Options controlling how the window and
	/// its context should be created are specified using other functions in
	/// [WindowBuilder].
	///
	/// The created window and framebuffer may differ from what you
	/// requested, as not all parameters and hints are hard constraints. This
	/// includes the size of the window, especially for full screen windows. To
	/// query the actual attributes of the created window, use the associated
	/// functions under [Window].
	///
	/// To create a full screen window, you need to specify the monitor the
	/// [Window] will cover. If no monitor is specified, the window will be
	/// windowed mode. Unless you have a way for the user to choose a specified
	/// monitor, it is recommended that you pick the [primary
	/// monitor](Monitor::try_primary). For more information on how to query
	/// connected monitors, see [retrieving
	/// monitors](crate::monitor#retrieving-monitors).
	///
	/// For full screen windows, the specified size becomes the resolution of
	/// the window's *desired video mode*. As long as a full screen window is
	/// not iconified, the supported video mode most closely matching the
	/// desired video mode is set for the specified monitor. For more
	/// information about full screen windows, including the creation of so
	/// called *windows full screen* or *borderless full screen* windows, see
	/// [Windowed Full Screen
	/// Windows](crate::window#windowed-full-screen-windows).
	///
	/// By default, newly created windows use the placement recommended by the
	/// window system. To create the window at a specific position, use the
	/// [WindowBuilder::position] function before creation.
	///
	/// As long as at least one full screen window is not iconified, the
	/// screensaver is prohibited from starting.
	///
	/// Window systems put limits on window sizes. Very large or very small
	/// window dimensions may be overridden by the window system on creation.
	/// Check the actual size after creation.
	///
	/// The swap interval is not set during window creation and the initial
	/// value may vary depending on driver settings and defaults.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized], [XErr::InvalidEnum],
	/// [XErr::InvalidValue], [XErr::ApiUnavailable],
	/// [XErr::VersionUnavailable], [XErr::FormatUnavailable],
	/// [XErr::Platform].
	///
	/// # Remarks
	/// **Windows**.
	/// - Window creation will fail if the Microsoft GDI software OpenGL
	///   implementation is the only one available.
	/// - If the executable has an icon resource named `GLFW_ICON`, it will be
	///   set as the initial icon for the window. If no such icon is present,
	///   the `IDI_APPLICATION` icon will be used instead. To set a different
	///   icon, see [Window::set_icon].
	///
	/// **MacOS**.
	/// - The GLFW window has no icon, as it is not a document window, but the
	///   dock icon will be the same as the application bundle's ion. For more
	///   information on bundles, see the **Bundle Programming Guide** in the
	///   Mac Developer Library.
	/// - On OS X 10.10 and later the window frame will not be rendered at full
	///   resolution on Retina displays unless the
	///   [WindowBuilder::scale_framebuffer] hint is `true` and the
	///   `NSHighResolutionCapable` key is enabled in the application bundle's
	///   `Info.plist`. For more information, see **High Resolution Guidelines
	///   for OS X** in the Mac Developer Library.
	/// - When activating frame autosaving with
	///   [WindowBuilder::cocoa_frame_name], the specified window size and
	///   position may be overridden by previously saved values.
	///
	/// **Wayland**.
	/// - XWin uses **libdecor** where available to create its window
	///   decorations. This in turn uses server-side XDG decorations where
	///   available and provides high quality client-side decorations on
	///   compositors like GNOME. If both XDG decorations and libdecor are
	///   unavailable, XWin falls back to a very simple set of window
	///   decorations that only support moving, resizing and the window
	///   manager's right-click menu.
	///
	/// **X11**.
	/// - Some window managers will not respect the placement of initially
	///   hidden windows.
	/// - Due to the asynchronous nature of X11, it may take a moment for a
	///   window to reach its requested state. This means you may not be able to
	///   query the final size, position or other attributes directly after
	///   window creation.
	/// - The class part of the `WM_CLASS` window property will by default be
	///   set to the window title passed to this function. The instance part
	///   will use the contents of the `RESOURCE_NAME` environment variable, if
	///   present and not empty, or fall back to the window title. Set the
	///   [WindowBuilder::x11_class_name] window hint to override this.
	pub fn try_new(
		width: i32,
		height: i32,
		title: &str,
		monitor: Option<Monitor>,
	) -> Result<Self, XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?
			.read()
			.unwrap()
			.post_rcv(
				XWinMessage::CreateWindow {
					width,
					height,
					title: String::from(title),
					monitor,
					builder: None,
					tx,
				},
				rx,
			)?
			.map(|win| Self::from_glfw(win))
	}

	// =======================
	//     CRATE FUNCTIONS
	// =======================

	/// Construct a new [Window] from a `GLFWwindow`.
	pub(crate) fn from_glfw(win: *mut GLFWwindow) -> Self
	{
		Window(win)
	}

	// =======================
	//    PRIVATE FUNCTIONS
	// =======================

	fn attr(&self, attr: u32) -> Result<bool, XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?
			.read()
			.unwrap()
			.post_rcv(XWinMessage::GetWindowAttribute(self.0, attr as i32, tx), rx)?
			.map(|v| v == GLFW_TRUE as i32)
	}

	fn set_attr(&mut self, attr: u32, value: bool) -> Result<(), XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?.read().unwrap().post_rcv(
			XWinMessage::SetWindowAttribute(
				self.0,
				attr as i32,
				if value
				{
					GLFW_TRUE as i32
				}
				else
				{
					GLFW_FALSE as i32
				},
				tx,
			),
			rx,
		)?
	}

	fn with_context<F>(&mut self, err: &str, func: F) -> Result<(), XErr>
	where
		F: FnOnce(&mut WindowContext),
	{
		WindowContext::with_context(&self.0, err, func)
	}
}

impl Drop for Window
{
	/// Destroys the window. Callbacks on the window may continue to be called
	/// until the window has been fully destroyed.
	///
	/// # Reentrancy
	/// This function must not be called from a callback.
	fn drop(&mut self)
	{
		let (tx, rx) = channel();
		if let Ok(xwin) = XWin::get()
		{
			let _ = xwin
				.read()
				.unwrap()
				.post_rcv(XWinMessage::DestroyWindow(self.0, tx), rx);
		}
	}
}
