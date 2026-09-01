use std::{
	ffi::{
		CStr,
		c_char,
		c_double,
		c_uint,
	},
	os::raw::{
		c_float,
		c_int,
	},
	slice,
};

use crate::{
	bind::{
		GLFW_TRUE,
		GLFWwindow,
		glfwSetCharCallback,
		glfwSetCursorEnterCallback,
		glfwSetCursorPosCallback,
		glfwSetDropCallback,
		glfwSetFramebufferSizeCallback,
		glfwSetKeyCallback,
		glfwSetMouseButtonCallback,
		glfwSetScrollCallback,
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
		ButtonEvent,
		ButtonState,
		keyboard::{
			Key,
			Modifiers,
		},
		mouse::MouseButton,
	},
	window::context::WindowContext,
};

/// Struct type for window events
#[derive(Debug, Clone, Copy)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyEvent
{
	key:      Key,
	scancode: i32,
	action:   ButtonEvent,
	mods:     Modifiers,
}

/// Struct type for mouse button events
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MouseButtonEvent
{
	button: MouseButton,
	action: ButtonState,
	mods:   Modifiers,
}

/// Enum type for mouse events
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MouseEvent
{
	/// Sent when a mouse button is pressed or released.
	///
	/// When a window loses input focus, it will generate synthetic mouse button
	/// release events fopr all pressed mouse buttons. These synthetic mouse
	/// button events will be sent after the focus loss event has been sent.
	Button(MouseButtonEvent),
	/// Sent when the cursor is moved. The cursor position is given in screen
	/// coordinates relative to the upper-left corner of the content are of the
	/// window.
	Position(ScreenCoordinates<f64>),
	/// Sent when the cursor enter or leaves the content area of the window. The
	/// contained boolean is `true` when the cursor has entered, `false` when it
	/// has left.
	Enter(bool),
	/// Sent when a scrolling device is used, such as a mouse wheel or scrolling
	/// area of a touchpad.
	///
	/// This event contains both the `x_offset` and `y_offset` of the scroll, in
	/// that order.
	Scroll(f64, f64),
}

extern "C" fn char_cb(win: *mut GLFWwindow, codepoint: c_uint)
{
	let _ = WindowContext::with_context(&win, "", |ctx| ctx.post_char(codepoint));
}

extern "C" fn close_cb(win: *mut GLFWwindow)
{
	config_event(win, WindowEvent::Close);
}

extern "C" fn content_scale_cb(win: *mut GLFWwindow, x: c_float, y: c_float)
{
	config_event(win, WindowEvent::ContentScale(ContentScale { x, y }));
}

extern "C" fn drop_cb(win: *mut GLFWwindow, count: c_int, paths: *mut *const c_char)
{
	if paths.is_null() || count <= 0
	{
		return;
	}

	let slice = unsafe { slice::from_raw_parts(paths, count as usize) };

	let vec: Vec<String> = slice
		.iter()
		.filter(|&&ptr| !ptr.is_null())
		.map(|&ptr| {
			unsafe { CStr::from_ptr(ptr) }
				.to_string_lossy()
				.into_owned()
		})
		.collect();

	let _ = WindowContext::with_context(&win, "", |ctx| ctx.post_drop(vec));
}

extern "C" fn focus_cb(win: *mut GLFWwindow, focused: c_int)
{
	config_event(win, WindowEvent::Focus(focused == GLFW_TRUE as i32));
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

extern "C" fn iconify_cb(win: *mut GLFWwindow, iconified: c_int)
{
	config_event(win, WindowEvent::Iconify(iconified == GLFW_TRUE as i32));
}

extern "C" fn key_cb(win: *mut GLFWwindow, key: c_int, scancode: c_int, action: c_int, mods: c_int)
{
	let _ = WindowContext::with_context(&win, "", |ctx| {
		ctx.post_key(KeyEvent {
			key: unsafe { Key::from_glfw(key as u32) },
			scancode,
			action: unsafe { ButtonEvent::from_glfw(action as u32) },
			mods: Modifiers::from_glfw_crate(mods),
		})
	});
}

extern "C" fn maximize_cb(win: *mut GLFWwindow, maximized: c_int)
{
	config_event(win, WindowEvent::Maximize(maximized == GLFW_TRUE as i32));
}

extern "C" fn mouse_button_cb(win: *mut GLFWwindow, button: c_int, action: c_int, mods: c_int)
{
	let _ = WindowContext::with_context(&win, "", |ctx| {
		ctx.post_mouse(MouseEvent::Button(MouseButtonEvent {
			button: unsafe { MouseButton::from_glfw(button as u32) },
			action: unsafe { ButtonState::from_glfw(action as u32) },
			mods:   Modifiers::from_glfw_crate(mods),
		}))
	});
}

extern "C" fn mouse_enter_cb(win: *mut GLFWwindow, entered: c_int)
{
	let _ = WindowContext::with_context(&win, "", |ctx| {
		ctx.post_mouse(MouseEvent::Enter(
			if entered == GLFW_TRUE as i32
			{
				true
			}
			else
			{
				false
			},
		))
	});
}

extern "C" fn mouse_pos_cb(win: *mut GLFWwindow, xpos: c_double, ypos: c_double)
{
	let _ = WindowContext::with_context(&win, "", |ctx| {
		ctx.post_mouse(MouseEvent::Position(ScreenCoordinates {
			x: xpos as f64,
			y: ypos as f64,
		}))
	});
}

extern "C" fn mouse_scroll_cb(win: *mut GLFWwindow, xoff: c_double, yoff: c_double)
{
	let _ = WindowContext::with_context(&win, "", |ctx| {
		ctx.post_mouse(MouseEvent::Scroll(xoff as f64, yoff as f64))
	});
}

extern "C" fn pos_cb(win: *mut GLFWwindow, x: c_int, y: c_int)
{
	config_event(win, WindowEvent::Position(ScreenCoordinates { x, y }));
}

extern "C" fn refresh_cb(win: *mut GLFWwindow)
{
	config_event(win, WindowEvent::Refresh);
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

fn config_event(win: *mut GLFWwindow, evt: WindowEvent)
{
	let _ = WindowContext::with_context(&win, "", |ctx| ctx.post_config(evt));
}

pub(crate) fn set_window_callbacks(win: *mut GLFWwindow)
{
	unsafe {
		glfwSetCharCallback(win, Some(char_cb));
		glfwSetCursorEnterCallback(win, Some(mouse_enter_cb));
		glfwSetCursorPosCallback(win, Some(mouse_pos_cb));
		glfwSetDropCallback(win, Some(drop_cb));
		glfwSetFramebufferSizeCallback(win, Some(framebuffer_size_cb));
		glfwSetKeyCallback(win, Some(key_cb));
		glfwSetMouseButtonCallback(win, Some(mouse_button_cb));
		glfwSetScrollCallback(win, Some(mouse_scroll_cb));
		glfwSetWindowCloseCallback(win, Some(close_cb));
		glfwSetWindowContentScaleCallback(win, Some(content_scale_cb));
		glfwSetWindowFocusCallback(win, Some(focus_cb));
		glfwSetWindowIconifyCallback(win, Some(iconify_cb));
		glfwSetWindowMaximizeCallback(win, Some(maximize_cb));
		glfwSetWindowPosCallback(win, Some(pos_cb));
		glfwSetWindowRefreshCallback(win, Some(refresh_cb));
		glfwSetWindowSizeCallback(win, Some(size_cb));
	}
}
