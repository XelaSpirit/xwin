use xch::Sender;

use crate::{
	bind::{
		GLFWwindow,
		glfwGetWindowUserPointer,
	},
	error::XErr,
	window::{
		KeyEvent,
		WindowEvent,
	},
};

pub(crate) struct WindowContext
{
	cfg_tx:  Option<Box<dyn Sender<WindowEvent> + Send + Sync>>,
	char_tx: Option<Box<dyn Sender<u32> + Send + Sync>>,
	key_tx:  Option<Box<dyn Sender<KeyEvent> + Send + Sync>>,
}

impl WindowContext
{
	pub(crate) fn new() -> Self
	{
		WindowContext {
			cfg_tx:  None,
			char_tx: None,
			key_tx:  None,
		}
	}

	pub(super) fn with_context<F>(win: &*mut GLFWwindow, err: &str, func: F) -> Result<(), XErr>
	where
		F: FnOnce(&mut WindowContext),
	{
		if let Some(ctx) =
			unsafe { (glfwGetWindowUserPointer(*win) as *mut WindowContext).as_mut() }
		{
			func(ctx);
			Ok(())
		}
		else
		{
			Err(XErr::NotInitialized(err.to_string()))
		}
	}

	pub(super) fn set_cfg_tx<T>(&mut self, tx: T)
	where
		T: Sender<WindowEvent> + Send + Sync + 'static,
	{
		self.cfg_tx = Some(Box::new(tx));
	}

	pub(super) fn remove_cfg_tx(&mut self)
	{
		self.cfg_tx = None;
	}

	pub(super) fn set_char_tx<T>(&mut self, tx: T)
	where
		T: Sender<u32> + Send + Sync + 'static,
	{
		self.char_tx = Some(Box::new(tx));
	}

	pub(super) fn remove_char_tx(&mut self)
	{
		self.char_tx = None;
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
		if let Some(tx) = &self.cfg_tx
		{
			if let Err(_) = tx.send(evt)
			{
				self.cfg_tx = None;
			}
		}
	}

	pub(super) fn post_char(&mut self, evt: u32)
	{
		if let Some(tx) = &self.char_tx
		{
			if let Err(_) = tx.send(evt)
			{
				self.char_tx = None;
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
