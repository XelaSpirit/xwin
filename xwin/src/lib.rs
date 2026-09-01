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
//! OpenGL or Vulkan. This means another library will be needed to perform the
//! actual rendering
//!
//! TODO - add links to rendering library
//!
//! Since this is just a wrapper library, much of the documentation is taken
//! directly from the [GLFW documentation](https://www.glfw.org/docs/latest/index.html),
//! with some alteration where necessary to better match the XWin API.
//!
//! # Introduction
//! For an introduction to the basic concepts of XWin, including initialization
//! and error handling, see the documentation for the [core module](core).
//!
//! For a broad but shallow tutorial, see [#Getting Started] below.
//!
//! Different modules also contain their own guides introducing their
//! functionality:
//! - [core]
//! - [err]
//! - [monitor]
//!
//! There are also guides for the other areas of XWin:
//! 	 - TODO window, context, monitor, input, (Vulkan?)
//!
//! # Getting Started
//! TODO give `Cargo.lock` entry
//! TODO getting started glfw doc
//!
//! ### Dependencies
//! - CMake

mod bind;
pub mod core;
pub mod err;
pub mod monitor;

pub use linkme as __linkme;
