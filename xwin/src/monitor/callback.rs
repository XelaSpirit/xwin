use std::{
	os::raw::c_int,
	sync::{
		Arc,
		LazyLock,
		RwLock,
	},
};

use crate::{
	bind::{
		GLFW_CONNECTED,
		GLFW_DISCONNECTED,
		GLFWmonitor,
		glfwSetMonitorCallback,
	},
	monitor::Monitor,
};

/// Alias for a monitor window function.
pub type MonitorFn = fn(&Monitor, MonitorEvent);

static MONITOR_CALLBACKS: LazyLock<RwLock<Vec<Arc<MonitorFn>>>> = LazyLock::new(RwLock::default);

/// Describes a change to a monitor's configuration
///
/// If a monitor is disconnected, all windows that are full screen on it will be
/// switched to windowed mode before the window is called.
///
/// Only [Monitor::name] and [Monitor::userdata] will return useful values for a
/// disconnected monitor and only before the monitor window returns.
#[derive(Copy, Clone, Debug)]
pub enum MonitorEvent
{
	Connected,
	Disconnected,
}

/// Adds a monitor configuration window. This is called when a monitor is
/// connected to or disconnected from the system.
///
/// # Returns
/// An [Arc] referring to the window, which may be used to later remove the
/// window using [remove_monitor_callback].
///
/// # See Also
/// - [MonitorFn]
pub fn add_monitor_callback(f: MonitorFn) -> Arc<MonitorFn>
{
	let arc = Arc::new(f);
	if let Ok(mut vec) = MONITOR_CALLBACKS.write()
	{
		vec.push(arc.clone());
	}
	arc
}

/// Removed a monitor configuration window, such that it will no longer be
/// called when a monitor is connected to or disconnected from the system.
///
/// # See Also
/// - [MonitorFn]
pub fn remove_monitor_callback(f: Arc<MonitorFn>)
{
	if let Ok(mut vec) = MONITOR_CALLBACKS.write()
	{
		(*vec).retain(|cb| !Arc::ptr_eq(&f, cb));
	}
}

extern "C" fn glfw_monitor_handler(mon: *mut GLFWmonitor, ev: c_int)
{
	let monitor = Monitor::from_glfw(mon);
	let event = match ev as u32
	{
		| GLFW_CONNECTED => MonitorEvent::Connected,
		| GLFW_DISCONNECTED => MonitorEvent::Disconnected,
		| _ => return,
	};

	if let Ok(vec) = MONITOR_CALLBACKS.read()
	{
		for cb in &*vec
		{
			cb(&monitor, event);
		}
	}
}

pub(crate) fn set_monitor_callback()
{
	unsafe { glfwSetMonitorCallback(Some(glfw_monitor_handler)) };
}
