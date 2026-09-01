use std::os::raw::c_int;

use xch::Sender;

use crate::{
	bind::{
		GLFW_CONNECTED,
		GLFW_DISCONNECTED,
		glfwSetJoystickCallback,
	},
	core::XWin,
	error::XErr,
	input::gamepad::Joystick,
};

/// Describes a change to a joystick's configuration.
#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JoystickConfigEvent
{
	Connected(Joystick) = GLFW_CONNECTED as u8,
	Disconnected(Joystick) = GLFW_DISCONNECTED as u8,
}

impl JoystickConfigEvent
{
	pub(crate) fn from_glfw(jid: u32, evt: u32) -> Self
	{
		if evt == GLFW_CONNECTED
		{
			JoystickConfigEvent::Connected(Joystick::from_glfw(jid))
		}
		else
		{
			JoystickConfigEvent::Disconnected(Joystick::from_glfw(jid))
		}
	}
}

/// Sets the [Sender] that will be used to send joystick configuration events. A
/// new event will be sent each time a [Joystick] is connected to or
/// disconnected from the system.
///
/// Note that joystick disconnection may also be detected and sent by joystick
/// functions. The function will then return whatever it returns if the joystick
/// is not present. This also means that if no disconnect message has been sent,
/// that is not sufficient to guarantee that a [Joystick] is present when a
/// joystick function is called.
///
/// # Errors
/// Possible errors include [XErr::NotInitialized].
pub fn set_joystick_channel<T>(tx: T) -> Result<(), XErr>
where
	T: Sender<JoystickConfigEvent> + Send + Sync + 'static,
{
	let mut xwin = XWin::get()?.write().unwrap();
	xwin.set_joystick_tx(tx);
	Ok(())
}

/// Closes the joystick configuration event channel.
///
/// See [set_joystick_channel].
///
/// # Errors
/// Possible errors include [XErr::NotInitialized].
pub fn clear_joystick_channel() -> Result<(), XErr>
{
	XWin::get()?.write().unwrap().remove_joystick_tx();
	Ok(())
}

pub(crate) fn set_joystick_callback()
{
	unsafe { glfwSetJoystickCallback(Some(joystick_handler)) };
}

extern "C" fn joystick_handler(jid: c_int, evt: c_int)
{
	if let Ok(lock) = XWin::get()
	{
		if let Ok(xwin) = lock.read()
		{
			if let Some(tx) = xwin.joystick_tx()
			{
				let _ = tx.send(JoystickConfigEvent::from_glfw(jid as u32, evt as u32));
			}
		}
	}
}
