use std::{
	ffi::CStr,
	os::raw::c_uint,
	slice,
};

use crate::{
	bind::{
		GLFW_TRUE,
		VkInstance,
		VkPhysicalDevice,
		glfwGetPhysicalDevicePresentationSupport,
		glfwGetRequiredInstanceExtensions,
		glfwVulkanSupported,
	},
	error::XErr,
};

/// See [try_is_vk_supported].
pub fn is_vk_supported() -> bool
{
	try_is_vk_supported().unwrap_or_default()
}

/// See [try_vk_physical_device_presentation_support].
pub fn vk_physical_device_presentation_support(
	instance: VkInstance,
	device: VkPhysicalDevice,
	queue_family: u32,
) -> bool
{
	try_vk_physical_device_presentation_support(instance, device, queue_family).unwrap_or_default()
}

/// See [try_vk_required_extensions].
pub fn vk_required_extensions() -> Vec<String>
{
	try_vk_required_extensions().unwrap_or_default()
}

/// Returns whether the Vulkan loader and any minimally functional ICD have been
/// found.
///
/// The availability of a Vulkan loader and even an ICD does not by itself
/// guarantee that surface creation or even instance creation is possible. Call
/// [try_vk_required_instance_extensions] to check whether the extensions
/// necessary for Vulkan surface creation are available and
/// [try_vk_physical_device_presentation_support] to check whether a queue
/// family of a physical device supports image presentation.
///
/// # Errors
/// Possible errors include [XErr::NotInitialized].
pub fn try_is_vk_supported() -> Result<bool, XErr>
{
	let value = unsafe { glfwVulkanSupported() };
	XErr::result(|| value == GLFW_TRUE as i32)
}

/// Returns whether the specified queue family of the specified physical device
/// supports presentation to the platform XWin was built for.
///
/// If Vulkan or the required window surface creation instance extensions are
/// not available on the machine, or if the specified instance was not created
/// with the required extensions, this function returns [XErr::ApiUnavailable].
/// Call [try_is_vk_supported] to check whether Vulkan is at least minimally
/// available and [try_vk_required_extensions] to check what instance extensions
/// are required.
///
/// # Errors
/// Possible errors include [XErr::NotInitialized], [XErr::ApiUnavailable], and
/// [XErr::Platform].
///
/// # Remarks
/// - **macOS**. This function currently always returns `true`, as the
///   `VK_MVK_macos_surface` and `VK_EXT_metal_surface` extensions do not
///   provide a `vkGetPhysicalDevice*PresentationSupport` type function.
pub fn try_vk_physical_device_presentation_support(
	instance: VkInstance,
	device: VkPhysicalDevice,
	queue_family: u32,
) -> Result<bool, XErr>
{
	let res = unsafe { glfwGetPhysicalDevicePresentationSupport(instance, device, queue_family) };
	XErr::result(|| res == GLFW_TRUE as i32)
}

/// Returns a [Vec] of names of Vulkan instance extensions required by XWin for
/// creating Vulkan surfaces for XWin windows. If successful, the list will
/// always contain `VK_KHR_surface`, so if you don't require any additional
/// extensions you can pass this list directly to the `VkInstanceCreateInfo`
/// struct.
///
/// If Vulkan is not available on the machine, this function return
/// [XErr::ApiUnavailable]. Call [try_is_vk_supported] to check whether
/// Vulkan is at least minimally available.
///
/// If Vulkan is available but no set of extensions allowing window surface
/// creation was found, this function returns an empty [Vec]. You may still use
/// Vulkan for off-screen rendering and compute work.
///
/// # Errors
/// Possible errors include [XErr::NotInitialized] and [XErr::ApiUnavailable].
///
/// # Remarks
/// Additional extensions may be required by future versions of XWin. You should
/// check if any extensions you wish to enable are already in the returned
/// [Vec], as it is an error to specify an extension more than once in the
/// `VkInstanceCreateInfo` struct.
pub fn try_vk_required_extensions() -> Result<Vec<String>, XErr>
{
	let mut count: c_uint = 0;
	let ext = unsafe { glfwGetRequiredInstanceExtensions(&mut count) };

	XErr::result(|| {
		if ext.is_null()
		{
			Vec::new()
		}
		else
		{
			unsafe { slice::from_raw_parts(ext, count as usize) }
				.iter()
				.map(|&ptr| {
					if ptr.is_null()
					{
						String::new()
					}
					else
					{
						unsafe { CStr::from_ptr(ptr) }
							.to_string_lossy()
							.into_owned()
					}
				})
				.collect()
		}
	})
}
