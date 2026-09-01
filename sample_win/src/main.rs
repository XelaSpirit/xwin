use std::sync::mpsc;

use xwin::{
	input::keyboard::Key,
	window::Window,
};

fn main()
{
	xwin::core::init(|| {
		let mut win = Window::try_new(1920, 1080, "Sample", None).expect("Failed to create window");
		let (tx, rx) = mpsc::channel();
		win.set_key_channel(tx).expect("Unable to set key channel");

		while !win.should_close()
		{
			if let Ok(evt) = rx.recv()
			{
				match evt.key()
				{
					| Key::Escape => win.set_should_close(true),
					| _ =>
					{},
				}
			}
		}
	})
	.expect("Failed to initialize xwin");
}
