use crate::bind::GLFWvidmode;

/// A struct containing the width, height, rgb bit depth, and refresh rate of a
/// video mode for a monitor.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct VideoMode
{
	width:        i32,
	height:       i32,
	red_bits:     i32,
	green_bits:   i32,
	blue_bits:    i32,
	refresh_rate: i32,
}

impl VideoMode
{
	/// The width, in screen coordinates, of the video mode.
	pub fn width(&self) -> i32
	{
		self.width
	}

	/// The height, in screen coordinates, of the video mode
	pub fn height(&self) -> i32
	{
		self.height
	}

	/// The bit depth of the red channel of the video mode.
	pub fn red_bits(&self) -> i32
	{
		self.red_bits
	}

	/// The bit depth of the green channel of the video mode.
	pub fn green_bits(&self) -> i32
	{
		self.green_bits
	}

	/// The bit depth of the blue channel of the video mode.
	pub fn blue_bits(&self) -> i32
	{
		self.blue_bits
	}

	/// The refresh rate, in Hz, of the video mode.
	pub fn refresh_rate(&self) -> i32
	{
		self.refresh_rate
	}

	/// Construct a video mode from a GLFWvidmode
	pub(crate) fn from_glfw(vm: &GLFWvidmode) -> Self
	{
		VideoMode {
			width:        vm.width,
			height:       vm.height,
			red_bits:     vm.redBits,
			green_bits:   vm.greenBits,
			blue_bits:    vm.blueBits,
			refresh_rate: vm.refreshRate,
		}
	}
}
