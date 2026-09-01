use std::{
	ffi::{
		CStr,
		CString,
	},
	ptr::null_mut,
	sync::mpsc::Sender,
};

use crate::{
	bind::{
		GLFW_ARROW_CURSOR,
		GLFW_CROSSHAIR_CURSOR,
		GLFW_IBEAM_CURSOR,
		GLFW_NOT_ALLOWED_CURSOR,
		GLFW_POINTING_HAND_CURSOR,
		GLFW_RESIZE_ALL_CURSOR,
		GLFW_RESIZE_EW_CURSOR,
		GLFW_RESIZE_NESW_CURSOR,
		GLFW_RESIZE_NS_CURSOR,
		GLFW_RESIZE_NWSE_CURSOR,
		GLFW_TRUE,
		GLFWcursor,
		GLFWwindow,
		glfwCreateCursor,
		glfwCreateStandardCursor,
		glfwDestroyCursor,
		glfwGetClipboardString,
		glfwGetCursorPos,
		glfwGetInputMode,
		glfwGetKey,
		glfwGetKeyName,
		glfwGetMouseButton,
		glfwRawMouseMotionSupported,
		glfwSetClipboardString,
		glfwSetCursor,
		glfwSetCursorPos,
		glfwSetInputMode,
	},
	core::{
		ScreenCoordinates,
		exec::send_string,
	},
	error::XErr,
	input::{
		ButtonState,
		mouse::CursorShape,
	},
};

pub(super) fn create_cursor(shape: CursorShape, tx: Sender<Result<*mut GLFWcursor, XErr>>)
{
	let cursor = unsafe {
		match shape
		{
			| CursorShape::Arrow => glfwCreateStandardCursor(GLFW_ARROW_CURSOR as i32),
			| CursorShape::IBeam => glfwCreateStandardCursor(GLFW_IBEAM_CURSOR as i32),
			| CursorShape::Crosshair => glfwCreateStandardCursor(GLFW_CROSSHAIR_CURSOR as i32),
			| CursorShape::PointingHand =>
			{
				glfwCreateStandardCursor(GLFW_POINTING_HAND_CURSOR as i32)
			},
			| CursorShape::ResizeHorizontal =>
			{
				glfwCreateStandardCursor(GLFW_RESIZE_EW_CURSOR as i32)
			},
			| CursorShape::ResizeVertical => glfwCreateStandardCursor(GLFW_RESIZE_NS_CURSOR as i32),
			| CursorShape::ResizeTopLeft =>
			{
				glfwCreateStandardCursor(GLFW_RESIZE_NWSE_CURSOR as i32)
			},
			| CursorShape::ResizeTopRight =>
			{
				glfwCreateStandardCursor(GLFW_RESIZE_NESW_CURSOR as i32)
			},
			| CursorShape::ResizeAll => glfwCreateStandardCursor(GLFW_RESIZE_ALL_CURSOR as i32),
			| CursorShape::NotAllowed => glfwCreateStandardCursor(GLFW_NOT_ALLOWED_CURSOR as i32),
			| CursorShape::Custom(img, hotspot) =>
			{
				glfwCreateCursor(&img.as_glfw(), hotspot.x, hotspot.y)
			},
		}
	};

	let _ = tx.send(XErr::result(|| cursor));
}

pub(super) fn destroy_cursor(cursor: *mut GLFWcursor, tx: Sender<Result<(), XErr>>)
{
	unsafe { glfwDestroyCursor(cursor) };
	let _ = tx.send(XErr::result(|| ()));
}

pub(super) fn input_mode(window: *mut GLFWwindow, mode: i32, tx: Sender<Result<i32, XErr>>)
{
	let value = unsafe { glfwGetInputMode(window, mode) };
	let _ = tx.send(XErr::result(|| value));
}

pub(super) fn set_input_mode(
	window: *mut GLFWwindow,
	mode: i32,
	value: i32,
	tx: Sender<Result<(), XErr>>,
)
{
	unsafe { glfwSetInputMode(window, mode, value) };
	let _ = tx.send(XErr::result(|| ()));
}

pub(super) fn raw_mouse_supported(tx: Sender<Result<bool, XErr>>)
{
	let value = unsafe { glfwRawMouseMotionSupported() } as u32;
	let _ = tx.send(XErr::result(
		|| if value == GLFW_TRUE { true } else { false },
	));
}

pub(super) fn key_name(key: i32, scancode: i32, tx: Sender<Result<Option<String>, XErr>>)
{
	send_string(unsafe { glfwGetKeyName(key, scancode) }, tx);
}

pub(super) fn key(win: *mut GLFWwindow, key: i32, tx: Sender<Result<ButtonState, XErr>>)
{
	let value = unsafe { glfwGetKey(win, key) };
	let _ = tx.send(XErr::result(|| ButtonState::from_glfw(value as u32)));
}

pub(super) fn mouse_button(win: *mut GLFWwindow, button: i32, tx: Sender<Result<ButtonState, XErr>>)
{
	let value = unsafe { glfwGetMouseButton(win, button) };
	let _ = tx.send(XErr::result(|| ButtonState::from_glfw(value as u32)));
}

pub(super) fn cursor_pos(win: *mut GLFWwindow, tx: Sender<Result<ScreenCoordinates<f64>, XErr>>)
{
	let mut x = 0.0;
	let mut y = 0.0;
	unsafe { glfwGetCursorPos(win, &mut x, &mut y) };
	let _ = tx.send(XErr::result(|| ScreenCoordinates { x, y }));
}

pub(super) fn set_cursor_pos(win: *mut GLFWwindow, x: f64, y: f64, tx: Sender<Result<(), XErr>>)
{
	unsafe { glfwSetCursorPos(win, x, y) };
	let _ = tx.send(XErr::result(|| ()));
}

pub(super) fn set_cursor(
	win: *mut GLFWwindow,
	cursor: *mut GLFWcursor,
	tx: Sender<Result<(), XErr>>,
)
{
	unsafe { glfwSetCursor(win, cursor) };
	let _ = tx.send(XErr::result(|| ()));
}

pub(super) fn set_clipboard_string(str: String, tx: Sender<Result<(), XErr>>)
{
	if let Ok(cstr) = CString::new(str)
	{
		unsafe { glfwSetClipboardString(null_mut(), cstr.as_ptr()) };
		let _ = tx.send(XErr::result(|| ()));
	}
}

pub(super) fn clipboard_string(tx: Sender<Result<String, XErr>>)
{
	let value = unsafe { glfwGetClipboardString(null_mut()) };
	let _ = tx.send(XErr::result(|| {
		if value.is_null()
		{
			String::new()
		}
		else
		{
			unsafe { CStr::from_ptr(value).to_string_lossy().into_owned() }
		}
	}));
}
