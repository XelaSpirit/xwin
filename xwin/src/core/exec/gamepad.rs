use std::{
	ffi::CString,
	os::raw::c_int,
	sync::mpsc::Sender,
};

use crate::{
	bind::{
		GLFW_TRUE,
		GLFWgamepadstate,
		glfwGetGamepadName,
		glfwGetGamepadState,
		glfwGetJoystickAxes,
		glfwGetJoystickButtons,
		glfwGetJoystickGUID,
		glfwGetJoystickHats,
		glfwGetJoystickName,
		glfwJoystickIsGamepad,
		glfwJoystickPresent,
		glfwUpdateGamepadMappings,
	},
	core::exec::send_string,
	error::XErr,
	input::{
		ButtonState,
		gamepad::JoystickHatState,
	},
};

pub(super) fn joystick_present(jid: i32, tx: Sender<Result<bool, XErr>>)
{
	let present = unsafe { glfwJoystickPresent(jid) };
	let _ = tx.send(XErr::result(|| {
		if present == GLFW_TRUE as i32
		{
			true
		}
		else
		{
			false
		}
	}));
}

pub(super) fn joystick_axes(jid: i32, tx: Sender<Result<Option<Vec<f32>>, XErr>>)
{
	let mut count: c_int = 0;
	let axes = unsafe { glfwGetJoystickAxes(jid, &mut count) };
	let _ = tx.send(XErr::result(|| {
		if count == 0 || axes.is_null()
		{
			None
		}
		else
		{
			Some(unsafe { std::slice::from_raw_parts(axes, count as usize) }.to_vec())
		}
	}));
}

pub(super) fn joystick_buttons(jid: i32, tx: Sender<Result<Option<Vec<ButtonState>>, XErr>>)
{
	let mut count: c_int = 0;
	let buttons = unsafe { glfwGetJoystickButtons(jid, &mut count) };
	let _ = tx.send(XErr::result(|| {
		if count == 0 || buttons.is_null()
		{
			None
		}
		else
		{
			Some(
				unsafe { std::slice::from_raw_parts(buttons, count as usize) }
					.iter()
					.map(|value| unsafe { ButtonState::from_glfw(*value as u32) })
					.collect(),
			)
		}
	}));
}

pub(super) fn joystick_hats(jid: i32, tx: Sender<Result<Option<Vec<JoystickHatState>>, XErr>>)
{
	let mut count: c_int = 0;
	let hats = unsafe { glfwGetJoystickHats(jid, &mut count) };
	let _ = tx.send(XErr::result(|| {
		if count == 0 || hats.is_null()
		{
			None
		}
		else
		{
			Some(
				unsafe { std::slice::from_raw_parts(hats, count as usize) }
					.iter()
					.map(|value| unsafe { JoystickHatState::from_glfw(*value as u32) })
					.collect(),
			)
		}
	}));
}

pub(super) fn joystick_name(jid: i32, tx: Sender<Result<Option<String>, XErr>>)
{
	send_string(unsafe { glfwGetJoystickName(jid) }, tx);
}

pub(super) fn joystick_guid(jid: i32, tx: Sender<Result<Option<String>, XErr>>)
{
	send_string(unsafe { glfwGetJoystickGUID(jid) }, tx);
}

pub(super) fn joystick_is_gamepad(jid: i32, tx: Sender<Result<bool, XErr>>)
{
	let value = unsafe { glfwJoystickIsGamepad(jid) };
	let _ = tx.send(XErr::result(|| {
		if value == GLFW_TRUE as i32
		{
			true
		}
		else
		{
			false
		}
	}));
}

pub(super) fn update_gamepad_mappings(mappings: String, tx: Sender<Result<(), XErr>>)
{
	if let Ok(str) = CString::new(mappings)
	{
		unsafe { glfwUpdateGamepadMappings(str.as_ptr()) };
		let _ = tx.send(XErr::result(|| ()));
	}
	else
	{
		let _ = tx.send(Ok(()));
	}
}

pub(super) fn gamepad_name(jid: i32, tx: Sender<Result<Option<String>, XErr>>)
{
	send_string(unsafe { glfwGetGamepadName(jid) }, tx);
}

pub(super) fn gamepad_state(jid: i32, tx: Sender<Result<Option<GLFWgamepadstate>, XErr>>)
{
	let mut state = GLFWgamepadstate {
		buttons: [0; 15usize],
		axes:    [0.0; 6usize],
	};
	let res = unsafe { glfwGetGamepadState(jid, &mut state) };

	let _ = tx.send(XErr::result(|| {
		if res == GLFW_TRUE as i32
		{
			Some(state)
		}
		else
		{
			None
		}
	}));
}
