use std::sync::mpsc::channel;

use crate::{
	bind::{
		GLFW_GAMEPAD_AXIS_LEFT_TRIGGER,
		GLFW_GAMEPAD_AXIS_LEFT_X,
		GLFW_GAMEPAD_AXIS_LEFT_Y,
		GLFW_GAMEPAD_AXIS_RIGHT_TRIGGER,
		GLFW_GAMEPAD_AXIS_RIGHT_X,
		GLFW_GAMEPAD_AXIS_RIGHT_Y,
		GLFW_GAMEPAD_BUTTON_A,
		GLFW_GAMEPAD_BUTTON_B,
		GLFW_GAMEPAD_BUTTON_BACK,
		GLFW_GAMEPAD_BUTTON_DPAD_DOWN,
		GLFW_GAMEPAD_BUTTON_DPAD_LEFT,
		GLFW_GAMEPAD_BUTTON_DPAD_RIGHT,
		GLFW_GAMEPAD_BUTTON_DPAD_UP,
		GLFW_GAMEPAD_BUTTON_GUIDE,
		GLFW_GAMEPAD_BUTTON_LEFT_BUMPER,
		GLFW_GAMEPAD_BUTTON_LEFT_THUMB,
		GLFW_GAMEPAD_BUTTON_RIGHT_BUMPER,
		GLFW_GAMEPAD_BUTTON_RIGHT_THUMB,
		GLFW_GAMEPAD_BUTTON_START,
		GLFW_GAMEPAD_BUTTON_X,
		GLFW_GAMEPAD_BUTTON_Y,
		GLFW_HAT_CENTERED,
		GLFW_HAT_DOWN,
		GLFW_HAT_LEFT,
		GLFW_HAT_RIGHT,
		GLFW_HAT_UP,
		GLFW_JOYSTICK_1,
		GLFW_JOYSTICK_10,
		GLFW_JOYSTICK_11,
		GLFW_JOYSTICK_12,
		GLFW_JOYSTICK_13,
		GLFW_JOYSTICK_14,
		GLFW_JOYSTICK_15,
		GLFW_JOYSTICK_16,
		GLFW_JOYSTICK_2,
		GLFW_JOYSTICK_3,
		GLFW_JOYSTICK_4,
		GLFW_JOYSTICK_5,
		GLFW_JOYSTICK_6,
		GLFW_JOYSTICK_7,
		GLFW_JOYSTICK_8,
		GLFW_JOYSTICK_9,
		GLFWgamepadstate,
	},
	core::{
		XWin,
		exec::XWinMessage,
	},
	error::XErr,
	glfw_enum,
	input::ButtonState,
};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GamepadAxis
{
	LeftX        = GLFW_GAMEPAD_AXIS_LEFT_X as u8,
	LeftY        = GLFW_GAMEPAD_AXIS_LEFT_Y as u8,
	RightX       = GLFW_GAMEPAD_AXIS_RIGHT_X as u8,
	RightY       = GLFW_GAMEPAD_AXIS_RIGHT_Y as u8,
	LeftTrigger  = GLFW_GAMEPAD_AXIS_LEFT_TRIGGER as u8,
	RightTrigger = GLFW_GAMEPAD_AXIS_RIGHT_TRIGGER as u8,
}
glfw_enum!(GamepadAxis, u8);

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GamepadButton
{
	A           = GLFW_GAMEPAD_BUTTON_A as u16,
	B           = GLFW_GAMEPAD_BUTTON_B as u16,
	X           = GLFW_GAMEPAD_BUTTON_X as u16,
	Y           = GLFW_GAMEPAD_BUTTON_Y as u16,
	LeftBumper  = GLFW_GAMEPAD_BUTTON_LEFT_BUMPER as u16,
	RightBumper = GLFW_GAMEPAD_BUTTON_RIGHT_BUMPER as u16,
	Back        = GLFW_GAMEPAD_BUTTON_BACK as u16,
	Start       = GLFW_GAMEPAD_BUTTON_START as u16,
	Guide       = GLFW_GAMEPAD_BUTTON_GUIDE as u16,
	LeftThumb   = GLFW_GAMEPAD_BUTTON_LEFT_THUMB as u16,
	RightThumb  = GLFW_GAMEPAD_BUTTON_RIGHT_THUMB as u16,
	Up          = GLFW_GAMEPAD_BUTTON_DPAD_UP as u16,
	Right       = GLFW_GAMEPAD_BUTTON_DPAD_RIGHT as u16,
	Down        = GLFW_GAMEPAD_BUTTON_DPAD_DOWN as u16,
	Left        = GLFW_GAMEPAD_BUTTON_DPAD_LEFT as u16,
}
glfw_enum!(GamepadButton, u16);

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JoystickHatState
{
	Centered  = GLFW_HAT_CENTERED as u8,
	Up        = GLFW_HAT_UP as u8,
	Right     = GLFW_HAT_RIGHT as u8,
	Down      = GLFW_HAT_DOWN as u8,
	Left      = GLFW_HAT_LEFT as u8,
	UpRight   = JoystickHatState::Up as u8 | JoystickHatState::Right as u8,
	DownRight = JoystickHatState::Down as u8 | JoystickHatState::Right as u8,
	DownLeft  = JoystickHatState::Down as u8 | JoystickHatState::Left as u8,
	UpLeft    = JoystickHatState::Up as u8 | JoystickHatState::Left as u8,
}
glfw_enum!(JoystickHatState, u8);

/// Contains the input state of a gamepad.
pub struct GamepadState
{
	buttons: [ButtonState; 15],
	axes:    [f32; 6],
}

impl GamepadState
{
	/// Returns the [ButtonState] of a given [GamepadButton].
	pub fn button(&self, button: GamepadButton) -> ButtonState
	{
		self.buttons[button.as_glfw() as usize]
	}

	/// Returns the state of a given [GamepadAxis], in the range -1.0 to 1.0
	/// inclusive.
	pub fn axis(&self, axis: GamepadAxis) -> f32
	{
		self.axes[axis.as_glfw() as usize]
	}

	#[cfg(feature = "glfw")]
	pub unsafe fn from_glfw(state: GLFWgamepadstate) -> GamepadState
	{
		Self::from_glfw_crate(state)
	}
	
	#[cfg(feature = "glfw")]
	pub fn as_glfw(&self) -> GLFWgamepadstate
	{
		GLFWgamepadstate
		{
			buttons: [
				self.buttons[GLFW_GAMEPAD_BUTTON_A as usize].as_glfw(),
				self.buttons[GLFW_GAMEPAD_BUTTON_B as usize].as_glfw(),
				self.buttons[GLFW_GAMEPAD_BUTTON_X as usize].as_glfw(),
				self.buttons[GLFW_GAMEPAD_BUTTON_Y as usize].as_glfw(),
				self.buttons[GLFW_GAMEPAD_BUTTON_LEFT_BUMPER as usize].as_glfw(),
				self.buttons[GLFW_GAMEPAD_BUTTON_RIGHT_BUMPER as usize].as_glfw(),
				self.buttons[GLFW_GAMEPAD_BUTTON_BACK as usize].as_glfw(),
				self.buttons[GLFW_GAMEPAD_BUTTON_START as usize].as_glfw(),
				self.buttons[GLFW_GAMEPAD_BUTTON_GUIDE as usize].as_glfw(),
				self.buttons[GLFW_GAMEPAD_BUTTON_LEFT_THUMB as usize].as_glfw(),
				self.buttons[GLFW_GAMEPAD_BUTTON_RIGHT_THUMB as usize].as_glfw(),
				self.buttons[GLFW_GAMEPAD_BUTTON_DPAD_UP as usize].as_glfw(),
				self.buttons[GLFW_GAMEPAD_BUTTON_DPAD_RIGHT as usize].as_glfw(),
				self.buttons[GLFW_GAMEPAD_BUTTON_DPAD_DOWN as usize].as_glfw(),
				self.buttons[GLFW_GAMEPAD_BUTTON_DPAD_LEFT as usize].as_glfw(),
			],
			axes: [
				self.axes[GLFW_GAMEPAD_AXIS_LEFT_X as usize],
				self.axes[GLFW_GAMEPAD_AXIS_LEFT_Y as usize],
				self.axes[GLFW_GAMEPAD_AXIS_RIGHT_X as usize],
				self.axes[GLFW_GAMEPAD_AXIS_RIGHT_Y as usize],
				self.axes[GLFW_GAMEPAD_AXIS_LEFT_TRIGGER as usize],
				self.axes[GLFW_GAMEPAD_AXIS_RIGHT_TRIGGER as usize],
			]
		}
	}

	pub(crate) fn from_glfw_crate(state: GLFWgamepadstate) -> GamepadState
	{
		unsafe {
			GamepadState {
				buttons: [
					ButtonState::from_glfw(state.buttons[GLFW_GAMEPAD_BUTTON_A as usize] as u32),
					ButtonState::from_glfw(state.buttons[GLFW_GAMEPAD_BUTTON_B as usize] as u32),
					ButtonState::from_glfw(state.buttons[GLFW_GAMEPAD_BUTTON_X as usize] as u32),
					ButtonState::from_glfw(state.buttons[GLFW_GAMEPAD_BUTTON_Y as usize] as u32),
					ButtonState::from_glfw(
						state.buttons[GLFW_GAMEPAD_BUTTON_LEFT_BUMPER as usize] as u32,
					),
					ButtonState::from_glfw(
						state.buttons[GLFW_GAMEPAD_BUTTON_RIGHT_BUMPER as usize] as u32,
					),
					ButtonState::from_glfw(state.buttons[GLFW_GAMEPAD_BUTTON_BACK as usize] as u32),
					ButtonState::from_glfw(
						state.buttons[GLFW_GAMEPAD_BUTTON_START as usize] as u32,
					),
					ButtonState::from_glfw(
						state.buttons[GLFW_GAMEPAD_BUTTON_GUIDE as usize] as u32,
					),
					ButtonState::from_glfw(
						state.buttons[GLFW_GAMEPAD_BUTTON_LEFT_THUMB as usize] as u32,
					),
					ButtonState::from_glfw(
						state.buttons[GLFW_GAMEPAD_BUTTON_RIGHT_THUMB as usize] as u32,
					),
					ButtonState::from_glfw(
						state.buttons[GLFW_GAMEPAD_BUTTON_DPAD_UP as usize] as u32,
					),
					ButtonState::from_glfw(
						state.buttons[GLFW_GAMEPAD_BUTTON_DPAD_RIGHT as usize] as u32,
					),
					ButtonState::from_glfw(
						state.buttons[GLFW_GAMEPAD_BUTTON_DPAD_DOWN as usize] as u32,
					),
					ButtonState::from_glfw(
						state.buttons[GLFW_GAMEPAD_BUTTON_DPAD_LEFT as usize] as u32,
					),
				],
				axes:    [
					state.axes[GLFW_GAMEPAD_AXIS_LEFT_X as usize],
					state.axes[GLFW_GAMEPAD_AXIS_LEFT_Y as usize],
					state.axes[GLFW_GAMEPAD_AXIS_RIGHT_X as usize],
					state.axes[GLFW_GAMEPAD_AXIS_RIGHT_Y as usize],
					state.axes[GLFW_GAMEPAD_AXIS_LEFT_TRIGGER as usize],
					state.axes[GLFW_GAMEPAD_AXIS_RIGHT_TRIGGER as usize],
				],
			}
		}
	}
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Joystick
{
	One      = GLFW_JOYSTICK_1 as u8,
	Two      = GLFW_JOYSTICK_2 as u8,
	Three    = GLFW_JOYSTICK_3 as u8,
	Four     = GLFW_JOYSTICK_4 as u8,
	Five     = GLFW_JOYSTICK_5 as u8,
	Six      = GLFW_JOYSTICK_6 as u8,
	Seven    = GLFW_JOYSTICK_7 as u8,
	Eight    = GLFW_JOYSTICK_8 as u8,
	Nine     = GLFW_JOYSTICK_9 as u8,
	Ten      = GLFW_JOYSTICK_10 as u8,
	Eleven   = GLFW_JOYSTICK_11 as u8,
	Twelve   = GLFW_JOYSTICK_12 as u8,
	Thirteen = GLFW_JOYSTICK_13 as u8,
	Fourteen = GLFW_JOYSTICK_14 as u8,
	Fifteen  = GLFW_JOYSTICK_15 as u8,
	Sixteen  = GLFW_JOYSTICK_16 as u8,
}
glfw_enum!(Joystick, u8);

impl Joystick
{
	// =======================
	//     QUERY FUNCTIONS
	// =======================

	/// See [Joystick::try_axes].
	pub fn axes(&self) -> Option<Vec<f32>>
	{
		self.try_axes().unwrap_or_default()
	}

	/// See [Joystick::try_buttons].
	pub fn buttons(&self) -> Option<Vec<ButtonState>>
	{
		self.try_buttons().unwrap_or_default()
	}

	/// See [Joystick::try_gamepad_name].
	pub fn gamepad_name(&self) -> Option<String>
	{
		self.try_gamepad_name().unwrap_or_default()
	}

	/// See [Joystick::try_guid].
	pub fn guid(&self) -> Option<String>
	{
		self.try_guid().unwrap_or_default()
	}

	/// See [Joystick::try_hats].
	pub fn hats(&self) -> Option<Vec<JoystickHatState>>
	{
		self.try_hats().unwrap_or_default()
	}

	/// See [Joystick::try_is_gamepad].
	pub fn is_gamepad(&self) -> bool
	{
		self.try_is_gamepad().unwrap_or_default()
	}

	/// See [Joystick::try_is_present].
	pub fn is_present(&self) -> bool
	{
		self.try_is_present().unwrap_or_default()
	}

	/// See [Joystick::try_name].
	pub fn name(&self) -> Option<String>
	{
		self.try_name().unwrap_or_default()
	}

	/// See [Joystick::try_state].
	pub fn state(&self) -> Option<GamepadState>
	{
		self.try_state().unwrap_or_default()
	}

	// =======================
	//   TRY QUERY FUNCTIONS
	// =======================

	/// Returns the values of all axes of the joystick. Each element in the
	/// [Vec] is a value between -1.0 and 1.0.
	///
	/// If the joystick is not present, `None` is returned without generating an
	/// error. This can be used instead of first calling
	/// [Joystick::try_is_present].
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized] and [XErr::Platform].
	pub fn try_axes(&self) -> Result<Option<Vec<f32>>, XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?.read().unwrap().post_rcv(
			XWinMessage::JoystickAxes(unsafe { self.as_glfw() } as i32, tx),
			rx,
		)?
	}

	/// Returns the state of all buttons of the joystick.
	///
	/// If the joystick is not present, `None` is returned without generating an
	/// error. This can be used instead of first calling
	/// [Joystick::try_is_present].
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized] and [XErr::Platform].
	pub fn try_buttons(&self) -> Result<Option<Vec<ButtonState>>, XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?.read().unwrap().post_rcv(
			XWinMessage::JoystickButtons(unsafe { self.as_glfw() } as i32, tx),
			rx,
		)?
	}

	/// Returns the human-readable name of the gamepad from the gamepad mapping
	/// assigned to the joystick.
	///
	/// If the joystick is not present or does not have a gamepad mapping this
	/// function will return `None` without generating an error. Call
	/// [Joystick::try_is_present] to check whether it is present regardless of
	/// whether it has a mapping.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized].
	pub fn try_gamepad_name(&self) -> Result<Option<String>, XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?.read().unwrap().post_rcv(
			XWinMessage::GetGamepadName(unsafe { self.as_glfw() } as i32, tx),
			rx,
		)?
	}

	/// Returns the SDL compatible GUID, as a hexadecimal string, of the
	/// joystick.
	///
	/// If the joystick is not present, `None` is returned without generating an
	/// error. This can be used instead of first calling
	/// [Joystick::try_is_present].
	///
	/// The GUID is what connects a joystick to a gamepad mapping. A connected
	/// joystick will always have a GUID even if there is no gamepad mapping
	/// assigned to it.
	///
	/// The GUID uses the format introduced in SDL 2.0.5. This GUID tries to
	/// uniquely identify the make and model of a joystick but does not identify
	/// a specific unit, e.g. all wired Xbox 360 controllers will have the same
	/// GUID on that platform. The GUID for a unit may vary between platforms
	/// depending on what hardware information the platform specific APIs
	/// provide.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized] and [XErr::Platform].
	pub fn try_guid(&self) -> Result<Option<String>, XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?.read().unwrap().post_rcv(
			XWinMessage::JoystickGuid(unsafe { self.as_glfw() } as i32, tx),
			rx,
		)?
	}

	/// Returns the state of all hats of the joystick.
	///
	/// If the joystick is not present, `None` is returned without generating an
	/// error. This can be used instead of first calling
	/// [Joystick::try_is_present].
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized] and [XErr::Platform].
	pub fn try_hats(&self) -> Result<Option<Vec<JoystickHatState>>, XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?.read().unwrap().post_rcv(
			XWinMessage::JoystickHats(unsafe { self.as_glfw() } as i32, tx),
			rx,
		)?
	}

	/// Returns whether the joystick is both present and has a gamepad mapping.
	///
	/// If the joystick is present but does not have a gamepad mapping this
	/// function will return `false` without generating an error. Call
	/// [Joystick::is_present] to check if a joystick is present regardless of
	/// whether it has a mapping.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized].
	pub fn try_is_gamepad(&self) -> Result<bool, XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?.read().unwrap().post_rcv(
			XWinMessage::JoystickIsGamepad(unsafe { self.as_glfw() } as i32, tx),
			rx,
		)?
	}

	/// Returns whether a given joystick is present.
	///
	/// There is no need to call this function before other functions that
	/// accept a [Joystick] argument, as they all check for presence before
	/// performing any other work.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized] and [XErr::Platform].
	pub fn try_is_present(&self) -> Result<bool, XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?.read().unwrap().post_rcv(
			XWinMessage::JoystickPresent(unsafe { self.as_glfw() } as i32, tx),
			rx,
		)?
	}

	/// Returns the name of the joystick.
	///
	/// If the joystick is not present, `None` is returned without generating an
	/// error. This can be used instead of first calling
	/// [Joystick::try_is_present].
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized] and [XErr::Platform].
	pub fn try_name(&self) -> Result<Option<String>, XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?.read().unwrap().post_rcv(
			XWinMessage::JoystickName(unsafe { self.as_glfw() } as i32, tx),
			rx,
		)?
	}

	/// Returns the state of the joystick remapped to an Xbox-like gamepad.
	///
	/// If the joystick is not present or does not have a gamepad mapping this
	/// function will return `None` without generating an error. Call
	/// [Joystick::try_is_present] to check whether it is present regardless of
	/// whether it has a mapping.
	///
	/// The `Guide` button may not be available for input as it is often hooked
	/// by the system or the Steam client.
	///
	/// Not all devices have all the buttons or axes provided by [GamepadState].
	/// Unavailable buttons and axes will always have values of
	/// [ButtonState::Release] and `0.0` respectively.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized].
	pub fn try_state(&self) -> Result<Option<GamepadState>, XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?
			.read()
			.unwrap()
			.post_rcv(
				XWinMessage::GetGamepadState(unsafe { self.as_glfw() } as i32, tx),
				rx,
			)?
			.map(|opt| opt.map(|state| GamepadState::from_glfw_crate(state)))
	}
}

pub fn update_gamepad_mappings(mappings: String)
{
	let _ = try_update_gamepad_mappings(mappings);
}

// TODO - Add documentation for mapping format
/// Parses the specified string and updates the internal list with any gamepad
/// mappings it finds. The string may contain either a single gamepad mapping or
/// many mappings separated by newlines. The parser supports the full format of
/// the `gamecontrollerdb.txt` source file including empty lines and comments.
///
/// See the [GLFW documentation on Gamepad Mappings](https://www.glfw.org/docs/latest/input_guide.html#gamepad_mapping) for a more complete description of the format.
///
/// If there is already a gamepad mapping for a given GUID in the internal list,
/// it will be replaced by the one passed to this function.
///
/// # Errors
/// Possible errors include [XErr::NotInitialized].
pub fn try_update_gamepad_mappings(mappings: String) -> Result<(), XErr>
{
	let (tx, rx) = channel();
	XWin::get()?
		.read()
		.unwrap()
		.post_rcv(XWinMessage::UpdateGamepadMappings(mappings, tx), rx)?
}
