use crate::{
	bind::{
		GLFW_PRESS,
		GLFW_RELEASE,
		GLFW_REPEAT,
	},
	glfw_enum,
};

pub(crate) mod event;
pub mod gamepad;
pub mod keyboard;
pub mod mouse;

#[repr(u8)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonState
{
	#[default]
	Release = GLFW_RELEASE as u8,
	Press   = GLFW_PRESS as u8,
}
glfw_enum!(ButtonState, u8);

#[repr(u8)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonEvent
{
	#[default]
	Release = GLFW_RELEASE as u8,
	Press   = GLFW_PRESS as u8,
	Repeat  = GLFW_REPEAT as u8,
}
glfw_enum!(ButtonEvent, u8);
