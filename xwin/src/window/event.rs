use std::os::raw::{
	c_float,
	c_int,
};

use crate::{
	bind::{
		GLFW_TRUE,
		GLFWwindow,
		glfwSetFramebufferSizeCallback,
		glfwSetKeyCallback,
		glfwSetWindowCloseCallback,
		glfwSetWindowContentScaleCallback,
		glfwSetWindowFocusCallback,
		glfwSetWindowIconifyCallback,
		glfwSetWindowMaximizeCallback,
		glfwSetWindowPosCallback,
		glfwSetWindowRefreshCallback,
		glfwSetWindowSizeCallback,
	},
	core::{
		ContentScale,
		Pixels,
		ScreenCoordinates,
	},
	input::{
		ButtonState,
		keyboard::{
			Key,
			Modifiers,
		},
	},
	window::context::WindowContext,
};

/// Struct type for window events
pub enum WindowEvent
{
	/// Sent when the window is moved. Contains the position, in screen
	/// coordinates, of the upper-left corner of the content area of the window.
	///
	/// # Remarks
	/// - **Wayland.** This callback will never be called, as there is no way
	///   for an application to know its global position.
	Position(ScreenCoordinates<i32>),
	/// Sent when the window is resized. Contains the size, in screen
	/// coordinates, of the content area of the window.
	Size(ScreenCoordinates<i32>),
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
	FramebufferSize(Pixels),
	/// Sent when the content scale of the window changes.
	ContentScale(ContentScale),
}

/// Struct type for key events.
pub struct KeyEvent
{
	key:      Key,
	scancode: i32,
	action:   ButtonState,
	mods:     Modifiers,
}

extern "C" fn pos_cb(win: *mut GLFWwindow, x: c_int, y: c_int)
{
	config_event(win, WindowEvent::Position(ScreenCoordinates { x, y }));
}

extern "C" fn size_cb(win: *mut GLFWwindow, width: c_int, height: c_int)
{
	config_event(
		win,
		WindowEvent::Size(ScreenCoordinates {
			x: width,
			y: height,
		}),
	);
}

extern "C" fn close_cb(win: *mut GLFWwindow)
{
	config_event(win, WindowEvent::Close);
}

extern "C" fn refresh_cb(win: *mut GLFWwindow)
{
	config_event(win, WindowEvent::Refresh);
}

extern "C" fn focus_cb(win: *mut GLFWwindow, focused: c_int)
{
	config_event(win, WindowEvent::Focus(focused == GLFW_TRUE as i32));
}

extern "C" fn iconify_cb(win: *mut GLFWwindow, iconified: c_int)
{
	config_event(win, WindowEvent::Iconify(iconified == GLFW_TRUE as i32));
}

extern "C" fn maximize_cb(win: *mut GLFWwindow, maximized: c_int)
{
	config_event(win, WindowEvent::Maximize(maximized == GLFW_TRUE as i32));
}

extern "C" fn framebuffer_size_cb(win: *mut GLFWwindow, width: c_int, height: c_int)
{
	config_event(
		win,
		WindowEvent::FramebufferSize(Pixels {
			x: width,
			y: height,
		}),
	);
}

extern "C" fn key_cb(win: *mut GLFWwindow, key: c_int, scancode: c_int, action: c_int, mods: c_int)
{
	key_event(
		win,
		Key::from_glfw(key as u32),
		scancode,
		ButtonState::from_glfw(action as u32),
		Modifiers::from_glfw(mods),
	);
}

extern "C" fn content_scale_cb(win: *mut GLFWwindow, x: c_float, y: c_float)
{
	config_event(win, WindowEvent::ContentScale(ContentScale { x, y }));
}

fn config_event(win: *mut GLFWwindow, ev: WindowEvent)
{
	if let Some(ctx) = WindowContext::get(&win)
	{
		ctx.post_config(ev);
	}
}

fn key_event(win: *mut GLFWwindow, key: Key, scancode: i32, action: ButtonState, mods: Modifiers)
{
	if let Some(ctx) = WindowContext::get(&win)
	{
		ctx.post_key(KeyEvent {
			key,
			scancode,
			action,
			mods,
		});
	}
}

pub(crate) fn set_window_callbacks(win: *mut GLFWwindow)
{
	unsafe {
		glfwSetWindowPosCallback(win, Some(pos_cb));
		glfwSetWindowSizeCallback(win, Some(size_cb));
		glfwSetWindowCloseCallback(win, Some(close_cb));
		glfwSetWindowRefreshCallback(win, Some(refresh_cb));
		glfwSetWindowFocusCallback(win, Some(focus_cb));
		glfwSetWindowIconifyCallback(win, Some(iconify_cb));
		glfwSetWindowMaximizeCallback(win, Some(maximize_cb));
		glfwSetFramebufferSizeCallback(win, Some(framebuffer_size_cb));
		glfwSetWindowContentScaleCallback(win, Some(content_scale_cb));
		glfwSetKeyCallback(win, Some(key_cb));
	}
}
