use std::sync::mpsc::SyncSender;

use crate::{
	bind::{
		GLFWwindow,
		glfwGetWindowUserPointer,
	},
	window::WindowEvent,
};

pub(crate) struct WindowContext
{
	ev_tx: Option<SyncSender<WindowEvent>>,
}

impl WindowContext
{
	pub(crate) fn new() -> Self
	{
		WindowContext { ev_tx: None }
	}

	pub(super) fn get(win: &*mut GLFWwindow) -> Option<&mut WindowContext>
	{
		unsafe { (glfwGetWindowUserPointer(*win) as *mut WindowContext).as_mut() }
	}

	pub(super) fn set_ev_tx(&mut self, tx: Option<SyncSender<WindowEvent>>)
	{
		self.ev_tx = tx;
	}

	pub(super) fn post(&mut self, event: WindowEvent)
	{
		if let Some(tx) = &self.ev_tx
		{
			if let Err(_) = tx.send(event)
			{
				self.ev_tx = None;
			}
		}
	}
}
