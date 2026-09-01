//! # xwin
//! XWin is a library that provides a wrapper around GLFW. From the
//! [GLFW documentation](https://www.glfw.org/docs/latest/index.html): "GLFW is
//! a free, Open Source, multi-platform library for OpenGL, OpenGL ES and Vulkan
//! application development. It provides a simple, platform-independent API for
//! creating windows, contexts and surfaces, reading input, handling events,
//! etc."
//!
//! Rather than just raw bindings, XWin wraps the GLFW API in a more
//! rust-friendly api that should be easier to work with. Still, those familiar
//! with the GLFW API from another language should find this library relatively
//! easy to learn. XWin does not, however, provide any means of working with
//! OpenGL or Vulkan.
//!
//! TODO - add links to Vulkan library
//!
//! Since this is just a wrapper library, much of the documentation is taken
//! directly from the [GLFW documentation](https://www.glfw.org/docs/latest/index.html),
//! with some alteration where necessary to better match the XWin API.
//!
//! ### Dependencies
//! - CMake

mod bind;
pub mod core;
pub mod err;
