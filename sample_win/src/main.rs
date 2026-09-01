use xwin::window::Window;

fn main()
{
	xwin::core::init(|| {
		let mut win = Window::try_new(1920, 1080, "Sample", None).expect("Failed to create window");
		win.set_decorated(true);
		while !win.should_close()
		{}
	})
	.expect("Failed to initialize xwin");
}
