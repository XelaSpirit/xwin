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
		GLFW_JOYSTICK_2,
		GLFW_JOYSTICK_3,
		GLFW_JOYSTICK_4,
		GLFW_JOYSTICK_5,
		GLFW_JOYSTICK_6,
		GLFW_JOYSTICK_7,
		GLFW_JOYSTICK_8,
		GLFW_JOYSTICK_9,
		GLFW_JOYSTICK_10,
		GLFW_JOYSTICK_11,
		GLFW_JOYSTICK_12,
		GLFW_JOYSTICK_13,
		GLFW_JOYSTICK_14,
		GLFW_JOYSTICK_15,
		GLFW_JOYSTICK_16,
	},
	glfw_enum,
};

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq)]
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

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GamepadButton
{
	A           = GLFW_GAMEPAD_BUTTON_A as u8,
	B           = GLFW_GAMEPAD_BUTTON_B as u8,
	X           = GLFW_GAMEPAD_BUTTON_X as u8,
	Y           = GLFW_GAMEPAD_BUTTON_Y as u8,
	LeftBumper  = GLFW_GAMEPAD_BUTTON_LEFT_BUMPER as u8,
	RightBumper = GLFW_GAMEPAD_BUTTON_RIGHT_BUMPER as u8,
	Back        = GLFW_GAMEPAD_BUTTON_BACK as u8,
	Start       = GLFW_GAMEPAD_BUTTON_START as u8,
	Guide       = GLFW_GAMEPAD_BUTTON_GUIDE as u8,
	LeftThumb   = GLFW_GAMEPAD_BUTTON_LEFT_THUMB as u8,
	RightThumb  = GLFW_GAMEPAD_BUTTON_RIGHT_THUMB as u8,
	Up          = GLFW_GAMEPAD_BUTTON_DPAD_UP as u8,
	Right       = GLFW_GAMEPAD_BUTTON_DPAD_RIGHT as u8,
	Down        = GLFW_GAMEPAD_BUTTON_DPAD_DOWN as u8,
	Left        = GLFW_GAMEPAD_BUTTON_DPAD_LEFT as u8,
}
glfw_enum!(GamepadButton, u8);

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq)]
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

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq)]
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
