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
//! Since this is just a wrapper library, much of the documentation is taken
//! directly from the [GLFW documentation](https://www.glfw.org/docs/latest/index.html),
//! with some alteration where necessary to better describe the XWin API. See
//! [getting started](crate#getting-started) for a quick-start guide on using
//! XWin.
//!
//! # Dependencies
//! XWin relies on the [cmake](https://docs.rs/cmake/latest/cmake/) crate to compile GLFW,
//! which itself runs the system `cmake` command to build GLFW. It is up to the
//! user of this library to ensure cmake is available during compilation.
//!
//! # Getting Started
//! Before you can use most XWin functions, the library must be initialized.
//! XWin *must* be initialized on the main thread, and once initialized XWin
//! will take control of the main thread until termination. A new thread will be
//! spawned for your own code to run on. This is due to platform limitations
//! requiring many GLFW functions to be run on the main thread. After
//! initialization, XWin will handle passing messages to and from the main
//! thread for functions that require it. This means you shouldn't need to worry
//! about multithreading in your own program. Note, however, that this does mean
//! many functions in this library need to wait on some function to run on the
//! main thread. This means if you have many threads all calling XWin functions
//! frequently enough, you may encounter performance problems as all those
//! threads wait their turn for the main thread. That said, given the purpose of
//! this library, unless you intend to rename a window many times per second or
//! something similar, this shouldn't be an issue.
//!
//! ## Initializing and Terminating XWin
//! To initialize XWin, simply call [core::init] from the main thread, passing
//! in a function to run while XWin is initialized.
//!
//! ```
//! # fn main()
//! # {
//! xwin::core::init(|| {
//! 	// run_program();
//! })
//! .expect("Failed to initialize XWin");
//! # }
//! ```
//!
//! The [init](core::init) function will halt until the passed function
//! terminates or XWin is terminated, at which point it will return `Ok(())`.
//! The passed function should therefore continue running until you intend to
//! close the program, likely using a loop calling
//! [Window::should_close](window::Window::should_close).
//!
//! If you want to terminate XWin early, you can call [core::terminate]. Once
//! this is called, XWin will destroy any remaining windows and release any
//! resources it has allocated, then [core::init] will return, and most XWin
//! functions will no longer be useful. You must initialize again before using
//! any XWin functions that require it.
//!
//! ## Errors
//! Most functions in XWin return `Result<_, XErr>`. Each error type contains a
//! `String` describing the error. Note that not all functions that may return
//! [XErr](error::XErr) necessarily return all types of errors. See the
//! documentation for [XErr](error::XErr) for details on what the different
//! types of errors are, and consult the documentation for a particular
//! functions for what errors that function may return.
//!
//! Functions that may return errors are prefixed with 'try_'. Many of these
//! functions have a variant that does not include the 'try_' prefix and does
//! not return a `Result`. Such functions are a shorthand for calling the 'try_'
//! variant, and return the return type's default value in the case of an error.
//! Such function's documentation will point you to the 'try_' variant of the
//! function for reference.
//!
//! ## Creating a Window
//! A [Window](window::Window) can be created using
//! [Window::try_new](window::Window::try_new) for a window with default
//! options, or by using a [WindowBuilder](window::WindowBuilder) for more
//! customization.
//!
//! ```
//! # use xwin::window::Window;
//! # fn main()
//! # {
//! # xwin::core::init(|| {
//! let win = Window::try_new(1920, 1080, "Sample", None).expect("Failed to create window");
//! # })
//! # .expect("Failed to initialize XWin");
//! # }
//! ```
//!
//! When a [Window](window::Window) is no longer needed, drop it. XWin will
//! handle window destruction automatically once the window drops. Once the
//! window has been destroyed, no more events will be generated for the window.
//! Note that window destruction happens on the main thread, meaning an event
//! may be triggered between the time the [Window](window::Window) drops and
//! when the actual window is destroyed.
//!
//! ### The Window Close Flag
//! Each [Window](window::Window) has a flag indicating whether the window
//! should be closed.
//!
//! When the user attempts the close the window, either by pressing the close
//! widget in the title bar or using a key combination like Alt+F4, this flag is
//! set to `true`. Note that **the window isn't actually closed,** so you are
//! expected to monitor this flag and either destroy the window or give some
//! kind of feedback to the user.
//!
//! ```
//! # use xwin::window::Window;
//! # fn main()
//! # {
//! # xwin::core::init(|| {
//! # let mut win = Window::try_new(1920, 1080, "Sample", None).expect("Failed to create window");
//! while !win.should_close()
//! {
//! 	// Keep running
//! 	# win.set_should_close(true);
//! }
//! // Drop window
//! # })
//! # .expect("Failed to initialize XWin");
//! # }
//! ```
//!
//! You can be notified when the user is attempting to close the window by
//! listening for close events on the window's [config event
//! channel](window::Window::set_config_channel).
//! [WindowEvent::Close](event::WindowEvent::Close) is sent on the
//! channel immediately after the close flag has been set.
//!
//! You can also set it yourself with
//! [Window::set_should_close](window::Window::set_should_close). This can be
//! useful if you want to interpret other kinds of input as closing the window,
//! such as pressing the escape key.
//!
//! ## Receiving Events
//! Each window has a number of channels that can be used to receive the various
//! kinds of events. To receive key press and release events, for example,
//! listen on the [key event channel](window::Window::set_key_channel).
//!
//! ```
//! # use xwin::window::Window;
//! # use std::sync::mpsc::channel;
//! # use std::time::Duration;
//! # fn main()
//! # {
//! # xwin::core::init(|| {
//! # let mut win = Window::try_new(1920, 1080, "Sample", None).expect("Failed to create window");
//! let (tx, rx) = channel();
//! win.set_key_channel(tx);
//! if let Ok(evt) = rx.recv_timeout(Duration::from_millis(10))
//! {
//! 	// Handle event
//! }
//! # })
//! # .expect("Failed to initialize XWin");
//! # }
//! ```
//!
//! XWin will handle polling for events on the main thread and send messages on
//! the relevant channels as events occur.
//!
//! Each type of event for a window has its own channel. These channel accept
//! any sender that implements [xch::Sender] from the `xela_channels` crate.
//! This trait is already implemented for the standard mpsc channel.
//! `xela_channels` also provides the [xch::funnel] channel. When combined with
//! the [XWinEvent](event::XWinEvent) enum, the funnel channel can be used to
//! put all of a window's events on a single channel.
//!
//! ```
//! # use xwin::window::Window;
//! # use std::sync::mpsc::channel;
//! # use std::time::Duration;
//! # use xch::funnel;
//! # use std::sync::mpsc;
//! # use xwin::event::XWinEvent;
//! # fn main()
//! # {
//! # xwin::core::init(|| {
//! # let mut win = Window::try_new(1920, 1080, "Sample", None).expect("Failed to create window");
//! let (tx, rx) = funnel::channel(mpsc::channel::<XWinEvent>());
//! win.set_key_channel(tx.clone());
//! win.set_config_channel(tx.clone());
//! if let Ok(evt) = rx.recv_timeout(Duration::from_millis(10))
//! {
//! 	// Handle event
//! }
//! # })
//! # .expect("Failed to initialize XWin");
//! # }
//! ```
//!
//! ## Rendering
//!
//! XWin does not provide any means of rendering with OpenGL or Vulkan. This
//! means another library will be needed to perform the actual rendering. XWin
//! also does not support the creation/management of OpenGL contexts (though
//! support may be added in the future). If this behavior is desired, you will
//! need to enable the `bindings` feature and use the GLFW bindings directly.
//! See [Features](crate#crate-features). If you intend to render using Vulkan,
//! you should enable the `vulkan` feature.
//!
//! ## Reading the Timer
//! To create smooth animation, a time source is needed. XWin provides a timer
//! that returns the number of seconds since initialization. The time source
//! used is the most accurate on each platform and generally has a micro- or
//! nanosecond resolution.
//!
//! ```
//! # fn main()
//! # {
//! # xwin::core::init(|| {
//! let time = xwin::utility::time();
//! # }).expect("Failed to initialize XWin");
//! # }
//! ```
//!
//! ## Putting it Together
//! Now that you know how to initialize XWin, create a window and poll for
//! keyboard input, it's possible to create a small program.
//!
//! This program creates a 640x480 windowed mode window and starts a loop that
//! waits for the user to press the escape key, at which point it closes the
//! window and terminates.
//!
//! ```
//! use std::sync::mpsc;
//!
//! use xwin::{
//! 	input::keyboard::Key,
//! 	window::Window,
//! };
//!
//! fn main()
//! {
//! 	xwin::core::init(|| {
//! 		let mut win =
//! 			Window::try_new(640, 480, "Sample", None).expect("Failed to create window");
//! 		let (tx, rx) = mpsc::channel();
//! 		win.set_key_channel(tx).expect("Unable to set key channel");
//!
//! 		while !win.should_close()
//! 		{
//! 			# win.set_should_close(true);
//! 			# continue;
//! 			if let Ok(evt) = rx.recv()
//! 			{
//! 				match evt.key()
//! 				{
//! 					| Key::Escape => win.set_should_close(true),
//! 					| _ =>
//! 					{},
//! 				}
//! 			}
//! 		}
//! 	})
//! 	.expect("Failed to initialize XWin");
//! }
//! ```
//!
//! This tutorial used only a few of the many functions XWin provides. There is
//! additional documentation in other module covering more specific areas of
//! XWin.
//! - [core]
//! - [window]
//! - [monitor]
//! - [input]
//!
//! # Crate Features
//! XWin provides a number of optional features which may be enabled to expose
//! additional functions or add additional functionality. For more information
//! on raw GLFW bindings, see the [GLFW documentation](https://www.glfw.org/docs/latest/index.html).
//!
//! By default, no features are enabled. It is likely you will need at least one
//! of these depending on how you intend to render graphics.
//!
//! The features available are as follows:
//! - `tracing`. Adds the `tracing` crate as a dependency to XWin. XWin will
//!   then use tracing's `warn!` macro to report errors generated by GLFW
//!   functions.
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

// TODO - link rendering library in docs

pub mod core;
pub(crate) mod crate_util;
pub mod error;
pub mod event;
pub mod input;
pub mod monitor;
pub mod utility;
pub mod window;

pub mod bind;
pub mod vulkan;
