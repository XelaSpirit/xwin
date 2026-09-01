use crate::bind::GLFWgammaramp;

#[derive(Clone, Debug)]
pub struct GammaRamp
{
	size:  u32,
	red:   Vec<u16>,
	green: Vec<u16>,
	blue:  Vec<u16>,
}

impl Default for GammaRamp
{
	/// Constructs and returns a new [GammaRamp], with no values in each channel
	fn default() -> Self
	{
		GammaRamp {
			size:  0,
			red:   vec![],
			green: vec![],
			blue:  vec![],
		}
	}
}

impl GammaRamp
{
	/// Constructs and returns a new [GammaRamp] with a given size, where all
	/// values are set to `fill`
	pub fn new(size: u32, fill: u16) -> Self
	{
		GammaRamp {
			size,
			red: vec![fill; size as usize],
			green: vec![fill; size as usize],
			blue: vec![fill; size as usize],
		}
	}

	/// Constructs and returns a new [GammaRamp], where all values are set to
	/// the value returned by calling `f` with the index of that value (`0..S`).
	pub fn from_fn<F>(size: u32, f: F) -> Self
	where
		F: Fn(u32) -> u16,
	{
		let mut ramp = GammaRamp {
			size,
			red: Vec::with_capacity(size as usize),
			green: Vec::with_capacity(size as usize),
			blue: Vec::with_capacity(size as usize),
		};

		for idx in 0..size
		{
			ramp.red.push(f(idx as u32));
			ramp.green.push(f(idx as u32));
			ramp.blue.push(f(idx as u32));
		}
		ramp
	}

	/// Returns the size of the array stored in this ramp.
	pub fn size(&self) -> u32
	{
		self.size
	}

	/// Returns the value in the red array at index `idx`.
	pub fn red(&self, idx: usize) -> u16
	{
		self.red[idx]
	}

	/// Returns the value in the green array at index `idx`.
	pub fn green(&self, idx: usize) -> u16
	{
		self.green[idx]
	}

	/// Returns the value in the blue array at index `idx`.
	pub fn blue(&self, idx: usize) -> u16
	{
		self.blue[idx]
	}

	/// Sets the value in the red array at index `idx`.
	pub fn set_red(&mut self, idx: usize, val: u16)
	{
		self.red[idx] = val;
	}

	/// Sets the value in the green array at index `idx`.
	pub fn set_green(&mut self, idx: usize, val: u16)
	{
		self.green[idx] = val;
	}

	/// Sets the value in the blue array at index `idx`.
	pub fn set_blue(&mut self, idx: usize, val: u16)
	{
		self.blue[idx] = val;
	}

	/// Construct a gamma ramp from a GLFWgammaramp.
	pub(crate) fn from_glfw(ramp: &GLFWgammaramp) -> Self
	{
		let mut gr = GammaRamp {
			size: ramp.size,
			red: Vec::with_capacity(ramp.size as usize),
			green: Vec::with_capacity(ramp.size as usize),
			blue: Vec::with_capacity(ramp.size as usize),
		};
		for idx in 0..ramp.size as usize {
			gr.red.push(unsafe { *ramp.red.add(idx) });
			gr.green.push(unsafe { *ramp.green.add(idx) });
			gr.blue.push(unsafe { *ramp.blue.add(idx) });
		}
		gr
	}

	/// Runs the function `f`, passing in a [GLFWgammaramp] constructed from
	/// this ramp.
	pub(crate) fn with_glfw<F, R>(&mut self, f: F) -> R
	where
		F: FnOnce(&GLFWgammaramp) -> R,
	{
		let ramp = GLFWgammaramp {
			size:  self.size,
			red:   self.red.as_mut_ptr(),
			green: self.green.as_mut_ptr(),
			blue:  self.blue.as_mut_ptr(),
		};
		f(&ramp)
	}
}
