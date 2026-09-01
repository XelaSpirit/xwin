use std::sync::mpsc::channel;

use crate::{
	bind::{
		GLFW_CURSOR_CAPTURED,
		GLFW_CURSOR_DISABLED,
		GLFW_CURSOR_HIDDEN,
		GLFW_CURSOR_NORMAL,
		GLFW_MOUSE_BUTTON_4,
		GLFW_MOUSE_BUTTON_5,
		GLFW_MOUSE_BUTTON_6,
		GLFW_MOUSE_BUTTON_7,
		GLFW_MOUSE_BUTTON_8,
		GLFW_MOUSE_BUTTON_LEFT,
		GLFW_MOUSE_BUTTON_MIDDLE,
		GLFW_MOUSE_BUTTON_RIGHT,
		GLFWcursor,
	},
	core::{
		Pixels,
		XWin,
		exec::XWinMessage,
		image::Image,
	},
	error::XErr,
	glfw_enum,
};

#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton
{
	#[default]
	Left   = GLFW_MOUSE_BUTTON_LEFT as u8,
	Right  = GLFW_MOUSE_BUTTON_RIGHT as u8,
	Middle = GLFW_MOUSE_BUTTON_MIDDLE as u8,
	Four   = GLFW_MOUSE_BUTTON_4 as u8,
	Five   = GLFW_MOUSE_BUTTON_5 as u8,
	Six    = GLFW_MOUSE_BUTTON_6 as u8,
	Seven  = GLFW_MOUSE_BUTTON_7 as u8,
	Eight  = GLFW_MOUSE_BUTTON_8 as u8,
}
glfw_enum!(MouseButton, u8);

pub enum CursorShape
{
	/// Standard arrow cursor
	Arrow,
	/// Text input I-beam cursor
	IBeam,
	/// Crosshair cursor
	Crosshair,
	/// Pointing hand cursor
	PointingHand,
	/// Horizontal resize/move arrow cursor
	ResizeHorizontal,
	/// Vertical resize/move arrow cursor
	ResizeVertical,
	/// Top-left to bottom-right resize/move arrow cursor
	ResizeTopLeft,
	/// Top-right to bottom-left resize/move arrow cursor
	ResizeTopRight,
	/// Omni-directional resize/move cursor
	ResizeAll,
	/// Operation-not-allowed cursor
	NotAllowed,
	/// Custom cursor image
	///
	/// The pixels are 32-bit, little-endian, non-premultiplied RGBA, i.e. eight
	/// bits per channel with the red channel first. They are arranged
	/// canonically as packed sequential rows, starting from the top-left
	/// corner.
	///
	/// The cursor hotspot is specified in [Pixels], relative to the upper-left
	/// corner of the cursor image. X-axis points right and Y-axis points down.
	Custom(Image, Pixels),
}

#[repr(u32)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CursorMode
{
	/// Default. Makes the cursor visible and behave normally.
	Normal   = GLFW_CURSOR_NORMAL,
	/// Makes the cursor invisible when it is over the content area of the
	/// window but does not restrict the cursor from leaving.
	Hidden   = GLFW_CURSOR_HIDDEN,
	/// Hides and grabs the cursor, providing virtual and unlimited cursor
	/// movement. This is useful for implementing, for example, 3D camera
	/// controls.
	Disabled = GLFW_CURSOR_DISABLED,
	/// Makes the cursor visible and confines it to the content area of the
	/// window.
	Captured = GLFW_CURSOR_CAPTURED,
}
glfw_enum!(CursorMode, u32, CursorMode::Normal);

pub struct Cursor(*mut GLFWcursor);

impl Cursor
{
	/// Creates a new cursor that can be set for a window with TODO().
	///
	/// See [CursorShape] for more details.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized], [XErr::InvalidValue],
	/// [XErr::CursorUnavailable], and [XErr::Platform]
	pub fn try_new(shape: CursorShape) -> Result<Self, XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?
			.read()
			.unwrap()
			.post_rcv(XWinMessage::CreateCursor(shape, tx), rx)?
			.map(|win| Self::from_glfw(win))
	}

	pub(crate) fn as_glfw(&self) -> *mut GLFWcursor
	{
		self.0
	}

	/// Construct a new [Cursor] from a `GLFWcursor`.
	fn from_glfw(win: *mut GLFWcursor) -> Self
	{
		Self(win)
	}
}

impl Drop for Cursor
{
	/// Destroys the cursor. If the cursor is current for any window, that
	/// window will be reverted to the default cursor. This does not affect the
	/// cursor mode.
	fn drop(&mut self)
	{
		let (tx, rx) = channel();
		if let Ok(xwin) = XWin::get()
		{
			let _ = xwin
				.read()
				.unwrap()
				.post_rcv(XWinMessage::DestroyCursor(self.0, tx), rx);
		}
	}
}

/// Returns whether raw mouse motion is supported on the current system.
/// This status does not change after XWin has been initialized so you only
/// need to check this once. If you attempt to enable raw motion on a system
/// that does not support it, [XErr::Platform] will be returned.
///
/// Raw mouse motion is closer to the actual motion of the mouse across a
/// surface. It is not affected by the scaling and acceleration applied to the
/// motion of the desktop cursor. That processing is suitable for a cursor while
/// raw motion is better for controlling for example a 3D camera. Because of
/// this, raw mouse motion is only provided when the cursor is disabled.
///
/// # Errors
/// Possible errors include [XErr::NotInitialized]
pub fn try_raw_mouse_supported() -> Result<bool, XErr>
{
	let (tx, rx) = channel();
	XWin::get()?
		.read()
		.unwrap()
		.post_rcv(XWinMessage::RawMouseSupported(tx), rx)?
}

/// See [try_raw_mouse_supported].
pub fn raw_mouse_supported() -> bool
{
	try_raw_mouse_supported().unwrap_or_default()
}
