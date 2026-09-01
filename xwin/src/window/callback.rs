use crate::{
	core::{
		ContentScale,
		ScreenCoordinates,
	},
	window::Window,
};

/// Almost all positions and sizes in XWin are measured in
/// [ScreenCoordinates](ScreenCoordinates). However, framebuffer sizes
/// are measured in pixels.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct Pixels
{
	pub x: i32,
	pub y: i32,
}

pub type WindowPosFn = fn(&Window, ScreenCoordinates);
pub type WindowSizeFn = fn(&Window, ScreenCoordinates);
pub type WindowCloseFn = fn(&Window);
pub type WindowRefreshFn = fn(&Window);
pub type WindowFocusFn = fn(&Window, bool);
pub type WindowIconifyFn = fn(&Window, bool);
pub type WindowMaximizeFn = fn(&Window, bool);
pub type WindowBufferSizeFn = fn(&Window, Pixels);
pub type WindowContentScaleFn = fn(&Window, ContentScale);
