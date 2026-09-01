use std::os::raw::c_int;
use std::sync::{
	Arc,
	LazyLock,
	RwLock,
};
use crate::bind::{glfwSetMonitorCallback, GLFWmonitor, GLFW_CONNECTED, GLFW_DISCONNECTED};
use crate::monitor::{
	Monitor,
};

/// Alias for a monitor callback function.
pub type MonitorCallback = fn(&Monitor, MonitorEvent);

static MONITOR_CALLBACKS: LazyLock<RwLock<Vec<Arc<MonitorCallback>>>> =
	LazyLock::new(RwLock::default);

/// Describes a change to a monitor's configuration
///
/// If a monitor is disconnected, all windows that are full screen on it will be
/// switched to windowed mode before the callback is called.
///
/// Only [Monitor::name] and [Monitor::userdata] will return useful values for a
/// disconnected monitor and only before the monitor callback returns.
#[derive(Copy, Clone, Debug)]
pub enum MonitorEvent
{
	Connected,
	Disconnected,
}

/// Adds a monitor configuration callback. This is called when a monitor is
/// connected to or disconnected from the system.
///
/// # Returns
/// An [Arc] referring to the callback, which may be used to later remove the
/// callback using [remove_monitor_callback].
///
/// # Thread Safety
/// This function may be called from any thread.
///
/// # See Also
/// - [MonitorCallback]
pub fn add_monitor_callback(f: MonitorCallback) -> Arc<MonitorCallback>
{
	let arc = Arc::new(f);
	if let Ok(mut vec) = MONITOR_CALLBACKS.write()
	{
		vec.push(arc.clone());
	}
	arc
}

/// Removed a monitor configuration callback, such that it will no longer be
/// called when a monitor is connected to or disconnected from the system.
///
/// # Thread Safety
/// This function may be called from any thread.
///
/// # See Also
/// - [MonitorCallback]
pub fn remove_monitor_callback(f: Arc<MonitorCallback>)
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