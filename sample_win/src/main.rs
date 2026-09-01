use xwin::{
	core::XWin,
	window::Window,
};

fn main()
{
	XWin::init(|| {
		let win =
			Window::create(1920, 1080, "Sample", None, None).expect("Failed to create window");
		while !win.should_close().unwrap()
		{}
	})
	.expect("Failed to initialize xwin");
}
