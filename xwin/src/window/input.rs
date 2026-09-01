use std::sync::mpsc::channel;

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
	event::{
		KeyEvent,
		MouseEvent,
	},
	input::{
		ButtonState,
		keyboard::Key,
		mouse::{
			Cursor,
			CursorMode,
			MouseButton,
		},
	},
	window::Window,
};

impl Window
{
	// =======================
	//     EVENT FUNCTIONS
	// =======================

	/// Sets the [Sender] that will be used to send character events, which is
	/// done when a Unicode character is input.
	///
	/// The character channel is intended for Unicode text input. As it deals
	/// with characters, it is keyboard layout dependent, whereas the [key
	/// channel](Window::set_key_channel) is not. Characters fo not map 1:1 to
	/// physical keys, as a key may produce zero, one, or more characters. If
	/// you want to know whether a specific physical key was pressed or
	/// released, see [Window::set_key_channel] instead.
	///
	/// The character channel behaves as system text input normally does and
	/// will not receive events if modifier keys are held down that would
	/// prevent normal text input on that platform. For example a Super
	/// (Command) key on macOS or Alt key on Windows.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized].
	pub fn set_char_channel<T>(&mut self, tx: T) -> Result<(), XErr>
	where
		T: Sender<u32> + Send + Sync + 'static,
	{
		self.with_context(
			"Unable to set char channel when XWin is uninitialized",
			|ctx| ctx.set_char_tx(tx),
		)
	}

	/// Close the char event channel. See [Window::set_char_channel].
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized].
	pub fn clear_char_channel(&mut self) -> Result<(), XErr>
	{
		self.with_context(
			"Unable to clear char channel when XWin is uninitialized",
			|ctx| ctx.remove_char_tx(),
		)
	}

	/// Sets the [Sender] that will be used to send drop events, which is done
	/// when one or more dragged paths are dropped on the window.
	///
	/// The [Vec] of strings sent on the channel are the file and/or directory
	/// path names.
	pub fn set_drop_channel<T>(&mut self, tx: T) -> Result<(), XErr>
	where
		T: Sender<Vec<String>> + Send + Sync + 'static,
	{
		self.with_context(
			"Unable to set drop channel when XWin is uninitialized",
			|ctx| ctx.set_drop_tx(tx),
		)
	}

	/// Close the drop event channel. See [Window::set_drop_channel].
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized].
	pub fn clear_drop_channel(&mut self) -> Result<(), XErr>
	{
		self.with_context(
			"Unable to clear drop channel when XWin is uninitialized",
			|ctx| ctx.remove_drop_tx(),
		)
	}

	/// Sets the [Sender] that will be used to send key events for the window.
	/// Events are sent when a key is pressed, repeated, or released.
	///
	/// The key channel deals with physical keys, with layout independent key
	/// tokens named after their values in the standard US keyboard layout. If
	/// you want to input text, use [Window::set_char_channel] instead.
	///
	/// When a window loses input focus, it will generate synthetic key release
	/// events for all pressed keys with associated key tokens. These synthetic
	/// key events will be sent after the focus loss event has been sent.
	///
	/// The scancode of a key is specific to that platform or sometimes even to
	/// that machine. Scancodes are intended to allow users to bind keys that
	/// don't have an associated key token. Such keys will be sent with
	/// [Key::Unknown]. Their state is not saved and so it cannot be queried
	/// with [Window::key_state].
	///
	/// Sometimes XWin needs to generate synthetic key events, in which case the
	/// scancode may be zero.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized].
	pub fn set_key_channel<T>(&mut self, tx: T) -> Result<(), XErr>
	where
		T: Sender<KeyEvent> + Send + Sync + 'static,
	{
		self.with_context(
			"Unable to set event channel when XWin is uninitialized",
			|ctx| ctx.set_key_tx(tx),
		)
	}

	/// Close the key event channel. See [Window::set_key_channel].
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized].
	pub fn clear_key_channel(&mut self) -> Result<(), XErr>
	{
		self.with_context(
			"Unable to clear event channel when XWin is uninitialized",
			|ctx| ctx.remove_key_tx(),
		)
	}

	/// Sets the [Sender] that will be used to send mouse events for the window.
	/// See [MouseEvent] for the specific conditions under which each event is
	/// sent.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized].
	pub fn set_mouse_channel<T>(&mut self, tx: T) -> Result<(), XErr>
	where
		T: Sender<MouseEvent> + Send + Sync + 'static,
	{
		self.with_context(
			"Unable to set mouse channel when XWin is uninitialized",
			|ctx| ctx.set_mouse_tx(tx),
		)
	}

	/// Close the mouse event channel. See [Window::set_mouse_channel].
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized].
	pub fn clear_mouse_channel(&mut self) -> Result<(), XErr>
	{
		self.with_context(
			"Unable to clear mouse channel when XWin is uninitialized",
			|ctx| ctx.remove_mouse_tx(),
		)
	}

	// =======================
	//     QUERY FUNCTIONS
	// =======================

	/// See [Window::try_cursor_mode].
	pub fn cursor_mode(&self) -> CursorMode
	{
		self.try_cursor_mode().unwrap_or_default()
	}

	/// See [Window::try_cursor_pos].
	pub fn cursor_pos(&self) -> ScreenCoordinates<f64>
	{
		self.try_cursor_pos().unwrap_or_default()
	}

	/// See [Window::try_key_state].
	pub fn key_state(&self, key: Key) -> ButtonState
	{
		self.try_key_state(key).unwrap_or_default()
	}

	/// See [Window::try_lock_key_mods].
	pub fn lock_key_mods(&self) -> bool
	{
		self.try_lock_key_mods().unwrap_or_default()
	}

	/// See [Window::try_mouse_state].
	pub fn mouse_state(&self, button: MouseButton) -> ButtonState
	{
		self.try_mouse_state(button).unwrap_or_default()
	}

	/// See [Window::try_raw_mouse_motion].
	pub fn raw_mouse_motion(&self) -> bool
	{
		self.try_raw_mouse_motion().unwrap_or_default()
	}

	/// See [Window::try_sticky_keys].
	pub fn sticky_keys(&self) -> bool
	{
		self.try_sticky_keys().unwrap_or_default()
	}

	/// See [Window::try_sticky_mouse].
	pub fn sticky_mouse(&self) -> bool
	{
		self.try_sticky_mouse().unwrap_or_default()
	}

	/// See [Window::try_will_mouse_passthrough].
	pub fn will_mouse_passthrough(&self) -> bool
	{
		self.try_will_mouse_passthrough().unwrap_or_default()
	}

	// =======================
	//    UPDATE FUNCTIONS
	// =======================

	/// See [Window::try_set_cursor].
	pub fn set_cursor(&mut self, cursor: Cursor)
	{
		let _ = self.try_set_cursor(cursor);
	}

	/// See [Window::try_set_cursor_mode].
	pub fn set_cursor_mode(&mut self, mode: CursorMode)
	{
		let _ = self.try_set_cursor_mode(mode);
	}

	/// See [Window::try_set_cursor_pos].
	pub fn set_cursor_pos(&mut self, pos: ScreenCoordinates<f64>)
	{
		let _ = self.try_set_cursor_pos(pos);
	}

	/// See [Window::try_set_lock_key_mods].
	pub fn set_lock_key_mods(&mut self, value: bool)
	{
		let _ = self.try_set_lock_key_mods(value);
	}

	/// See [Window::try_set_mouse_passthrough].
	pub fn set_mouse_passthrough(&mut self, value: bool)
	{
		let _ = self.try_set_mouse_passthrough(value);
	}

	/// See [Window::try_set_raw_mouse_motion].
	pub fn set_raw_mouse_motion(&mut self, value: bool)
	{
		let _ = self.try_set_raw_mouse_motion(value);
	}

	/// See [Window::try_set_sticky_keys].
	pub fn set_sticky_keys(&mut self, value: bool)
	{
		let _ = self.try_set_sticky_keys(value);
	}

	/// See [Window::try_set_sticky_mouse].
	pub fn set_sticky_mouse(&mut self, value: bool)
	{
		let _ = self.try_set_sticky_mouse(value);
	}

	// =======================
	//   TRY QUERY FUNCTIONS
	// =======================

	/// Returns the cursor mode of the window.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized].
	pub fn try_cursor_mode(&self) -> Result<CursorMode, XErr>
	{
		self.input_mode(GLFW_CURSOR)
			.map(|v| unsafe { CursorMode::from_glfw(v) })
	}

	/// Returns the position of the cursor, in [ScreenCoordinates], relative to
	/// the upper-left corner of the content area of the window. If the cursor
	/// is [disabled](Window::set_cursor_mode) then the cursor position is
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
			.post_rcv(XWinMessage::GetCursorPos(self.0, tx), rx)?
	}

	/// Returns the last state reported for the specified key to the
	/// window. The returned state is one of [ButtonState::Press] or
	/// [ButtonState::Release]. The action [ButtonState::Repeat] is only
	/// reported during a key event.
	///
	/// If [sticky keys](Window::set_sticky_keys) are enabled, this
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
			XWinMessage::GetKey(self.0, key.as_glfw() as i32, tx),
			rx,
		)?
	}

	/// Returns whether lock key mods are enabled for the window. See
	/// [Window::try_set_lock_key_mods].
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized].
	pub fn try_lock_key_mods(&self) -> Result<bool, XErr>
	{
		self.input_mode(GLFW_LOCK_KEY_MODS)
			.map(|v| if v == GLFW_TRUE { true } else { false })
	}

	/// Returns the last state reported for the specified mouse button to the
	/// window. The returned state is one of [ButtonState::Press] or
	/// [ButtonState::Release].
	///
	/// If [sticky mouse buttons](Window::set_sticky_mouse) are enabled,
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
			XWinMessage::GetMouseButton(self.0, button.as_glfw() as i32, tx),
			rx,
		)?
	}

	/// Returns whether raw mouse motion is enabled for the window. See
	/// [Window::try_set_raw_mouse_motion].
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized].
	pub fn try_raw_mouse_motion(&self) -> Result<bool, XErr>
	{
		self.input_mode(GLFW_RAW_MOUSE_MOTION)
			.map(|v| if v == GLFW_TRUE { true } else { false })
	}

	/// Returns whether sticky keys are enabled for the window. See
	/// [Window::try_set_sticky_keys].
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized].
	pub fn try_sticky_keys(&self) -> Result<bool, XErr>
	{
		self.input_mode(GLFW_STICKY_KEYS)
			.map(|v| if v == GLFW_TRUE { true } else { false })
	}

	/// Returns whether sticky mouse buttons are enabled for the window. See
	/// [Window::try_set_sticky_mouse].
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized].
	pub fn try_sticky_mouse(&self) -> Result<bool, XErr>
	{
		self.input_mode(GLFW_STICKY_MOUSE_BUTTONS)
			.map(|v| if v == GLFW_TRUE { true } else { false })
	}

	/// Indicates whether the window is transparent to mouse input, letting any
	/// mouse events pass through to whatever window is behind it.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized] and [XErr::Platform].
	pub fn try_will_mouse_passthrough(&self) -> Result<bool, XErr>
	{
		self.attr(GLFW_MOUSE_PASSTHROUGH)
	}

	// =======================
	//  TRY UPDATE FUNCTIONS
	// =======================

	/// Sets the cursor image to be used when the cursor is over the content
	/// area of the window. The set cursor will only be visible when the [cursor
	/// mode](Window::try_cursor_mode) of the window is
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
		XWin::get()?
			.read()
			.unwrap()
			.post_rcv(XWinMessage::SetCursor(self.0, cursor.as_glfw(), tx), rx)?
	}

	/// Sets the cursor mode of the window. See [CursorMode].
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized] and [XErr::Platform].
	pub fn try_set_cursor_mode(&mut self, mode: CursorMode) -> Result<(), XErr>
	{
		self.set_input_mode(GLFW_CURSOR, unsafe { mode.as_glfw() })
	}

	/// Sets the position, in [ScreenCoordinates], of the cursor relative to the
	/// upper-left corner of the content area of the window. The window must
	/// have input focus. If the window does not have input focus when this
	/// function is called, it fails silently.
	///
	/// **Do not use this function** to implement things like camera controls.
	/// XWin already provides [CursorMode::Disabled] which hides the cursor,
	/// transparently re-centers it and provides unconstrained cursor motion.
	/// See [Window::set_cursor_mode] for more information.
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
		XWin::get()?
			.read()
			.unwrap()
			.post_rcv(XWinMessage::SetCursorPos(self.0, pos.x, pos.y, tx), rx)?
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

	/// Sets whether the window is transparent to mouse input, letting any mouse
	/// events pass through to whatever window is behind it. Decorated window
	/// with this enabled will behave differently between platforms.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized] and [XErr::Platform].
	pub fn try_set_mouse_passthrough(&mut self, value: bool) -> Result<(), XErr>
	{
		self.set_attr(GLFW_MOUSE_PASSTHROUGH, value)
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

	/// Enables or disables sticky keys on the window.
	///
	/// If sticky keys are enabled, a key press will ensure that
	/// [Window::try_key_state] returns [ButtonState::Press] the next time
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

	/// Enables or disables sticky mouse buttons on the window.
	///
	/// If sticky mouse buttons are enabled, a mouse button press will ensure
	/// that [Window::try_mouse_state] returns [ButtonState::Press] the
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

	// =======================
	//    PRIVATE FUNCTIONS
	// =======================

	fn input_mode(&self, mode: u32) -> Result<u32, XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?
			.read()
			.unwrap()
			.post_rcv(XWinMessage::GetInputMode(self.0, mode as i32, tx), rx)?
			.map(|v| v as u32)
	}

	fn set_input_mode(&mut self, mode: u32, value: u32) -> Result<(), XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?.read().unwrap().post_rcv(
			XWinMessage::SetInputMode(self.0, mode as i32, value as i32, tx),
			rx,
		)?
	}
}
