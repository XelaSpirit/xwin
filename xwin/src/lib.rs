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
//! easy to learn. If raw GLFW bindings are needed, the `bindings` feature may
//! be enabled to expose them under the [bind] module. See
//! [Features](crate#crate-features).
//!
//! XWin does not, however, provide any means of rendering with
//! OpenGL or Vulkan. This means another library will be needed to perform the
//! actual rendering. XWin also does not support the creation/management of
//! OpenGL contexts (though support may be added in the future). If this
//! behavior is desired, you will need to enable the `bindings` feature and use
//! the GLFW bindings directly. See [Features](crate#crate-features).
//!
//! TODO - add links to rendering library
//!
//! Since this is just a wrapper library, much of the documentation is taken
//! directly from the [GLFW documentation](https://www.glfw.org/docs/latest/index.html),
//! with some alteration where necessary to better describe the XWin API.
//!
//! # Getting Started
//! TODO give `Cargo.lock` entry
//! TODO getting started glfw doc
//!
//! # Dependencies
//! XWin relies on the [cmake](https://docs.rs/cmake/latest/cmake/) crate to compile GLFW,
//! which itself runs the system `cmake` command to build GLFW. It is up to the
//! user of this library to ensure cmake is available during compilation.
//!
//! # Crate Features
//! XWin provides a number of optional features which may be enabled to expose
//! additional functions or add additional functionality. For more information
//! on raw GLFW bindings, see the [GLFW documentation](https://www.glfw.org/docs/latest/index.html).
//!
//! By default, both `vulkan` and `glfw` are enabled.
//!
//! The features available are as follows:
//! - `glfw`. Exposes functions `as_glfw`, `to_glfw`, and `from_glfw` for a
//!   number of XWin structs, allowing for access to the underlying GLFW types.
//!   This is separate from `bindings` in that it only exports the types used by
//!   these structs, and not the entirety of the GLFW api. This is primarily
//!   intended for interfacing with other libraries which work with these types.
//! - `bindings`. Enables the `glfw` feature. Exports the raw GLFW bindings
//!   under the [bind] module.
//! - `vulkan`. Enables the `glfw` feature, and exports a [vulkan] module
//!   containing functions related to vulkan functionality. Note that this does
//!   not include any actual rendering code, only information that may be
//!   necessary for another library to render to an XWin window. Also exports
//!   the `VkInstance` and `VkPhysicalDevice` types.
//! - `cocoa`. Enables the `bindings` feature. Exports GLFW's native Cocoa
//!   functions.
//! - `wayland`. Enables the `bindings` feature. Exports GLFW's native Wayland
//!   functions.
//! - `win32`. Enables the `bindings` feature. Exports GLFW's native Win32
//!   functions.
//! - `x11`. Enables the `bindings` feature. Exports GLFW's native X11
//!   functions.
//! - `egl`. Enables the `bindings` feature. Exports GLFW's native EGL
//!   functions.
//! - `glx`. Enables the `bindings` feature. Exports GLFW's native GLX
//!   functions.
//! - `nsgl`. Enables the `bindings` feature. Exports GLFW's native NSGL
//!   functions.
//! - `osmesa`. Enables the `bindings` feature. Exports GLFW's native Osmesa
//!   functions.
//! - `wgl`. Enables the `bindings` feature. Exports GLFW's native WGL
//!   functions.

pub mod core;
pub(crate) mod crate_util;
pub mod error;
pub mod event;
pub mod input;
pub mod monitor;
pub mod utility;
pub mod window;

#[cfg(feature = "bindings")]
pub mod bind;
#[cfg(not(feature = "bindings"))]
pub(crate) mod bind;

#[cfg(feature = "vulkan")]
pub mod vulkan;
