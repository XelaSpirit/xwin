//! Functions for Vulkan compatability.

#[cfg(feature = "vulkan")]
mod glfw;
#[cfg(feature = "vulkan")]
pub use glfw::*;
