# XWin

`xwin` is a Rust-friendly wrapper around [GLFW](https://www.glfw.org/). It provides owned window and monitor types,
`Result`-based error handling, window builders, and channel-based input and window events while retaining optional
access to the underlying GLFW API.

The crate handles window creation, monitor information, keyboard, mouse and gamepad input, clipboard access, timers, and
the GLFW event loop. It does not perform rendering. Bring your own renderer, or enable the Vulkan or raw GLFW features
when integrating with a graphics library.

> **Status:** xwin is under active development. The current build script
> builds GLFW as a Windows DLL, so Windows is the supported target at present.

## Requirements

- A Rust toolchain with Rust 2024 edition support
- [CMake](https://cmake.org/) available on `PATH`
- A working C/C++ build environment for GLFW
- libclang available to [`bindgen`](https://rust-lang.github.io/rust-bindgen/requirements.html)
- The Vulkan SDK and `VULKAN_SDK_PATH` when using the `vulkan` feature

GLFW is included in the repository as a submodule and compiled automatically; it does not need to be installed
separately.

## Installation

Add `xwin` to your application's `Cargo.toml`:

```toml
[dependencies]
xwin = { git = "https://github.com/XelaSpirit/xwin.git", branch = "master" }
```

## Documentation

This README provides an overview and quick start. For more complete documentation of the available modules, types,
functions, and platform-specific behavior, generate and open the crate's Cargo documentation from the repository root:

```console
cargo doc -p xwin --all-features --open
```

Using `--all-features` includes the feature-gated GLFW and Vulkan APIs. Omit it to document only the default feature
set.

## Quick start

Call `xwin::core::init` from the main thread. xwin keeps that thread for GLFW's event processing and runs the supplied
closure on a worker thread.

```rust
use std::sync::mpsc;

use xwin::{
	input::keyboard::Key,
	window::Window,
};

fn main()
{
	xwin::core::init(|| {
		let mut win = Window::try_new(1920, 1080, "Sample", None).expect("Failed to create window");
		let (tx, rx) = mpsc::channel();
		win.set_key_channel(tx).expect("Unable to set key channel");

		while !win.should_close()
		{
			if let Ok(evt) = rx.recv()
			{
				match evt.key()
				{
					| Key::Escape => win.set_should_close(true),
					| _ =>
						{}
				}
			}
		}
	})
		.expect("Failed to initialize XWin");
}
```

Dropping a `Window` schedules its underlying GLFW window for destruction. Calling `xwin::core::terminate` terminates
xwin early and releases remaining resources.

## Window configuration

Use `Window::try_new` for the defaults or `WindowBuilder` to set creation hints:

```rust
use xwin::window::WindowBuilder;

fn main()
{
	xwin::core::init(|| {
		let window = WindowBuilder::new()
			.resizable(true)
			.decorated(true)
			.transparent(false)
			.try_create(800, 600, "Configured window", None);

		// ...
	})
		.expect("Failed to initialize XWin");
}

```

Pass a `Monitor` to `try_new` or `try_create` to create a full-screen window. After creation, `Window` exposes controls
for visibility, focus, position, size, full-screen mode, cursor behavior, and the close flag.

## Events

Events are delivered through channels attached to a window. Separate channels are available for keyboard, mouse,
character, file-drop, and window configuration events. Standard-library `mpsc` senders work out of the box; the
underlying `xela_channels` abstraction can also combine event types into a single channel.

Monitor connection events and joystick configuration events can be received through their corresponding global channels.

## Error handling

Fallible APIs generally use a `try_` prefix and return
`Result<_, xwin::error::XErr>`. Many also have a convenience form without the prefix that returns a default value if an
error occurs. Prefer the `try_`
variants when an operation's failure matters to your application.

## Cargo features

No features are enabled by default.

| Feature    | Purpose                                                                        |
|------------|--------------------------------------------------------------------------------|
| `tracing`  | Reports GLFW errors through the `tracing` crate.                               |
| `glfw`     | Exposes conversions to and from selected underlying GLFW types.                |
| `bindings` | Enables `glfw` and publicly exposes the raw bindings in `xwin::bind`.          |
| `vulkan`   | Enables `glfw` and exposes Vulkan integration helpers and Vulkan handle types. |

For example:

```toml
[dependencies]
xwin = { git = "https://github.com/XelaSpirit/xwin.git", branch = "master", features = ["vulkan", "tracing"] }
```

## Running the sample

From the repository root:

```console
cargo run -p sample_win
```

Press Escape or use the window's close control to exit.

## License

This project is licensed under the terms in [LICENSE](LICENSE). GLFW is distributed under its own license in
[`glfw/LICENSE.md`](https://github.com/glfw/glfw/blob/master/LICENSE.md).
