use std::os::raw::c_int;

use crate::{
	bind::{
		GLFWwindow,
		glfwSetWindowPosCallback,
	},
	core::{
		ContentScale,
		ScreenCoordinates,
	},
	window::context::WindowContext,
};

/// Almost all positions and sizes in XWin are measured in
/// [ScreenCoordinates](ScreenCoordinates). However, framebuffer sizes
/// are measured in pixels.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct Pixels
{
	pub x: i32,
	pub y: i32,
}

/// Window events
pub enum WindowEvent
{
	/// Sent when the window is moved. Contains the position, in screen
	/// coordinates, of the upper-left corner of the content area of the window.
	///
	/// # Remarks
	/// - **Wayland.** This callback will never be called, as there is no way
	///   for an application to know its global position.
	Position(ScreenCoordinates),
	/// Sent when the window is resized. Contains the size, in screen
	/// coordinates, of the content area of the window.
	Size(ScreenCoordinates),
	/// Sent when the user attempts to close the window, for example by clicking
	/// the close widget in the title bar.
	///
	/// The close flag is set before this event is sent, but you can modify it
	/// at any time with
	/// [Window::set_should_close](crate::window::Window::set_should_close).
	///
	/// The close event is not sent by dropping a
	/// [Window](crate::window::Window).
	///
	/// # Remarks
	/// - **MacOS.** Selecting Quit from the application menu will trigger the
	///   close callback for all windows.
	Close,
	/// Sent when the content area of the window needs to be redrawn, for
	/// example if the window has been exposed after having been covered by
	/// another window.
	///
	/// On compositing window systems such as Aero, Compiz, Aqua or Wayland,
	/// where the window contents are saved off-screen, this event may be sent
	/// only very infrequently or never at all.
	Refresh,
	/// Sent when the window gains or loses input focus.
	///
	/// After the focus event is sent for a window that lost input focus,
	/// synthetic key and mouse button release events will be generated for all
	/// such that had been pressed.
	Focus(bool),
	/// Sent when the window is iconified or restored.
	Iconify(bool),
	/// Sent when the window is maximized or restored.
	Maximize(bool),
	/// Sent when the framebuffer of the window is resized.
	BufferSize(Pixels),
	/// Sent when the content scale of the window changes.
	ContentScale(ContentScale),
}

extern "C" fn window_pos_callback(win: *mut GLFWwindow, x: c_int, y: c_int)
{
	event(win, WindowEvent::Position(ScreenCoordinates { x, y }));
}

fn event(win: *mut GLFWwindow, ev: WindowEvent)
{
	if let Some(ctx) = WindowContext::get(&win)
	{
		ctx.post(ev);
	}
}

pub(crate) fn set_window_callbacks(win: *mut GLFWwindow)
{
	unsafe {
		glfwSetWindowPosCallback(win, Some(window_pos_callback));
	}
}
