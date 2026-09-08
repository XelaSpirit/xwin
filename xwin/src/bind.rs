//! Module containing raw GLFW bindings

pub(crate) mod glfw;

#[cfg(feature = "bindings")]
pub use glfw::*;
#[cfg(not(feature = "bindings"))]
pub(crate) use glfw::*;
