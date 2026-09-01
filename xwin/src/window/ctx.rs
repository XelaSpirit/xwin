use xch::Sender;

use crate::{
	bind::{
		GLFWwindow,
		glfwGetWindowUserPointer,
	},
	window::{
		KeyEvent,
		WindowEvent,
	},
};

pub(crate) struct WindowContext
{
	ev_tx:  Option<Box<dyn Sender<WindowEvent> + Send + Sync>>,
	key_tx: Option<Box<dyn Sender<KeyEvent> + Send + Sync>>,
}

impl WindowContext
{
	pub(crate) fn new() -> Self
	{
		WindowContext {
			ev_tx:  None,
			key_tx: None,
		}
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

	pub(super) fn set_key_tx<T>(&mut self, tx: T)
	where
		T: Sender<KeyEvent> + Send + Sync + 'static,
	{
		self.key_tx = Some(Box::new(tx));
	}

	pub(super) fn remove_key_tx(&mut self)
	{
		self.key_tx = None;
	}

	pub(super) fn post_config(&mut self, evt: WindowEvent)
	{
		if let Some(tx) = &self.ev_tx
		{
			if let Err(_) = tx.send(evt)
			{
				self.ev_tx = None;
			}
		}
	}

	pub(super) fn post_key(&mut self, evt: KeyEvent)
	{
		if let Some(tx) = &self.key_tx
		{
			if let Err(_) = tx.send(evt)
			{
				self.key_tx = None;
			}
		}
	}
}
