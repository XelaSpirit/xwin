use std::ptr::null_mut;

use crate::bind::GLFWimage;

/// Describes a single 2D image. See the documentation for related functions for
/// what the expected pixel format is.
pub struct Image(GLFWimage, usize, usize);

impl Image
{
	/// Construct a new, empty [Image]
	pub fn new() -> Self
	{
		Image(
			GLFWimage {
				width:  0,
				height: 0,
				pixels: null_mut(),
			},
			0,
			0,
		)
	}

	/// Returns the width of this [Image]
	pub fn width(&self) -> i32
	{
		self.0.width
	}

	/// Returns the height of this [Image]
	pub fn height(&self) -> i32
	{
		self.0.height
	}

	/// Returns a view of the pixels in this [Image]
	pub fn pixels(&self) -> &[u8]
	{
		unsafe { std::slice::from_raw_parts(self.0.pixels, self.1) }
	}

	/// Update the pixels in this [Image]. The given [Vec] must have a length
	/// equal to `width * height * 4`.
	pub fn set_pixels(&mut self, width: i32, height: i32, pixels: Vec<u8>)
	{
		debug_assert_eq!(width as usize * height as usize * 4, pixels.len());
		self.0.width = width;
		self.0.height = height;

		let parts = pixels.into_raw_parts();
		self.0.pixels = parts.0;
		self.1 = parts.1;
		self.2 = parts.2;
	}

	#[cfg(feature = "glfw")]
	pub fn to_glfw(mut self) -> GLFWimage
	{
		self.1 = 0;
		self.0
	}

	#[cfg(feature = "glfw")]
	pub fn from_glfw(img: GLFWimage) -> Self
	{
		Self(img, img.width as usize * img.height as usize, img.width as usize * img.height as usize)
	}

	/// Returns the underlying [GLFWimage] descriptor used by GLFW APIs.
	pub(crate) fn as_glfw(&self) -> GLFWimage
	{
		self.0
	}
}

impl Drop for Image
{
	fn drop(&mut self)
	{
		if self.1 > 0 || self.2 > 0
		{
			let vec = unsafe { Vec::from_raw_parts(self.0.pixels, self.1, self.2) };
			drop(vec);
		}
	}
}
