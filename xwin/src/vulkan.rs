use crate::bind::{glfwVulkanSupported, GLFW_TRUE};
use crate::error::XErr;

/// See [try_is_vulkan_supported].
pub fn is_vulkan_supported() -> bool
{
	try_is_vulkan_supported().unwrap_or_default()
}

// TODO finish doc
/// Returns whether the Vulkan loader and any minimally functional ICD have been
/// found.
///
/// The availability of a Vulkan loader and even an ICD does not by itself
/// guarantee that surface creation or even instance creation is possible.
///
/// # Errors
/// Possible errors include [XErr::NotInitialized].
pub fn try_is_vulkan_supported() -> Result<bool, XErr>
{
	let value = unsafe { glfwVulkanSupported() };
	XErr::result(|| {
		if value == GLFW_TRUE as i32
		{
			true
		}
		else
		{
			false
		}
	})
}