use std::ops::{
	BitAnd,
	BitAndAssign,
	BitOr,
	BitOrAssign,
	BitXor,
	BitXorAssign,
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
		GLFW_KEY_F2,
		GLFW_KEY_F20,
		GLFW_KEY_F21,
		GLFW_KEY_F22,
		GLFW_KEY_F23,
		GLFW_KEY_F24,
		GLFW_KEY_F25,
		GLFW_KEY_F3,
		GLFW_KEY_F4,
		GLFW_KEY_F5,
		GLFW_KEY_F6,
		GLFW_KEY_F7,
		GLFW_KEY_F8,
		GLFW_KEY_F9,
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
	},
	glfw_enum,
};

#[repr(u16)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Key
{
	Space          = GLFW_KEY_SPACE as u16,
	Apostrophe     = GLFW_KEY_APOSTROPHE as u16,
	Comma          = GLFW_KEY_COMMA as u16,
	Minus          = GLFW_KEY_MINUS as u16,
	Period         = GLFW_KEY_PERIOD as u16,
	Slash          = GLFW_KEY_SLASH as u16,
	Zero           = GLFW_KEY_0 as u16,
	One            = GLFW_KEY_1 as u16,
	Two            = GLFW_KEY_2 as u16,
	Three          = GLFW_KEY_3 as u16,
	Four           = GLFW_KEY_4 as u16,
	Five           = GLFW_KEY_5 as u16,
	Six            = GLFW_KEY_6 as u16,
	Seven          = GLFW_KEY_7 as u16,
	Eight          = GLFW_KEY_8 as u16,
	Nine           = GLFW_KEY_9 as u16,
	Semicolon      = GLFW_KEY_SEMICOLON as u16,
	Equal          = GLFW_KEY_EQUAL as u16,
	A              = GLFW_KEY_A as u16,
	B              = GLFW_KEY_B as u16,
	C              = GLFW_KEY_C as u16,
	D              = GLFW_KEY_D as u16,
	E              = GLFW_KEY_E as u16,
	F              = GLFW_KEY_F as u16,
	G              = GLFW_KEY_G as u16,
	H              = GLFW_KEY_H as u16,
	I              = GLFW_KEY_I as u16,
	J              = GLFW_KEY_J as u16,
	K              = GLFW_KEY_K as u16,
	L              = GLFW_KEY_L as u16,
	M              = GLFW_KEY_M as u16,
	N              = GLFW_KEY_N as u16,
	O              = GLFW_KEY_O as u16,
	P              = GLFW_KEY_P as u16,
	Q              = GLFW_KEY_Q as u16,
	R              = GLFW_KEY_R as u16,
	S              = GLFW_KEY_S as u16,
	T              = GLFW_KEY_T as u16,
	U              = GLFW_KEY_U as u16,
	V              = GLFW_KEY_V as u16,
	W              = GLFW_KEY_W as u16,
	X              = GLFW_KEY_X as u16,
	Y              = GLFW_KEY_Y as u16,
	Z              = GLFW_KEY_Z as u16,
	LeftBracket    = GLFW_KEY_LEFT_BRACKET as u16,
	Backslash      = GLFW_KEY_BACKSLASH as u16,
	RightBracket   = GLFW_KEY_RIGHT_BRACKET as u16,
	Grave          = GLFW_KEY_GRAVE_ACCENT as u16,
	World1         = GLFW_KEY_WORLD_1 as u16,
	World2         = GLFW_KEY_WORLD_2 as u16,
	Escape         = GLFW_KEY_ESCAPE as u16,
	Enter          = GLFW_KEY_ENTER as u16,
	Tab            = GLFW_KEY_TAB as u16,
	Backspace      = GLFW_KEY_BACKSPACE as u16,
	Insert         = GLFW_KEY_INSERT as u16,
	Delete         = GLFW_KEY_DELETE as u16,
	Right          = GLFW_KEY_RIGHT as u16,
	Left           = GLFW_KEY_LEFT as u16,
	Down           = GLFW_KEY_DOWN as u16,
	Up             = GLFW_KEY_UP as u16,
	PageUp         = GLFW_KEY_PAGE_UP as u16,
	PageDown       = GLFW_KEY_PAGE_DOWN as u16,
	Home           = GLFW_KEY_HOME as u16,
	End            = GLFW_KEY_END as u16,
	CapsLock       = GLFW_KEY_CAPS_LOCK as u16,
	ScrollLock     = GLFW_KEY_SCROLL_LOCK as u16,
	NumLock        = GLFW_KEY_NUM_LOCK as u16,
	PrintScreen    = GLFW_KEY_PRINT_SCREEN as u16,
	Pause          = GLFW_KEY_PAUSE as u16,
	F1             = GLFW_KEY_F1 as u16,
	F2             = GLFW_KEY_F2 as u16,
	F3             = GLFW_KEY_F3 as u16,
	F4             = GLFW_KEY_F4 as u16,
	F5             = GLFW_KEY_F5 as u16,
	F6             = GLFW_KEY_F6 as u16,
	F7             = GLFW_KEY_F7 as u16,
	F8             = GLFW_KEY_F8 as u16,
	F9             = GLFW_KEY_F9 as u16,
	F10            = GLFW_KEY_F10 as u16,
	F11            = GLFW_KEY_F11 as u16,
	F12            = GLFW_KEY_F12 as u16,
	F13            = GLFW_KEY_F13 as u16,
	F14            = GLFW_KEY_F14 as u16,
	F15            = GLFW_KEY_F15 as u16,
	F16            = GLFW_KEY_F16 as u16,
	F17            = GLFW_KEY_F17 as u16,
	F18            = GLFW_KEY_F18 as u16,
	F19            = GLFW_KEY_F19 as u16,
	F20            = GLFW_KEY_F20 as u16,
	F21            = GLFW_KEY_F21 as u16,
	F22            = GLFW_KEY_F22 as u16,
	F23            = GLFW_KEY_F23 as u16,
	F24            = GLFW_KEY_F24 as u16,
	F25            = GLFW_KEY_F25 as u16,
	Keypad0        = GLFW_KEY_KP_0 as u16,
	Keypad1        = GLFW_KEY_KP_1 as u16,
	Keypad2        = GLFW_KEY_KP_2 as u16,
	Keypad3        = GLFW_KEY_KP_3 as u16,
	Keypad4        = GLFW_KEY_KP_4 as u16,
	Keypad5        = GLFW_KEY_KP_5 as u16,
	Keypad6        = GLFW_KEY_KP_6 as u16,
	Keypad7        = GLFW_KEY_KP_7 as u16,
	Keypad8        = GLFW_KEY_KP_8 as u16,
	Keypad9        = GLFW_KEY_KP_9 as u16,
	KeypadDecimal  = GLFW_KEY_KP_DECIMAL as u16,
	KeypadDivide   = GLFW_KEY_KP_DIVIDE as u16,
	KeypadMultiply = GLFW_KEY_KP_MULTIPLY as u16,
	KeypadSubtract = GLFW_KEY_KP_SUBTRACT as u16,
	KeypadAdd      = GLFW_KEY_KP_ADD as u16,
	KeypadEnter    = GLFW_KEY_KP_ENTER as u16,
	KeypadEqual    = GLFW_KEY_KP_EQUAL as u16,
	LeftShift      = GLFW_KEY_LEFT_SHIFT as u16,
	LeftControl    = GLFW_KEY_LEFT_CONTROL as u16,
	LeftAlt        = GLFW_KEY_LEFT_ALT as u16,
	LeftSuper      = GLFW_KEY_LEFT_SUPER as u16,
	RightShift     = GLFW_KEY_RIGHT_SHIFT as u16,
	RightControl   = GLFW_KEY_RIGHT_CONTROL as u16,
	RightAlt       = GLFW_KEY_RIGHT_ALT as u16,
	RightSuper     = GLFW_KEY_RIGHT_SUPER as u16,
	Menu           = GLFW_KEY_MENU as u16,
}
glfw_enum!(Key, u16);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Modifier(u8);

impl Modifier
{
	pub const ALT: Modifier = Modifier(GLFW_MOD_ALT as u8);
	pub const CAPS_LOCK: Modifier = Modifier(GLFW_MOD_CAPS_LOCK as u8);
	pub const CONTROL: Modifier = Modifier(GLFW_MOD_CONTROL as u8);
	pub const NUM_LOCK: Modifier = Modifier(GLFW_MOD_NUM_LOCK as u8);
	pub const SHIFT: Modifier = Modifier(GLFW_MOD_SHIFT as u8);
	pub const SUPER: Modifier = Modifier(GLFW_MOD_SUPER as u8);

	pub fn is_shift(&self) -> bool
	{
		self.0 & GLFW_MOD_SHIFT as u8 > 0
	}

	pub fn is_control(&self) -> bool
	{
		self.0 & GLFW_MOD_CONTROL as u8 > 0
	}

	pub fn is_alt(&self) -> bool
	{
		self.0 & GLFW_MOD_ALT as u8 > 0
	}

	pub fn is_super(&self) -> bool
	{
		self.0 & GLFW_MOD_SUPER as u8 > 0
	}

	pub fn is_caps_lock(&self) -> bool
	{
		self.0 & GLFW_MOD_CAPS_LOCK as u8 > 0
	}

	pub fn is_num_lock(&self) -> bool
	{
		self.0 & GLFW_MOD_NUM_LOCK as u8 > 0
	}
}

impl BitAnd<Modifier> for Modifier
{
	type Output = Modifier;

	fn bitand(self, rhs: Modifier) -> Modifier
	{
		Modifier(self.0 & rhs.0)
	}
}

impl BitAndAssign<Modifier> for Modifier
{
	fn bitand_assign(&mut self, rhs: Modifier)
	{
		self.0 &= rhs.0;
	}
}

impl BitOr<Modifier> for Modifier
{
	type Output = Modifier;

	fn bitor(self, rhs: Modifier) -> Modifier
	{
		Modifier(self.0 | rhs.0)
	}
}

impl BitOrAssign<Modifier> for Modifier
{
	fn bitor_assign(&mut self, rhs: Modifier)
	{
		self.0 |= rhs.0;
	}
}

impl BitXor<Modifier> for Modifier
{
	type Output = Modifier;

	fn bitxor(self, rhs: Modifier) -> Modifier
	{
		Modifier(self.0 ^ rhs.0)
	}
}

impl BitXorAssign<Modifier> for Modifier
{
	fn bitxor_assign(&mut self, rhs: Modifier)
	{
		self.0 ^= rhs.0;
	}
}
