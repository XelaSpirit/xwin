use std::sync::mpsc::channel;

use crate::{
	bind::{
		GLFWcursor,
		GLFW_MOUSE_BUTTON_4,
		GLFW_MOUSE_BUTTON_5,
		GLFW_MOUSE_BUTTON_6,
		GLFW_MOUSE_BUTTON_7,
		GLFW_MOUSE_BUTTON_8,
		GLFW_MOUSE_BUTTON_LEFT,
		GLFW_MOUSE_BUTTON_MIDDLE,
		GLFW_MOUSE_BUTTON_RIGHT,
	},
	core::{
		exec::XWinMessage,
		image::Image,
		XWin,
	},
	error::XErr,
	glfw_enum,
	window::Pixels,
};

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MouseButton
{
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

pub struct Cursor(*mut GLFWcursor);

impl Cursor
{
	/// Creates a new cursor that can be set for a window with TODO().
	///
	/// See [CursorShape] for more details.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized], [XErr::InvalidValue],
	/// [XErr:CursorUnavailable], and [XErr::Platform]
	pub fn try_new(shape: CursorShape) -> Result<Self, XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?
			.read()
			.unwrap()
			.post_rcv(XWinMessage::CreateCursor(shape, tx), rx)?
			.map(|win| Self::from_glfw(win))
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
