use std::{
	ffi::CStr,
	os::raw::c_int,
};

use xch::Sender;

use crate::{
	bind::{
		GLFW_CONNECTED,
		GLFWmonitor,
		glfwGetMonitorName,
		glfwSetMonitorCallback,
	},
	core::XWin,
	error::XErr,
	monitor::Monitor,
};

/// Describes a change to a monitor's configuration
///
/// If a monitor is disconnected, all windows that are full screen on it will be
/// switched to windowed mode before the callback is called.
#[derive(Debug)]
pub enum MonitorEvent
{
	/// Contains a [Monitor] which has been connected.
	Connected(Monitor),
	/// Contains the name of a [Monitor] which has been disconnected. See
	/// [Monitor::try_name].
	Disconnected(String),
}

impl MonitorEvent
{
	fn from_glfw(mon: *mut GLFWmonitor, evt: u32) -> MonitorEvent
	{
		let monitor = Monitor::from_glfw(mon);
		if evt == GLFW_CONNECTED
		{
			MonitorEvent::Connected(monitor)
		}
		else
		{
			let title = unsafe { glfwGetMonitorName(monitor.as_glfw()) };

			MonitorEvent::Disconnected(
				if title.is_null()
				{
					String::new()
				}
				else
				{
					unsafe { CStr::from_ptr(title) }
						.to_str()
						.unwrap_or_else(|_| "")
						.to_owned()
				},
			)
		}
	}
}

impl Clone for MonitorEvent
{
	fn clone(&self) -> Self
	{
		match self
		{
			| MonitorEvent::Connected(monitor) =>
			{
				MonitorEvent::Connected(Monitor(monitor.as_glfw()))
			},
			| MonitorEvent::Disconnected(name) => MonitorEvent::Disconnected(name.clone()),
		}
	}
}

/// Sets the [Sender] that will be used to send monitor configuration events. A
/// new event will be pushed to the channel each time a [Monitor] is connected
/// to or disconnected from the system.
///
/// # Errors
/// Possible errors include [XErr::NotInitialized].
pub fn set_monitor_channel<T>(tx: T) -> Result<(), XErr>
where
	T: Sender<MonitorEvent> + Send + Sync + 'static,
{
	let mut xwin = XWin::get()?.write().unwrap();
	xwin.set_monitor_tx(tx);
	Ok(())
}

/// Closes the monitor event channel.
///
/// See [set_monitor_channel].
///
/// # Errors
/// Possible errors include [XErr::NotInitialized].
pub fn clear_monitor_channel() -> Result<(), XErr>
{
	XWin::get()?.write().unwrap().remove_monitor_tx();
	Ok(())
}

pub(crate) fn set_monitor_callback()
{
	unsafe { glfwSetMonitorCallback(Some(glfw_monitor_handler)) };
}

extern "C" fn glfw_monitor_handler(mon: *mut GLFWmonitor, evt: c_int)
{
	if let Ok(lock) = XWin::get()
	{
		if let Ok(xwin) = lock.read()
		{
			if let Some(tx) = xwin.monitor_tx()
			{
				let _ = tx.send(MonitorEvent::from_glfw(mon, evt as u32));
			}
		}
	}
}
