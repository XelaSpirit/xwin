use std::{
	ffi::CStr,
	os::raw::c_int,
	sync::{
		OnceLock,
		mpsc::{
			Receiver,
			Sender,
			channel,
		},
	},
};

use crate::{
	bind::{
		GLFW_CONNECTED,
		GLFW_DISCONNECTED,
		GLFWmonitor,
		glfwGetMonitorName,
		glfwSetMonitorCallback,
	},
	monitor::Monitor,
};

static MONITOR_CALLBACKS: OnceLock<Sender<MonitorEvent>> = OnceLock::new();

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

/// Creates a new asynchronous channel on which monitor configuration events
/// will be sent. A new event will be pushed to the channel each time a
/// [Monitor] is connected or disconnected.
///
/// Note that only one such channel may be created. If the caller disconnects
/// the returned [Receiver], there's no way to reopen this channel. It is
/// expected the caller will hold the [Receiver] for as long as they need to.
///
/// If a custom channel implementation is desired, see the [set_monitor_tx]
/// function.
///
/// # Errors
/// Will return an [Err] if a channel for monitor configuration events has
/// already been created, either with this function or [set_monitor_tx].
///
/// # Returns
/// A [Receiver] from which monitor configuration events can be retrieved.
pub fn monitor_event_rx() -> Result<Receiver<MonitorEvent>, ()>
{
	let (tx, rx) = channel();
	match MONITOR_CALLBACKS.set(tx)
	{
		| Ok(_) => Ok(rx),
		| Err(_) => Err(()),
	}
}

/// Set a sender on which monitor configuration events will be sent. A new event
/// will be sent each time a [Monitor] is connected or disconnected.
///
/// Not that only one monitor configuration event channel may exist. If the
/// caller disconnects the associated [Receiver], there's no way to set a new
/// [Sender]. It is expected the caller will hold the [Receiver] for as long as
/// they need to.
///
/// # Errors
/// Will return an [Err] if a sender for monitor configuration events has
/// already been set, either with this function or [monitor_event_rx].
pub fn set_monitor_tx(tx: Sender<MonitorEvent>) -> Result<(), ()>
{
	MONITOR_CALLBACKS.set(tx).map_err(|_| ())
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

	if let Some(tx) = MONITOR_CALLBACKS.get()
	{
		let _ = tx.send(event);
	}
}

pub(crate) fn set_monitor_callback()
{
	unsafe { glfwSetMonitorCallback(Some(glfw_monitor_handler)) };
}
