//! XWin event types

// TODO documentation

pub use crate::{
	input::event::*,
	monitor::event::*,
	window::event::*,
};

/// Enum containing each of the different events from XWin.
///
/// This is intended to be used with [xch::funnel::channel], to allow handling
/// multiple different events on a single channel.
#[derive(Debug, Clone)]
pub enum XWinEvent
{
	JoystickConfig(JoystickConfigEvent),
	Window(WindowEvent),
	Key(KeyEvent),
	Monitor(MonitorEvent),
	MouseButton(MouseButtonEvent),
	Mouse(MouseEvent),
}

impl From<JoystickConfigEvent> for XWinEvent
{
	fn from(value: JoystickConfigEvent) -> Self
	{
		XWinEvent::JoystickConfig(value)
	}
}

impl From<WindowEvent> for XWinEvent
{
	fn from(value: WindowEvent) -> Self
	{
		XWinEvent::Window(value)
	}
}

impl From<KeyEvent> for XWinEvent
{
	fn from(value: KeyEvent) -> Self
	{
		XWinEvent::Key(value)
	}
}

impl From<MonitorEvent> for XWinEvent
{
	fn from(value: MonitorEvent) -> Self
	{
		XWinEvent::Monitor(value)
	}
}

impl From<MouseButtonEvent> for XWinEvent
{
	fn from(value: MouseButtonEvent) -> Self
	{
		XWinEvent::MouseButton(value)
	}
}

impl From<MouseEvent> for XWinEvent
{
	fn from(value: MouseEvent) -> Self
	{
		XWinEvent::Mouse(value)
	}
}
