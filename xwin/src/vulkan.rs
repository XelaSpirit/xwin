//! Functions for Vulkan compatability.
//!
//! If you have enabled the `vulkan` feature, be sure to set the
//! `VULKAN_INCLUDE_PATH` environment variable. It must be the location of the
//! `vulkan` folder containing `vulkan.h`. If you're using the LunarG vulkan
//! SDK, this would be the location of the `Include` folder inside the SDK.

#[cfg(feature = "vulkan")]
mod glfw;
#[cfg(feature = "vulkan")]
pub use glfw::*;
