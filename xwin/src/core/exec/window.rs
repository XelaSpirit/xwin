use std::{
	ffi::{
		CStr,
		CString,
	},
	os::raw::c_void,
	ptr::null_mut,
	sync::mpsc::Sender,
};

use crate::{
	bind::{
		GLFW_CLIENT_API,
		GLFW_NO_API,
		GLFWimage,
		GLFWwindow,
		glfwCreateWindow,
		glfwDefaultWindowHints,
		glfwDestroyWindow,
		glfwFocusWindow,
		glfwGetFramebufferSize,
		glfwGetWindowAttrib,
		glfwGetWindowContentScale,
		glfwGetWindowFrameSize,
		glfwGetWindowMonitor,
		glfwGetWindowOpacity,
		glfwGetWindowPos,
		glfwGetWindowSize,
		glfwGetWindowTitle,
		glfwGetWindowUserPointer,
		glfwHideWindow,
		glfwIconifyWindow,
		glfwMaximizeWindow,
		glfwRequestWindowAttention,
		glfwRestoreWindow,
		glfwSetWindowAspectRatio,
		glfwSetWindowAttrib,
		glfwSetWindowIcon,
		glfwSetWindowMonitor,
		glfwSetWindowOpacity,
		glfwSetWindowPos,
		glfwSetWindowSize,
		glfwSetWindowSizeLimits,
		glfwSetWindowTitle,
		glfwSetWindowUserPointer,
		glfwShowWindow,
		glfwWindowHint,
	},
	core::{
		ContentScale,
		Pixels,
		ScreenCoordinates,
		image::Image,
	},
	error::XErr,
	event::set_window_callbacks,
	monitor::Monitor,
	window::{
		WindowBuilder,
		context::WindowContext,
	},
};

pub(super) fn set_window_attribute(
	win: *mut GLFWwindow,
	attr: i32,
	value: i32,
	tx: Sender<Result<(), XErr>>,
)
{
	unsafe { glfwSetWindowAttrib(win, attr, value) };
	let _ = tx.send(XErr::result(|| ()));
}

pub(super) fn window_attribute(win: *mut GLFWwindow, attr: i32, tx: Sender<Result<i32, XErr>>)
{
	let value = unsafe { glfwGetWindowAttrib(win, attr) };
	let _ = tx.send(XErr::result(|| value));
}

pub(super) fn set_window_monitor(
	win: *mut GLFWwindow,
	mon: Option<Monitor>,
	pos: ScreenCoordinates<i32>,
	size: ScreenCoordinates<i32>,
	refresh_rate: i32,
	tx: Sender<Result<(), XErr>>,
)
{
	unsafe {
		glfwSetWindowMonitor(
			win,
			match mon
			{
				| Some(m) => m.as_glfw(),
				| None => null_mut(),
			},
			pos.x,
			pos.y,
			size.x,
			size.y,
			refresh_rate,
		)
	}
	let _ = tx.send(XErr::result(|| ()));
}

pub(super) fn window_monitor(win: *mut GLFWwindow, tx: Sender<Result<Option<Monitor>, XErr>>)
{
	let monitor = unsafe { glfwGetWindowMonitor(win) };
	let _ = tx.send(XErr::result(|| {
		if monitor.is_null()
		{
			None
		}
		else
		{
			Some(unsafe { Monitor::from_glfw(monitor) })
		}
	}));
}

pub(super) fn request_window_attention(win: *mut GLFWwindow, tx: Sender<Result<(), XErr>>)
{
	unsafe { glfwRequestWindowAttention(win) };
	let _ = tx.send(XErr::result(|| ()));
}

pub(super) fn focus_window(win: *mut GLFWwindow, tx: Sender<Result<(), XErr>>)
{
	unsafe { glfwFocusWindow(win) };
	let _ = tx.send(XErr::result(|| ()));
}

pub(super) fn hide_window(win: *mut GLFWwindow, tx: Sender<Result<(), XErr>>)
{
	unsafe { glfwHideWindow(win) };
	let _ = tx.send(XErr::result(|| ()));
}

pub(super) fn show_window(win: *mut GLFWwindow, tx: Sender<Result<(), XErr>>)
{
	unsafe { glfwShowWindow(win) };
	let _ = tx.send(XErr::result(|| ()));
}

pub(super) fn maximize_window(win: *mut GLFWwindow, tx: Sender<Result<(), XErr>>)
{
	unsafe { glfwMaximizeWindow(win) };
	let _ = tx.send(XErr::result(|| ()));
}

pub(super) fn restore_window(win: *mut GLFWwindow, tx: Sender<Result<(), XErr>>)
{
	unsafe { glfwRestoreWindow(win) };
	let _ = tx.send(XErr::result(|| ()));
}

pub(super) fn iconify_window(win: *mut GLFWwindow, tx: Sender<Result<(), XErr>>)
{
	unsafe { glfwIconifyWindow(win) };
	let _ = tx.send(XErr::result(|| ()));
}

pub(super) fn set_window_opacity(win: *mut GLFWwindow, opacity: f32, tx: Sender<Result<(), XErr>>)
{
	unsafe { glfwSetWindowOpacity(win, opacity) };
	let _ = tx.send(XErr::result(|| ()));
}

pub(super) fn window_opacity(win: *mut GLFWwindow, tx: Sender<Result<f32, XErr>>)
{
	let opacity = unsafe { glfwGetWindowOpacity(win) };
	let _ = tx.send(XErr::result(|| opacity));
}

pub(super) fn window_content_scale(win: *mut GLFWwindow, tx: Sender<Result<ContentScale, XErr>>)
{
	let mut xscale = 0.0f32;
	let mut yscale = 0.0f32;
	unsafe { glfwGetWindowContentScale(win, &mut xscale, &mut yscale) };
	let _ = tx.send(XErr::result(|| {
		ContentScale {
			x: xscale,
			y: yscale,
		}
	}));
}

pub(super) fn window_frame_size(
	win: *mut GLFWwindow,
	tx: Sender<Result<(u32, u32, u32, u32), XErr>>,
)
{
	let mut left = 0i32;
	let mut top = 0i32;
	let mut right = 0i32;
	let mut bottom = 0i32;
	unsafe { glfwGetWindowFrameSize(win, &mut left, &mut top, &mut right, &mut bottom) };
	let _ = tx.send(XErr::result(|| {
		(left as u32, top as u32, right as u32, bottom as u32)
	}));
}

pub(super) fn framebuffer_size(win: *mut GLFWwindow, tx: Sender<Result<Pixels, XErr>>)
{
	let mut width = 0i32;
	let mut height = 0i32;
	unsafe { glfwGetFramebufferSize(win, &mut width, &mut height) };
	let _ = tx.send(XErr::result(|| {
		Pixels {
			x: width,
			y: height,
		}
	}));
}

pub(super) fn set_window_size(
	win: *mut GLFWwindow,
	size: ScreenCoordinates<i32>,
	tx: Sender<Result<(), XErr>>,
)
{
	unsafe { glfwSetWindowSize(win, size.x, size.y) };
	let _ = tx.send(XErr::result(|| ()));
}

pub(super) fn set_window_aspect_ratio(
	win: *mut GLFWwindow,
	numer: i32,
	denom: i32,
	tx: Sender<Result<(), XErr>>,
)
{
	unsafe { glfwSetWindowAspectRatio(win, numer, denom) };
	let _ = tx.send(XErr::result(|| ()));
}

pub(super) fn set_window_size_limits(
	win: *mut GLFWwindow,
	min: ScreenCoordinates<i32>,
	max: ScreenCoordinates<i32>,
	tx: Sender<Result<(), XErr>>,
)
{
	unsafe { glfwSetWindowSizeLimits(win, min.x, min.y, max.x, max.y) };
	let _ = tx.send(XErr::result(|| ()));
}

pub(super) fn window_size(win: *mut GLFWwindow, tx: Sender<Result<ScreenCoordinates<i32>, XErr>>)
{
	let mut size = ScreenCoordinates::default();
	unsafe { glfwGetWindowSize(win, &mut size.x, &mut size.y) };
	let _ = tx.send(XErr::result(|| size));
}

pub(super) fn set_window_pos(
	win: *mut GLFWwindow,
	pos: ScreenCoordinates<i32>,
	tx: Sender<Result<(), XErr>>,
)
{
	unsafe { glfwSetWindowPos(win, pos.x, pos.y) };
	let _ = tx.send(XErr::result(|| ()));
}

pub(super) fn window_pos(win: *mut GLFWwindow, tx: Sender<Result<ScreenCoordinates<i32>, XErr>>)
{
	let mut pos = ScreenCoordinates::default();
	unsafe { glfwGetWindowPos(win, &mut pos.x, &mut pos.y) };
	let _ = tx.send(XErr::result(|| pos));
}

pub(super) fn set_window_icon(win: *mut GLFWwindow, icons: Vec<Image>, tx: Sender<Result<(), XErr>>)
{
	let glfw_icons: Vec<GLFWimage> = icons.iter().map(Image::as_glfw).collect();

	unsafe {
		glfwSetWindowIcon(
			win,
			glfw_icons.len() as i32,
			if glfw_icons.is_empty()
			{
				null_mut()
			}
			else
			{
				glfw_icons.as_ptr()
			},
		)
	};
	let _ = tx.send(XErr::result(|| ()));
}

pub(super) fn set_window_title(win: *mut GLFWwindow, title: String, tx: Sender<Result<(), XErr>>)
{
	let str = CString::new(title)
		.map_err(|_| XErr::InvalidValue(String::from("Window title may not contain null bytes")));
	if let Err(err) = str
	{
		let _ = tx.send(Err(err));
		return;
	}
	let str = str.unwrap();

	unsafe { glfwSetWindowTitle(win, str.as_ptr()) };
	let _ = tx.send(XErr::result(|| ()));
}

pub(super) fn window_title(win: *mut GLFWwindow, tx: Sender<Result<String, XErr>>)
{
	let title = unsafe { glfwGetWindowTitle(win) };
	let _ = tx.send(XErr::result(|| {
		unsafe { CStr::from_ptr(title) }
			.to_str()
			.unwrap_or_else(|_| "")
			.to_owned()
	}));
}

pub(super) fn destroy_window(win: *mut GLFWwindow, tx: Sender<Result<(), XErr>>)
{
	let ptr = unsafe { glfwGetWindowUserPointer(win) };
	if check_err(&tx)
	{
		return;
	}

	unsafe { glfwDestroyWindow(win) };
	drop(unsafe { Box::from_raw(ptr) });

	let _ = tx.send(XErr::result(|| ()));
}

pub(super) fn create_window(
	width: i32,
	height: i32,
	title: String,
	monitor: Option<Monitor>,
	builder: Option<WindowBuilder>,
	tx: Sender<Result<*mut GLFWwindow, XErr>>,
)
{
	// Reset window hints
	unsafe { glfwDefaultWindowHints() };
	if check_err(&tx)
	{
		return;
	}

	// Handle user-defined window hints
	if let Some(bld) = builder
	{
		if let Err(err) = bld.apply()
		{
			let _ = tx.send(Err(err));
			return;
		}
	}

	// XWin does not use GLFW, so disable that context
	unsafe { glfwWindowHint(GLFW_CLIENT_API as i32, GLFW_NO_API as i32) };
	if check_err(&tx)
	{
		return;
	}

	// Get title as CString (with null byte check)
	let str = CString::new(title)
		.map_err(|_| XErr::InvalidValue(String::from("Title contains a null byte")));
	if let Err(err) = str
	{
		let _ = tx.send(Err(err));
		return;
	}

	let str = str.unwrap();

	// Create window
	let win = unsafe {
		glfwCreateWindow(
			width,
			height,
			str.as_ptr(),
			match monitor
			{
				| Some(mon) => mon.as_glfw(),
				| None => null_mut(),
			},
			null_mut(),
		)
	};

	// Create window context and have glfw hold it as user data
	// TODO - Currently, this memory will be freed when a window is destroyed by
	// 		- code. However, this memory **will not** be freed if GLFW destroys the
	// 		- window for us (either through glfwTerminate() or by the user closing the
	// 		- window). This may lead to a memory leak.
	// 		-
	// 		- That said, this is likely to only happen when the program is ending,
	//      - anyway. This problem should be fixed at some point, but is not a
	//      - priority.
	let ctx = Box::into_raw(Box::new(WindowContext::new()));
	unsafe { glfwSetWindowUserPointer(win, ctx as *mut c_void) };
	if check_err(&tx)
	{
		drop(unsafe { Box::from_raw(ctx) });
		return;
	}

	// Send result
	let _ = if win.is_null()
	{
		let _ = tx.send(Err(XErr::get()));
		return;
	}
	else
	{
		let _ = tx.send(Ok(win));
	};

	// Set callbacks
	set_window_callbacks(win);
}

fn check_err<T>(tx: &Sender<Result<T, XErr>>) -> bool
{
	if let Err(err) = XErr::result(|| ())
	{
		let _ = tx.send(Err(err));
		true
	}
	else
	{
		false
	}
}
