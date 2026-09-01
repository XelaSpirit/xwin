use crate::bind::{
	GLFW_TRUE,
	glfwInit,
};

pub fn init() -> Result<(), ()>
{
	unsafe {
		if glfwInit() == GLFW_TRUE as i32
		{
			Ok(())
		}
		else
		{
			Err(())
		}
	}
}
