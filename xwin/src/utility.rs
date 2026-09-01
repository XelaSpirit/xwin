use std::sync::mpsc::channel;

use crate::{
	bind::{
		GLFW_TRUE,
		glfwGetTime,
		glfwGetTimerFrequency,
		glfwGetTimerValue,
		glfwSetTime,
		glfwVulkanSupported,
	},
	core::{
		XWin,
		exec::XWinMessage,
	},
	error::XErr,
};

/// See [try_clipboard_string].
pub fn clipboard_string() -> String
{
	try_clipboard_string().unwrap_or_default()
}

/// See [try_set_clipboard_string].
pub fn set_clipboard_string(value: String)
{
	let _ = try_set_clipboard_string(value);
}

/// See [try_set_time].
pub fn set_time(value: f64)
{
	let _ = try_set_time(value);
}

/// See [try_time].
pub fn time() -> f64
{
	try_time().unwrap_or_default()
}

/// See [try_timer_frequency].
pub fn timer_frequency() -> u64
{
	try_timer_frequency().unwrap_or_default()
}

/// See [try_timer_value].
pub fn timer_value() -> u64
{
	try_timer_value().unwrap_or_default()
}

/// Returns the contents of the system clipboard, if it contains or is
/// convertible to a UTF-8 encoded string. If the clipboard is empty or if its
/// contents cannot be converted, [XErr::FormatUnavailable] is returned.
///
/// # Errors
/// Possible errors include [XErr::NotInitialized], [XErr::FormatUnavailable],
/// and [XErr::Platform].
///
/// # Remarks
/// **Win32**: The clipboard on Windows has a single global lock for reading and
/// writing. XWin tries to acquire it a few times, which is almost always
/// enough. If it cannot acquire the lock then this function returns
/// [XErr::Platform]. It is safe to try this multiple times.
pub fn try_clipboard_string() -> Result<String, XErr>
{
	let (tx, rx) = channel();
	XWin::get()?
		.read()
		.unwrap()
		.post_rcv(XWinMessage::GetClipboardString(tx), rx)?
}

/// Sets the system clipboard to the specified [String].
///
/// # Errors
/// Possible errors include [XErr::NotInitialized].
///
/// # Remarks
/// **Win32**: The clipboard on Windows has a single global lock for reading and
/// writing. XWin tries to acquire it a few times, which is almost always
/// enough. If it cannot acquire the lock then this function returns
/// [XErr::Platform]. It is safe to try this multiple times.
pub fn try_set_clipboard_string(value: String) -> Result<(), XErr>
{
	let (tx, rx) = channel();
	XWin::get()?
		.read()
		.unwrap()
		.post_rcv(XWinMessage::SetClipboardString(value, tx), rx)?
}

/// Sets the current XWin time, in seconds. The value must be a positive finite
/// number less than or equal to `18446744073.0`, which is approximately 584.5
/// yearsa.
///
/// This function and [try_get_time] are helper functions on top of
/// [try_timer_frequency] and [try_timer_value].
///
/// # Thread Safety
/// Reading and writing of the internal base time is not atomic, so it needs to
/// be externally synchronized with calls to [try_set_time].
///
/// # Errors
/// Possible errors include [XErr::NotInitialized].
///
/// # Remarks
/// The upper limit of XWin time is calculated as `floor((2^64 - 1) / 10^9)` and
/// is due to implementations storing nanoseconds in 64 bits. The limit may be
/// increased in the future.
pub fn try_set_time(value: f64) -> Result<(), XErr>
{
	unsafe { glfwSetTime(value) };
	XErr::result(|| ())
}

/// Returns the current XWin time, in seconds. Unless the time has been set
/// using [try_set_time] it measures the time elapsed since XWin was
/// initialized.
///
/// This function and [try_set_time] are helper functions on top of
/// [try_timer_frequency] and [try_timer_value].
///
/// The resolution of the timer is system dependent, but is usually on the order
/// of a few micro- or nanoseconds. It uses the highest-resolution monotonic
/// time source on each operating system.
///
/// # Thread Safety
/// Reading and writing of the internal base time is not atomic, so it needs to
/// be externally synchronized with calls to [try_set_time].
///
/// # Errors
/// Possible errors include [XErr::NotInitialized].
pub fn try_time() -> Result<f64, XErr>
{
	let time = unsafe { glfwGetTime() };
	XErr::result(|| time)
}

/// Returns the frequency, in Hz, of the raw timer.
///
/// # Errors
/// Possible errors include [XErr::NotInitialized].
pub fn try_timer_frequency() -> Result<u64, XErr>
{
	let time = unsafe { glfwGetTimerFrequency() };
	XErr::result(|| time)
}

/// Returns the current value of the raw timer, measured in `1/frequency`
/// seconds. To get the frequency, call [try_timer_frequency].
///
/// # Errors
/// Possible errors include [XErr::NotInitialized].
pub fn try_timer_value() -> Result<u64, XErr>
{
	let time = unsafe { glfwGetTimerValue() };
	XErr::result(|| time)
}
