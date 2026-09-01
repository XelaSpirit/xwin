use std::{
	ptr,
	sync::mpsc::channel,
};

use xch::Sender;

use crate::{
	bind::{
		GLFW_CURSOR,
		GLFW_FALSE,
		GLFW_LOCK_KEY_MODS,
		GLFW_MOUSE_PASSTHROUGH,
		GLFW_RAW_MOUSE_MOTION,
		GLFW_STICKY_KEYS,
		GLFW_STICKY_MOUSE_BUTTONS,
		GLFW_TRUE,
	},
	core::{
		ScreenCoordinates,
		XWin,
		exec::XWinMessage,
	},
	error::XErr,
	input::{
		ButtonState,
		keyboard::Key,
		mouse::{
			Cursor,
			CursorMode,
			MouseButton,
		},
	},
	window::{
		KeyEvent,
		Window,
		ctx::WindowContext,
	},
};

pub struct WindowInput<'a>(&'a mut Window);
impl PartialEq for WindowInput<'_>
{
	fn eq(&self, other: &Self) -> bool
	{
		ptr::eq(self.0, other.0)
	}
}
impl Eq for WindowInput<'_> {}

impl<'a> WindowInput<'a>
{
	/// TODO - events

	/// Sets the [Sender] that will be used to send key events for the window.
	pub fn set_key_channel<T>(&mut self, tx: T) -> Result<(), XErr>
	where
		T: Sender<KeyEvent> + Send + Sync + 'static,
	{
		if let Some(ctx) = WindowContext::get(&self.0.as_glfw())
		{
			ctx.set_key_tx(tx);
			Ok(())
		}
		else
		{
			Err(XErr::NotInitialized(
				"Unable to set event channels when XWin is uninitialized".to_string(),
			))
		}
	}

	pub fn clear_key_channel<T>(&mut self) -> Result<(), XErr>
	{
		if let Some(ctx) = WindowContext::get(&self.0.as_glfw())
		{
			ctx.remove_key_tx();
			Ok(())
		}
		else
		{
			Err(XErr::NotInitialized(
				"Unable to set event channels when XWin is uninitialized".to_string(),
			))
		}
	}

	/// Indicates whether the window is transparent to mouse input, letting any
	/// mouse events pass through to whatever window is behind it.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized] and [XErr::Platform].
	pub fn try_will_mouse_passthrough(&self) -> Result<bool, XErr>
	{
		self.0.attr(GLFW_MOUSE_PASSTHROUGH)
	}

	/// See [WindowInput::try_will_mouse_passthrough].
	pub fn will_mouse_passthrough(&self) -> bool
	{
		self.try_will_mouse_passthrough().unwrap_or_default()
	}

	/// Sets whether the window is transparent to mouse input, letting any mouse
	/// events pass through to whatever window is behind it. Decorated window
	/// with this enabled will behave differently between platforms.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized] and [XErr::Platform].
	pub fn try_set_mouse_passthrough(&mut self, value: bool) -> Result<(), XErr>
	{
		self.0.set_attr(GLFW_MOUSE_PASSTHROUGH, value)
	}

	/// See [WindowInput::try_set_mouse_passthrough].
	pub fn set_mouse_passthrough(&mut self, value: bool)
	{
		let _ = self.try_set_mouse_passthrough(value);
	}

	/// Returns the cursor mode of the window.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized].
	pub fn try_cursor_mode(&self) -> Result<CursorMode, XErr>
	{
		self.input_mode(GLFW_CURSOR)
			.map(|v| CursorMode::from_glfw(v))
	}

	/// See [WindowInput::try_cursor_mode].
	pub fn cursor_mode(&self) -> CursorMode
	{
		self.try_cursor_mode().unwrap_or_default()
	}

	/// Sets the cursor mode of the window. See [CursorMode].
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized] and [XErr::Platform].
	pub fn try_set_cursor_mode(&mut self, mode: CursorMode) -> Result<(), XErr>
	{
		self.set_input_mode(GLFW_CURSOR, mode.as_glfw())
	}

	/// See [WindowInput::try_set_cursor_mode].
	pub fn set_cursor_mode(&mut self, mode: CursorMode)
	{
		let _ = self.try_set_cursor_mode(mode);
	}

	/// Returns whether sticky keys are enabled for the window. See
	/// [WindowInput::try_set_sticky_keys].
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized].
	pub fn try_sticky_keys(&self) -> Result<bool, XErr>
	{
		self.input_mode(GLFW_STICKY_KEYS)
			.map(|v| if v == GLFW_TRUE { true } else { false })
	}

	/// See [WindowInput::try_sticky_keys].
	pub fn sticky_keys(&self) -> bool
	{
		self.try_sticky_keys().unwrap_or_default()
	}

	/// Enables or disables sticky keys on the window.
	///
	/// If sticky keys are enabled, a key press will ensure that
	/// [WindowInput::try_key_state] returns [ButtonState::Press] the next time
	/// it is called even if the key had been released before the call. This is
	/// useful when you are only interested in whether keys have been pressed
	/// but not when or in which order.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized] and [XErr::Platform].
	pub fn try_set_sticky_keys(&mut self, value: bool) -> Result<(), XErr>
	{
		self.set_input_mode(GLFW_STICKY_KEYS, if value { GLFW_TRUE } else { GLFW_FALSE })
	}

	/// See [WindowInput::try_set_sticky_keys].
	pub fn set_sticky_keys(&mut self, value: bool)
	{
		let _ = self.try_set_sticky_keys(value);
	}

	/// Returns whether sticky mouse buttons are enabled for the window. See
	/// [WindowInput::try_set_sticky_mouse].
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized].
	pub fn try_sticky_mouse(&self) -> Result<bool, XErr>
	{
		self.input_mode(GLFW_STICKY_MOUSE_BUTTONS)
			.map(|v| if v == GLFW_TRUE { true } else { false })
	}

	/// See [WindowInput::try_sticky_mouse].
	pub fn sticky_mouse(&self) -> bool
	{
		self.try_sticky_mouse().unwrap_or_default()
	}

	/// Enables or disables sticky mouse buttons on the window.
	///
	/// If sticky mouse buttons are enabled, a mouse button press will ensure
	/// that [WindowInput::try_mouse_state] returns [ButtonState::Press] the
	/// next time it is called even if the mouse button has been released
	/// before the call. This is useful when you are only interested in whether
	/// mouse buttons have been pressed but not when or in which order.
	///
	/// TODO finish linking PRESS
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized] and [XErr::Platform].
	pub fn try_set_sticky_mouse(&mut self, value: bool) -> Result<(), XErr>
	{
		self.set_input_mode(
			GLFW_STICKY_MOUSE_BUTTONS,
			if value { GLFW_TRUE } else { GLFW_FALSE },
		)
	}

	/// See [WindowInput::try_set_sticky_mouse].
	pub fn set_sticky_mouse(&mut self, value: bool)
	{
		let _ = self.try_set_sticky_mouse(value);
	}

	/// Returns whether lock key mods are enabled for the window. See
	/// [WindowInput::try_set_lock_key_mods].
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized].
	pub fn try_lock_key_mods(&self) -> Result<bool, XErr>
	{
		self.input_mode(GLFW_LOCK_KEY_MODS)
			.map(|v| if v == GLFW_TRUE { true } else { false })
	}

	/// See [WindowInput::try_lock_key_mods].
	pub fn lock_key_mods(&self) -> bool
	{
		self.try_lock_key_mods().unwrap_or_default()
	}

	/// Enables or disables lock key modifier bits.
	///
	/// If enabled, events that send modifier bits will also have the
	/// [Modifier::CAPS_LOCK](crate::input::keyboard::Modifiers::CAPS_LOCK) bit
	/// set when the event was generated with Caps Lock on, and the
	/// [Modifier::NUM_LOCK](crate::input::keyboard::Modifiers::NUM_LOCK) bit
	/// when Num Lock was on.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized] and [XErr::Platform].
	pub fn try_set_lock_key_mods(&mut self, value: bool) -> Result<(), XErr>
	{
		self.set_input_mode(
			GLFW_LOCK_KEY_MODS,
			if value { GLFW_TRUE } else { GLFW_FALSE },
		)
	}

	/// See [WindowInput::try_set_lock_key_mods].
	pub fn set_lock_key_mods(&mut self, value: bool)
	{
		let _ = self.try_set_lock_key_mods(value);
	}

	/// Returns whether raw mouse motion is enabled for the window. See
	/// [WindowInput::try_set_raw_mouse_motion].
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized].
	pub fn try_raw_mouse_motion(&self) -> Result<bool, XErr>
	{
		self.input_mode(GLFW_RAW_MOUSE_MOTION)
			.map(|v| if v == GLFW_TRUE { true } else { false })
	}

	/// See [WindowInput::try_raw_mouse_motion].
	pub fn raw_mouse_motion(&self) -> bool
	{
		self.try_raw_mouse_motion().unwrap_or_default()
	}

	/// Enabled or disabled raw mouse motion for the window.
	///
	/// When enabled, raw (unscaled and unaccelerated) mouse motion will be used
	/// for mouse events. This will only be used when the cursor is disabled.
	///
	/// If raw mouse motion is not supported, this will return
	/// [XErr::FeatureUnavailable]. See
	/// [try_raw_mouse_supported](crate::input::mouse::try_raw_mouse_supported).
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized], [XErr::Platform], and
	/// [XErr::FeatureUnavailable].
	pub fn try_set_raw_mouse_motion(&mut self, value: bool) -> Result<(), XErr>
	{
		self.set_input_mode(
			GLFW_RAW_MOUSE_MOTION,
			if value { GLFW_TRUE } else { GLFW_FALSE },
		)
	}

	/// See [WindowInput::try_set_raw_mouse_motion].
	pub fn set_raw_mouse_motion(&mut self, value: bool)
	{
		let _ = self.try_set_raw_mouse_motion(value);
	}

	/// Returns the last state reported for the specified key to the
	/// window. The returned state is one of [ButtonState::Press] or
	/// [ButtonState::Release]. The action [ButtonState::Repeat] is only
	/// reported during a key event.
	///
	/// If [sticky keys](WindowInput::set_sticky_keys) are enabled, this
	/// function returns [ButtonState::Press] the first time you call it for a
	/// key that was pressed, even if that key has already been released.
	///
	/// **Do not use this function** to implement text input.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized].
	pub fn try_key_state(&self, key: Key) -> Result<ButtonState, XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?.read().unwrap().post_rcv(
			XWinMessage::GetKey(self.0.as_glfw(), key.as_glfw() as i32, tx),
			rx,
		)?
	}

	/// See [WindowInput::try_key_state].
	pub fn key_state(&self, key: Key) -> ButtonState
	{
		self.try_key_state(key).unwrap_or_default()
	}

	/// Returns the last state reported for the specified mouse button to the
	/// window. The returned state is one of [ButtonState::Press] or
	/// [ButtonState::Release].
	///
	/// If [sticky mouse buttons](WindowInput::set_sticky_mouse) are enabled,
	/// this function returns [ButtonState::Press] the first time you call it
	/// for a mouse button that was pressed, even if that mouse button has
	/// already been released.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized].
	pub fn try_mouse_state(&self, button: MouseButton) -> Result<ButtonState, XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?.read().unwrap().post_rcv(
			XWinMessage::GetMouseButton(self.0.as_glfw(), button.as_glfw() as i32, tx),
			rx,
		)?
	}

	/// See [WindowInput::try_mouse_state].
	pub fn mouse_state(&self, button: MouseButton) -> ButtonState
	{
		self.try_mouse_state(button).unwrap_or_default()
	}

	/// Returns the position of the cursor, in [ScreenCoordinates], relative to
	/// the upper-left corner of the content area of the window. If the cursor
	/// is [disabled](WindowInput::set_cursor_mode) then the cursor position is
	/// unbounded and limited only by the minimum and maximum values of a
	/// double.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized] and [XErr::Platform].
	pub fn try_cursor_pos(&self) -> Result<ScreenCoordinates<f64>, XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?
			.read()
			.unwrap()
			.post_rcv(XWinMessage::GetCursorPos(self.0.as_glfw(), tx), rx)?
	}

	/// See [WindowInput::try_cursor_pos].
	pub fn cursor_pos(&self) -> ScreenCoordinates<f64>
	{
		self.try_cursor_pos().unwrap_or_default()
	}

	/// Sets the position, in [ScreenCoordinates], of the cursor relative to the
	/// upper-left corner of the content area of the window. The window must
	/// have input focus. If the window does not have input focus when this
	/// function is called, it fails silently.
	///
	/// **Do not use this function** to implement things like camera controls.
	/// XWin already provides [CursorMode::Disabled] which hides the cursor,
	/// transparently re-centers it and provides unconstrained cursor motion.
	/// See [WindowInput::set_cursor_mode] for more information.
	///
	/// If the cursor mode is [CursorMode::Disabled], then the cursor position
	/// is unconstrained and limited only by the maximum and minimum values of
	/// `f64`.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized], [XErr::Platform], and
	/// [XErr::FeatureUnavailable] (See remarks).
	///
	/// # Remarks
	/// - **Wayland.** This function will only work when the cursor mode is
	///   [CursorMode::Disabled], otherwise it will return
	///   [XErr::FeatureUnavailable].
	pub fn try_set_cursor_pos(&mut self, pos: ScreenCoordinates<f64>) -> Result<(), XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?.read().unwrap().post_rcv(
			XWinMessage::SetCursorPos(self.0.as_glfw(), pos.x, pos.y, tx),
			rx,
		)?
	}

	/// See [WindowInput::try_set_cursor_pos].
	pub fn set_cursor_pos(&mut self, pos: ScreenCoordinates<f64>)
	{
		let _ = self.try_set_cursor_pos(pos);
	}

	/// Sets the cursor image to be used when the cursor is over the content
	/// area of the window. The set cursor will only be visible when the [cursor
	/// mode](WindowInput::try_cursor_mode) of the window is
	/// [CursorMode::Normal].
	///
	/// On some platforms, the set cursor may not be visible unless the window
	/// also has input focus.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized] and [XErr::Platform].
	pub fn try_set_cursor(&mut self, cursor: Cursor) -> Result<(), XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?.read().unwrap().post_rcv(
			XWinMessage::SetCursor(self.0.as_glfw(), cursor.as_glfw(), tx),
			rx,
		)?
	}

	/// See [WindowInput::try_set_cursor].
	pub fn set_cursor(&mut self, cursor: Cursor)
	{
		let _ = self.try_set_cursor(cursor);
	}

	pub(super) fn new(window: &'a mut Window) -> Self
	{
		Self(window)
	}

	fn input_mode(&self, mode: u32) -> Result<u32, XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?
			.read()
			.unwrap()
			.post_rcv(
				XWinMessage::GetInputMode(self.0.as_glfw(), mode as i32, tx),
				rx,
			)?
			.map(|v| v as u32)
	}

	fn set_input_mode(&mut self, mode: u32, value: u32) -> Result<(), XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?.read().unwrap().post_rcv(
			XWinMessage::SetInputMode(self.0.as_glfw(), mode as i32, value as i32, tx),
			rx,
		)?
	}
}
