use std::{
	ffi::CStr,
	os::raw::c_int,
};

use xch::Sender;

use crate::{
	bind::{
		GLFW_CONNECTED,
		GLFW_DISCONNECTED,
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

/// Subscribes to monitor configuration events on the given channel. A new event
/// will be pushed to the channel each time a [Monitor] is connected or
/// disconnected.
///
/// See [crate::core#event-handling]
///
/// # Errors
/// Possible errors include [XErr::NotInitialized].
pub fn subscribe_monitors<T>(tx: T) -> Result<(), XErr>
where
	T: Sender<MonitorEvent> + Send + Sync + 'static,
{
	let mut xwin = XWin::get()?.write().unwrap();
	xwin.set_monitor_tx(tx);
	unsafe { glfwSetMonitorCallback(Some(glfw_monitor_handler)) };
	Ok(())
}

/// Closes the monitor event channel, disconnecting the event sender (if one
/// exists).
///
/// See also [subscribe_monitors].
pub fn unsubscribe_monitors() -> Result<(), XErr>
{
	unsafe { glfwSetMonitorCallback(None) };
	XWin::get()?.write().unwrap().remove_monitor_tx();
	Ok(())
}

extern "C" fn glfw_monitor_handler(mon: *mut GLFWmonitor, ev: c_int)
{
	let monitor = Monitor::from_glfw(mon);
	let event = match ev as u32
	{
		| GLFW_CONNECTED => MonitorEvent::Connected(monitor),
		| GLFW_DISCONNECTED =>
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
		},
		| _ => return,
	};

	if let Ok(lock) = XWin::get()
	{
		if let Ok(xwin) = lock.read()
		{
			if let Some(tx) = xwin.monitor_tx()
			{
				if let Err(_) = tx.send(event)
				{
					drop(xwin);

					// The only case where we wouldn't be able to acquire write access would be if a
					// new sender is being written already, in which case we don't want to override
					// that with None. This should be the only function calling read, so reads
					// wouldn't be the thing preventing this lock.
					if let Ok(mut xwin) = lock.try_write()
					{
						xwin.remove_monitor_tx();
					}
				}
			}
		}
	}
}
