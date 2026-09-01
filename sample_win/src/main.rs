use xwin::{
	core::XWin,
	window::{
		Window,
		WindowBuilder,
	},
};
// TODO - New thread for each window? Prevent context from being current on
// multiple threads Maybe make Windows not Send, and use function to send
// windows that ensures detach? Or have set_current return result, and store id
// of thread that it's on (would always require detach to be called explicitly)

fn main()
{
	XWin::init(|| {
		let win = Window::create(1920, 1080, "Sample", None).expect("Failed to create window");
		while !win.should_close().unwrap()
		{}
	})
	.expect("Failed to initialize xwin");
}
