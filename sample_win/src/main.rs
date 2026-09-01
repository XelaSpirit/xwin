use xwin::window::Window;

fn main()
{
	xwin::core::init(|| {
		let win = Window::try_new(1920, 1080, "Sample", None).expect("Failed to create window");
		while !win.should_close()
		{}
	})
	.expect("Failed to initialize xwin");
}
