use std::{
	ops::{
		BitAnd,
		BitAndAssign,
		BitOr,
		BitOrAssign,
		BitXor,
		BitXorAssign,
	},
	sync::mpsc::channel,
};

use crate::{
	bind::{
		GLFW_KEY_0,
		GLFW_KEY_1,
		GLFW_KEY_2,
		GLFW_KEY_3,
		GLFW_KEY_4,
		GLFW_KEY_5,
		GLFW_KEY_6,
		GLFW_KEY_7,
		GLFW_KEY_8,
		GLFW_KEY_9,
		GLFW_KEY_A,
		GLFW_KEY_APOSTROPHE,
		GLFW_KEY_B,
		GLFW_KEY_BACKSLASH,
		GLFW_KEY_BACKSPACE,
		GLFW_KEY_C,
		GLFW_KEY_CAPS_LOCK,
		GLFW_KEY_COMMA,
		GLFW_KEY_D,
		GLFW_KEY_DELETE,
		GLFW_KEY_DOWN,
		GLFW_KEY_E,
		GLFW_KEY_END,
		GLFW_KEY_ENTER,
		GLFW_KEY_EQUAL,
		GLFW_KEY_ESCAPE,
		GLFW_KEY_F,
		GLFW_KEY_F1,
		GLFW_KEY_F2,
		GLFW_KEY_F3,
		GLFW_KEY_F4,
		GLFW_KEY_F5,
		GLFW_KEY_F6,
		GLFW_KEY_F7,
		GLFW_KEY_F8,
		GLFW_KEY_F9,
		GLFW_KEY_F10,
		GLFW_KEY_F11,
		GLFW_KEY_F12,
		GLFW_KEY_F13,
		GLFW_KEY_F14,
		GLFW_KEY_F15,
		GLFW_KEY_F16,
		GLFW_KEY_F17,
		GLFW_KEY_F18,
		GLFW_KEY_F19,
		GLFW_KEY_F20,
		GLFW_KEY_F21,
		GLFW_KEY_F22,
		GLFW_KEY_F23,
		GLFW_KEY_F24,
		GLFW_KEY_F25,
		GLFW_KEY_G,
		GLFW_KEY_GRAVE_ACCENT,
		GLFW_KEY_H,
		GLFW_KEY_HOME,
		GLFW_KEY_I,
		GLFW_KEY_INSERT,
		GLFW_KEY_J,
		GLFW_KEY_K,
		GLFW_KEY_KP_0,
		GLFW_KEY_KP_1,
		GLFW_KEY_KP_2,
		GLFW_KEY_KP_3,
		GLFW_KEY_KP_4,
		GLFW_KEY_KP_5,
		GLFW_KEY_KP_6,
		GLFW_KEY_KP_7,
		GLFW_KEY_KP_8,
		GLFW_KEY_KP_9,
		GLFW_KEY_KP_ADD,
		GLFW_KEY_KP_DECIMAL,
		GLFW_KEY_KP_DIVIDE,
		GLFW_KEY_KP_ENTER,
		GLFW_KEY_KP_EQUAL,
		GLFW_KEY_KP_MULTIPLY,
		GLFW_KEY_KP_SUBTRACT,
		GLFW_KEY_L,
		GLFW_KEY_LEFT,
		GLFW_KEY_LEFT_ALT,
		GLFW_KEY_LEFT_BRACKET,
		GLFW_KEY_LEFT_CONTROL,
		GLFW_KEY_LEFT_SHIFT,
		GLFW_KEY_LEFT_SUPER,
		GLFW_KEY_M,
		GLFW_KEY_MENU,
		GLFW_KEY_MINUS,
		GLFW_KEY_N,
		GLFW_KEY_NUM_LOCK,
		GLFW_KEY_O,
		GLFW_KEY_P,
		GLFW_KEY_PAGE_DOWN,
		GLFW_KEY_PAGE_UP,
		GLFW_KEY_PAUSE,
		GLFW_KEY_PERIOD,
		GLFW_KEY_PRINT_SCREEN,
		GLFW_KEY_Q,
		GLFW_KEY_R,
		GLFW_KEY_RIGHT,
		GLFW_KEY_RIGHT_ALT,
		GLFW_KEY_RIGHT_BRACKET,
		GLFW_KEY_RIGHT_CONTROL,
		GLFW_KEY_RIGHT_SHIFT,
		GLFW_KEY_RIGHT_SUPER,
		GLFW_KEY_S,
		GLFW_KEY_SCROLL_LOCK,
		GLFW_KEY_SEMICOLON,
		GLFW_KEY_SLASH,
		GLFW_KEY_SPACE,
		GLFW_KEY_T,
		GLFW_KEY_TAB,
		GLFW_KEY_U,
		GLFW_KEY_UNKNOWN,
		GLFW_KEY_UP,
		GLFW_KEY_V,
		GLFW_KEY_W,
		GLFW_KEY_WORLD_1,
		GLFW_KEY_WORLD_2,
		GLFW_KEY_X,
		GLFW_KEY_Y,
		GLFW_KEY_Z,
		GLFW_MOD_ALT,
		GLFW_MOD_CAPS_LOCK,
		GLFW_MOD_CONTROL,
		GLFW_MOD_NUM_LOCK,
		GLFW_MOD_SHIFT,
		GLFW_MOD_SUPER,
		glfwGetKeyScancode,
	},
	core::{
		XWin,
		exec::XWinMessage,
	},
	error::XErr,
	glfw_enum,
};

#[repr(i16)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key
{
	#[default]
	Unknown        = GLFW_KEY_UNKNOWN as i16,
	Space          = GLFW_KEY_SPACE as i16,
	Apostrophe     = GLFW_KEY_APOSTROPHE as i16,
	Comma          = GLFW_KEY_COMMA as i16,
	Minus          = GLFW_KEY_MINUS as i16,
	Period         = GLFW_KEY_PERIOD as i16,
	Slash          = GLFW_KEY_SLASH as i16,
	Zero           = GLFW_KEY_0 as i16,
	One            = GLFW_KEY_1 as i16,
	Two            = GLFW_KEY_2 as i16,
	Three          = GLFW_KEY_3 as i16,
	Four           = GLFW_KEY_4 as i16,
	Five           = GLFW_KEY_5 as i16,
	Six            = GLFW_KEY_6 as i16,
	Seven          = GLFW_KEY_7 as i16,
	Eight          = GLFW_KEY_8 as i16,
	Nine           = GLFW_KEY_9 as i16,
	Semicolon      = GLFW_KEY_SEMICOLON as i16,
	Equal          = GLFW_KEY_EQUAL as i16,
	A              = GLFW_KEY_A as i16,
	B              = GLFW_KEY_B as i16,
	C              = GLFW_KEY_C as i16,
	D              = GLFW_KEY_D as i16,
	E              = GLFW_KEY_E as i16,
	F              = GLFW_KEY_F as i16,
	G              = GLFW_KEY_G as i16,
	H              = GLFW_KEY_H as i16,
	I              = GLFW_KEY_I as i16,
	J              = GLFW_KEY_J as i16,
	K              = GLFW_KEY_K as i16,
	L              = GLFW_KEY_L as i16,
	M              = GLFW_KEY_M as i16,
	N              = GLFW_KEY_N as i16,
	O              = GLFW_KEY_O as i16,
	P              = GLFW_KEY_P as i16,
	Q              = GLFW_KEY_Q as i16,
	R              = GLFW_KEY_R as i16,
	S              = GLFW_KEY_S as i16,
	T              = GLFW_KEY_T as i16,
	U              = GLFW_KEY_U as i16,
	V              = GLFW_KEY_V as i16,
	W              = GLFW_KEY_W as i16,
	X              = GLFW_KEY_X as i16,
	Y              = GLFW_KEY_Y as i16,
	Z              = GLFW_KEY_Z as i16,
	LeftBracket    = GLFW_KEY_LEFT_BRACKET as i16,
	Backslash      = GLFW_KEY_BACKSLASH as i16,
	RightBracket   = GLFW_KEY_RIGHT_BRACKET as i16,
	Grave          = GLFW_KEY_GRAVE_ACCENT as i16,
	World1         = GLFW_KEY_WORLD_1 as i16,
	World2         = GLFW_KEY_WORLD_2 as i16,
	Escape         = GLFW_KEY_ESCAPE as i16,
	Enter          = GLFW_KEY_ENTER as i16,
	Tab            = GLFW_KEY_TAB as i16,
	Backspace      = GLFW_KEY_BACKSPACE as i16,
	Insert         = GLFW_KEY_INSERT as i16,
	Delete         = GLFW_KEY_DELETE as i16,
	Right          = GLFW_KEY_RIGHT as i16,
	Left           = GLFW_KEY_LEFT as i16,
	Down           = GLFW_KEY_DOWN as i16,
	Up             = GLFW_KEY_UP as i16,
	PageUp         = GLFW_KEY_PAGE_UP as i16,
	PageDown       = GLFW_KEY_PAGE_DOWN as i16,
	Home           = GLFW_KEY_HOME as i16,
	End            = GLFW_KEY_END as i16,
	CapsLock       = GLFW_KEY_CAPS_LOCK as i16,
	ScrollLock     = GLFW_KEY_SCROLL_LOCK as i16,
	NumLock        = GLFW_KEY_NUM_LOCK as i16,
	PrintScreen    = GLFW_KEY_PRINT_SCREEN as i16,
	Pause          = GLFW_KEY_PAUSE as i16,
	F1             = GLFW_KEY_F1 as i16,
	F2             = GLFW_KEY_F2 as i16,
	F3             = GLFW_KEY_F3 as i16,
	F4             = GLFW_KEY_F4 as i16,
	F5             = GLFW_KEY_F5 as i16,
	F6             = GLFW_KEY_F6 as i16,
	F7             = GLFW_KEY_F7 as i16,
	F8             = GLFW_KEY_F8 as i16,
	F9             = GLFW_KEY_F9 as i16,
	F10            = GLFW_KEY_F10 as i16,
	F11            = GLFW_KEY_F11 as i16,
	F12            = GLFW_KEY_F12 as i16,
	F13            = GLFW_KEY_F13 as i16,
	F14            = GLFW_KEY_F14 as i16,
	F15            = GLFW_KEY_F15 as i16,
	F16            = GLFW_KEY_F16 as i16,
	F17            = GLFW_KEY_F17 as i16,
	F18            = GLFW_KEY_F18 as i16,
	F19            = GLFW_KEY_F19 as i16,
	F20            = GLFW_KEY_F20 as i16,
	F21            = GLFW_KEY_F21 as i16,
	F22            = GLFW_KEY_F22 as i16,
	F23            = GLFW_KEY_F23 as i16,
	F24            = GLFW_KEY_F24 as i16,
	F25            = GLFW_KEY_F25 as i16,
	Keypad0        = GLFW_KEY_KP_0 as i16,
	Keypad1        = GLFW_KEY_KP_1 as i16,
	Keypad2        = GLFW_KEY_KP_2 as i16,
	Keypad3        = GLFW_KEY_KP_3 as i16,
	Keypad4        = GLFW_KEY_KP_4 as i16,
	Keypad5        = GLFW_KEY_KP_5 as i16,
	Keypad6        = GLFW_KEY_KP_6 as i16,
	Keypad7        = GLFW_KEY_KP_7 as i16,
	Keypad8        = GLFW_KEY_KP_8 as i16,
	Keypad9        = GLFW_KEY_KP_9 as i16,
	KeypadDecimal  = GLFW_KEY_KP_DECIMAL as i16,
	KeypadDivide   = GLFW_KEY_KP_DIVIDE as i16,
	KeypadMultiply = GLFW_KEY_KP_MULTIPLY as i16,
	KeypadSubtract = GLFW_KEY_KP_SUBTRACT as i16,
	KeypadAdd      = GLFW_KEY_KP_ADD as i16,
	KeypadEnter    = GLFW_KEY_KP_ENTER as i16,
	KeypadEqual    = GLFW_KEY_KP_EQUAL as i16,
	LeftShift      = GLFW_KEY_LEFT_SHIFT as i16,
	LeftControl    = GLFW_KEY_LEFT_CONTROL as i16,
	LeftAlt        = GLFW_KEY_LEFT_ALT as i16,
	LeftSuper      = GLFW_KEY_LEFT_SUPER as i16,
	RightShift     = GLFW_KEY_RIGHT_SHIFT as i16,
	RightControl   = GLFW_KEY_RIGHT_CONTROL as i16,
	RightAlt       = GLFW_KEY_RIGHT_ALT as i16,
	RightSuper     = GLFW_KEY_RIGHT_SUPER as i16,
	Menu           = GLFW_KEY_MENU as i16,
}
glfw_enum!(Key, i16);

impl Key
{
	/// Returns the name of the specified printable key. This is typically the
	/// character that key would produce without any modifier keys, intended for
	/// displaying key bindings to the user. For dead keys, it is typically the
	/// diacritic it would add to a character.
	///
	/// **Do not use this function for text input**. You will break text input
	/// for many languages even if it happens to work for yours.
	///
	/// If you specify a non-printable key, this function returns `None`.
	///
	/// Names for printable keys depend on keyboard layout, while names for
	/// non-printable keys are the same across layouts but depend on the
	/// application language and should be localized along with other user
	/// interface text.
	///
	/// Printable keys:
	/// - [Key::Apostrophe]
	/// - [Key::Comma]
	/// - [Key::Minus]
	/// - [Key::Period]
	/// - [Key::Slash]
	/// - [Key::Semicolon]
	/// - [Key::Equal]
	/// - [Key::LeftBracket]
	/// - [Key::RightBracket]
	/// - [Key::Backslash]
	/// - [Key::World1]
	/// - [Key::World2]
	/// - [Key::Zero] to [Key::Nine]
	/// - [Key::A] to [Key::Z]
	/// - [Key::Keypad0] to [Key::Keypad9]
	/// - [Key::KeypadDecimal]
	/// - [Key::KeypadDivide]
	/// - [Key::KeypadMultiply]
	/// - [Key::KeypadSubtract]
	/// - [Key::KeypadAdd]
	/// - [Key::KeypadEqual]
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized]
	pub fn try_name(&self) -> Result<Option<String>, XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?.read().unwrap().post_rcv(
			XWinMessage::GetKeyName(unsafe { self.as_glfw() } as i32, -1, tx),
			rx,
		)?
	}

	/// See [Key::try_name].
	pub fn name(&self) -> Option<String>
	{
		self.try_name().unwrap_or_default()
	}

	/// Returns the platform-specific scancode of the specified key.
	///
	/// If `key` corresponds to a physical key not supported on the current
	/// platform then this method will return `-1`.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized].
	pub fn try_scancode(&self) -> Result<i32, XErr>
	{
		let value = unsafe { glfwGetKeyScancode(self.as_glfw() as i32) };
		XErr::result(|| value)
	}
}

/// Bitmask containing the state of modifier keys sent along with key events.
///
/// Contains bit flags for [Modifiers::ALT], [Modifiers::CAPS_LOCK],
/// [Modifiers::CONTROL], [Modifiers::NUM_LOCK], [Modifiers::SHIFT], and
/// [Modifiers::SUPER].
///
/// Bitwise operators may be used to manipulate these flags. Utility functions
/// have been provided for easily querying specific modifiers.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Modifiers(u8);

impl Modifiers
{
	pub const ALT: Modifiers = Modifiers(GLFW_MOD_ALT as u8);
	pub const CAPS_LOCK: Modifiers = Modifiers(GLFW_MOD_CAPS_LOCK as u8);
	pub const CONTROL: Modifiers = Modifiers(GLFW_MOD_CONTROL as u8);
	pub const NUM_LOCK: Modifiers = Modifiers(GLFW_MOD_NUM_LOCK as u8);
	pub const SHIFT: Modifiers = Modifiers(GLFW_MOD_SHIFT as u8);
	pub const SUPER: Modifiers = Modifiers(GLFW_MOD_SUPER as u8);

	/// Returns whether the [ALT](Modifiers::ALT) flag is set.
	pub fn is_alt(&self) -> bool
	{
		self.0 & GLFW_MOD_ALT as u8 > 0
	}

	/// Returns whether the [CAPS_LOCK](Modifiers::CAPS_LOCK) flag is set.
	pub fn is_caps_lock(&self) -> bool
	{
		self.0 & GLFW_MOD_CAPS_LOCK as u8 > 0
	}

	/// Returns whether the [CONTROL](Modifiers::CONTROL) flag is set.
	pub fn is_control(&self) -> bool
	{
		self.0 & GLFW_MOD_CONTROL as u8 > 0
	}

	/// Returns whether the [NUM_LOCK](Modifiers::NUM_LOCK) flag is set.
	pub fn is_num_lock(&self) -> bool
	{
		self.0 & GLFW_MOD_NUM_LOCK as u8 > 0
	}

	/// Returns whether the [SHIFT](Modifiers::SHIFT) flag is set.
	pub fn is_shift(&self) -> bool
	{
		self.0 & GLFW_MOD_SHIFT as u8 > 0
	}

	/// Returns whether the [SUPER](Modifiers::SUPER) flag is set.
	pub fn is_super(&self) -> bool
	{
		self.0 & GLFW_MOD_SUPER as u8 > 0
	}

	#[cfg(feature = "glfw")]
	pub fn from_glfw(value: u8) -> Self
	{
		Self::from_glfw_crate(value as i32)
	}

	#[cfg(feature = "glfw")]
	pub fn as_glfw(&self) -> u8
	{
		self.0
	}

	pub(crate) fn from_glfw_crate(value: i32) -> Modifiers
	{
		Modifiers((value & 0xff) as u8)
	}
}

impl BitAnd<Modifiers> for Modifiers
{
	type Output = Modifiers;

	fn bitand(self, rhs: Modifiers) -> Modifiers
	{
		Modifiers(self.0 & rhs.0)
	}
}

impl BitAndAssign<Modifiers> for Modifiers
{
	fn bitand_assign(&mut self, rhs: Modifiers)
	{
		self.0 &= rhs.0;
	}
}

impl BitOr<Modifiers> for Modifiers
{
	type Output = Modifiers;

	fn bitor(self, rhs: Modifiers) -> Modifiers
	{
		Modifiers(self.0 | rhs.0)
	}
}

impl BitOrAssign<Modifiers> for Modifiers
{
	fn bitor_assign(&mut self, rhs: Modifiers)
	{
		self.0 |= rhs.0;
	}
}

impl BitXor<Modifiers> for Modifiers
{
	type Output = Modifiers;

	fn bitxor(self, rhs: Modifiers) -> Modifiers
	{
		Modifiers(self.0 ^ rhs.0)
	}
}

impl BitXorAssign<Modifiers> for Modifiers
{
	fn bitxor_assign(&mut self, rhs: Modifiers)
	{
		self.0 ^= rhs.0;
	}
}

/// See [try_key_name].
pub fn try_scancode_name(scancode: i32) -> Result<Option<String>, XErr>
{
	let (tx, rx) = channel();
	XWin::get()?
		.read()
		.unwrap()
		.post_rcv(XWinMessage::GetKeyName(-1, scancode, tx), rx)?
}

/// See [try_key_name].
pub fn scancode_name(scancode: i32) -> Option<String>
{
	try_scancode_name(scancode).unwrap_or_default()
}
