use xch::Sender;

use crate::{
	bind::{
		GLFWwindow,
		glfwGetWindowUserPointer,
	},
	window::WindowEvent,
};

pub(crate) struct WindowContext
{
	ev_tx: Option<Box<dyn Sender<WindowEvent> + Send + Sync>>,
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

	pub(super) fn set_ev_tx<T>(&mut self, tx: T)
	where
		T: Sender<WindowEvent> + Send + Sync + 'static,
	{
		self.ev_tx = Some(Box::new(tx));
	}

	pub(super) fn remove_ev_tx(&mut self)
	{
		self.ev_tx = None;
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
