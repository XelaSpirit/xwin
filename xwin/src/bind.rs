//! Module containing raw GLFW bindings

mod glfw;

#[cfg(feature = "bindings")]
pub use glfw::*;
#[cfg(not(feature = "bindings"))]
pub(crate) use glfw::*;
