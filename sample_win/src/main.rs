use xwin::{
	core::XWin,
	window::Window,
};

fn main()
{
	XWin::init(|| {
		let win = Window::try_create(1920, 1080, "Sample", None).expect("Failed to create window");
		while !win.should_close()
		{}
	})
	.expect("Failed to initialize xwin");
}
