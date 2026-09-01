use std::{
	ffi::CString,
	sync::mpsc::channel,
};

use crate::{
	bind::{
		GLFW_ALPHA_BITS,
		GLFW_ANY_POSITION,
		GLFW_ANY_RELEASE_BEHAVIOR,
		GLFW_AUTO_ICONIFY,
		GLFW_BLUE_BITS,
		GLFW_CENTER_CURSOR,
		GLFW_CLIENT_API,
		GLFW_COCOA_FRAME_NAME,
		GLFW_COCOA_GRAPHICS_SWITCHING,
		GLFW_CONTEXT_CREATION_API,
		GLFW_CONTEXT_DEBUG,
		GLFW_CONTEXT_NO_ERROR,
		GLFW_CONTEXT_RELEASE_BEHAVIOR,
		GLFW_CONTEXT_ROBUSTNESS,
		GLFW_CONTEXT_VERSION_MAJOR,
		GLFW_CONTEXT_VERSION_MINOR,
		GLFW_DECORATED,
		GLFW_DEPTH_BITS,
		GLFW_DONT_CARE,
		GLFW_DOUBLEBUFFER,
		GLFW_EGL_CONTEXT_API,
		GLFW_FALSE,
		GLFW_FLOATING,
		GLFW_FOCUS_ON_SHOW,
		GLFW_FOCUSED,
		GLFW_GREEN_BITS,
		GLFW_LOSE_CONTEXT_ON_RESET,
		GLFW_MAXIMIZED,
		GLFW_MOUSE_PASSTHROUGH,
		GLFW_NATIVE_CONTEXT_API,
		GLFW_NO_API,
		GLFW_NO_RESET_NOTIFICATION,
		GLFW_NO_ROBUSTNESS,
		GLFW_OPENGL_ANY_PROFILE,
		GLFW_OPENGL_API,
		GLFW_OPENGL_COMPAT_PROFILE,
		GLFW_OPENGL_CORE_PROFILE,
		GLFW_OPENGL_ES_API,
		GLFW_OPENGL_FORWARD_COMPAT,
		GLFW_OPENGL_PROFILE,
		GLFW_OSMESA_CONTEXT_API,
		GLFW_POSITION_X,
		GLFW_POSITION_Y,
		GLFW_RED_BITS,
		GLFW_REFRESH_RATE,
		GLFW_RELEASE_BEHAVIOR_FLUSH,
		GLFW_RELEASE_BEHAVIOR_NONE,
		GLFW_RESIZABLE,
		GLFW_SAMPLES,
		GLFW_SCALE_FRAMEBUFFER,
		GLFW_SCALE_TO_MONITOR,
		GLFW_SRGB_CAPABLE,
		GLFW_STENCIL_BITS,
		GLFW_STEREO,
		GLFW_TRANSPARENT_FRAMEBUFFER,
		GLFW_TRUE,
		GLFW_VISIBLE,
		GLFW_WAYLAND_APP_ID,
		GLFW_WIN32_KEYBOARD_MENU,
		GLFW_WIN32_SHOWDEFAULT,
		GLFW_X11_CLASS_NAME,
		GLFW_X11_INSTANCE_NAME,
		glfwDefaultWindowHints,
		glfwWindowHint,
		glfwWindowHintString,
	},
	core::{
		XWin,
		exec::XWinMessage,
	},
	err::XErr,
	monitor::Monitor,
	window::{
		Pixels,
		Window,
	},
};

/// Client APIs for window creation.
pub enum ClientApi
{
	OpenGl,
	OpenGlEs,
	None,
}

/// Context creation APIs for window creation.
pub enum ContextCreationApi
{
	Native,
	Egl,
	Osmesa,
}

/// OpenGL profile for window context.
pub enum GlProfile
{
	Core,
	Compat,
	Any,
}

/// Robustness strategy for window context.
pub enum Robustness
{
	NoResetNotification,
	LoseContextOnReset,
	None,
}

/// Release behavior used by window context.
pub enum ContextReleaseBehavior
{
	Any,
	Flush,
	None,
}

/// Used to construct a window with some number of window creation hints.
///
/// # Window Creation Hint
/// There are a number of hints that can be set before the creation of a window
/// and context. Some affect the window itself, others affect the framebuffer or
/// context. These hints are set to their default values each time the library
/// is initialized.
///
/// Some hints are platform specific. These are always valid to set on any
/// platform, but they will only affect their specific platform. Other platforms
/// will ignore them. Setting these hints requires no platform specific calls.
///
/// ## Note
/// Window hints must be set before the creation of the window. After a
/// window has been created, there will be a more limited set of attributes that
/// may be modified
///
/// # Hard and Soft Constraints
/// Some window hints are hard constraints. These must match the available
/// capabilities *exactly* for window and context creation to succeed. Hints
/// that are not hard constraints are matched as closely as possible, but the
/// resulting context and framebuffer may differ from what these hints
/// requested.
///
/// The following hints are always hard constraints:
/// - [WindowBuilder::stereo]
/// - [WindowBuilder::double_buffer]
#[derive(Clone, Debug)]
pub struct WindowBuilder
{
	hints:    Vec<IntegerHint>,
	strings:  Vec<StringHint>,
	position: Pixels,
}
#[derive(Clone, Debug)]
struct IntegerHint(u32, i32);
#[derive(Clone, Debug)]
struct StringHint(u32, String);

impl WindowBuilder
{
	/// Construct and return a new WindowBuilder
	pub fn new() -> WindowBuilder
	{
		WindowBuilder {
			hints:    Vec::new(),
			strings:  Vec::new(),
			position: Pixels {
				x: GLFW_ANY_POSITION as i32,
				y: GLFW_ANY_POSITION as i32,
			},
		}
	}

	/// This function creates a window and its associated context with the hints
	/// set in this [WindowBuilder]. See [Window::create] for a more complete
	/// description.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized], [XErr::InvalidEnum],
	/// [XErr::InvalidValue], [XErr::ApiUnavailable],
	/// [XErr::VersionUnavailable], [XErr::FormatUnavailable],
	/// [XErr::NoWindowContext], [XErr::Platform].
	pub fn create(
		&self,
		width: i32,
		height: i32,
		title: &str,
		monitor: Option<Monitor>,
	) -> Result<Window, XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?
			.post_rcv(
				XWinMessage::CreateWindow {
					width,
					height,
					title: String::from(title),
					monitor,
					builder: Some(self.clone()),
					tx,
				},
				rx,
			)?
			.map(|win| Window::from_glfw(win))
	}

	/// Specifies whether the windowed mode window will be resizable by the
	/// user. The window will still be resizable using the [Window::resize]
	/// function. This hint is ignored for full screen and undecorated windows.
	///
	/// **Default:** `true`
	pub fn resizable(&mut self, value: bool) -> &mut Self
	{
		self.hint(GLFW_RESIZABLE, value)
	}

	/// Specifies whether the windowed mode window will be initially visible.
	/// This hint is ignored for full screen windows.
	///
	/// **Default:** `true`
	pub fn visible(&mut self, value: bool) -> &mut Self
	{
		self.hint(GLFW_VISIBLE, value)
	}

	/// Specifies whether the windowed mode window will have window decorations
	/// such as a border, a close widget, etc. An undecorated window will not be
	/// resizable by the user but will still allow the user to generate close
	/// events on some platforms. This hint is ignored for full screen windows.
	///
	/// **Default:** `true`
	pub fn decorated(&mut self, value: bool) -> &mut Self
	{
		self.hint(GLFW_DECORATED, value)
	}

	/// Specifies whether the windowed mode window will be given input focus
	/// when created. This hint is ignored for full screen and initially hidden
	/// windows.
	///
	/// **Default:** `true`
	pub fn focused(&mut self, value: bool) -> &mut Self
	{
		self.hint(GLFW_FOCUSED, value)
	}

	/// Specifies whether the full screen window will automatically iconify and
	/// restore the previous video mode on input focus loss. This hint is
	/// ignored for windowed mode windows.
	///
	/// **Default:** `true`
	pub fn auto_iconify(&mut self, value: bool) -> &mut Self
	{
		self.hint(GLFW_AUTO_ICONIFY, value)
	}

	/// Specifies whether the windowed mode window will be floating above other
	/// regular windows, also called topmost or always-on-top. This is intended
	/// primarily for debugging purposes and cannot be used to implement proper
	/// full screen windows. This hint is ignored for full screen windows.
	///
	/// **Default:** `false`
	pub fn floating(&mut self, value: bool) -> &mut Self
	{
		self.hint(GLFW_FLOATING, value)
	}

	/// Specifies whether the windowed mode window will be maximized when
	/// created. This hint is ignored for full screen windows.
	///
	/// **Default:** `false`
	pub fn maximized(&mut self, value: bool) -> &mut Self
	{
		self.hint(GLFW_MAXIMIZED, value)
	}

	/// Specifies whether the cursor should be centered over newly created full
	/// screen windows. This hint is ignored for windowed mode windows.
	///
	/// **Default:** `true`
	pub fn center_cursor(&mut self, value: bool) -> &mut Self
	{
		self.hint(GLFW_CENTER_CURSOR, value)
	}

	/// Specifies whether the window framebuffer will be transparent. If enabled
	/// and supported by the system, the window framebuffer alpha channel will
	/// be used to combine the framebuffer with the background. This does not
	/// affect window decorations.
	///
	/// **Default:** `false`
	pub fn transparent(&mut self, value: bool) -> &mut Self
	{
		self.hint(GLFW_TRANSPARENT_FRAMEBUFFER, value)
	}

	/// Specifies whether the window will be given input focus when
	/// [Window::show] is called.
	///
	/// **Default:** `true`
	pub fn focus_on_show(&mut self, value: bool) -> &mut Self
	{
		self.hint(GLFW_FOCUS_ON_SHOW, value)
	}

	/// Specifies whether the window content area should be resized based on
	/// content scale changes. This can be because of a global user settings
	/// change or because the window was moved to a monitor with different scale
	/// settings.
	///
	/// This hint only has an effect on platforms where screen coordinates and
	/// pixels always map 1:1, such as Windows and X11. On platforms like macOS
	/// the resolution of the framebuffer can change independently of the window
	/// size.
	///
	/// **Default:** `false`
	pub fn scale_to_monitor(&mut self, value: bool) -> &mut Self
	{
		self.hint(GLFW_SCALE_TO_MONITOR, value)
	}

	/// Specifies whether the framebuffer should be resized based on content
	/// scale changes. This can be because of a global user settings change or
	/// because the window was moved to a monitor with different scale settings.
	///
	/// This hint only has an effect on platforms where screen coordinates can
	/// be scaled relative to pixel coordinates, such as macOS and Wayland. On
	/// platforms like Windows and X11 the framebuffer and window content area
	/// sizes always map 1:1.
	///
	/// **Default:** `true`
	pub fn scale_framebuffer(&mut self, value: bool) -> &mut Self
	{
		self.hint(GLFW_SCALE_FRAMEBUFFER, value)
	}

	/// Specifies whether the window is transparent to mouse input, letting any
	/// mouse events pass through to whatever window is behind it. This is only
	/// supported for undecorated windows. Decorated windows with this enabled
	/// will behave differently between platforms.
	///
	/// **Default:** `false`
	pub fn mouse_passthrough(&mut self, value: bool) -> &mut Self
	{
		self.hint(GLFW_MOUSE_PASSTHROUGH, value)
	}

	/// Specify the desired initial position of the window. The window manager
	/// may modify of ignore these coordinates. If either or both of these
	/// values are `None` then the window manager will position the window where
	/// it thinks the user will prefer it.
	///
	/// **Default:** `None`, `None`
	pub fn position(&mut self, x: Option<i32>, y: Option<i32>) -> &mut Self
	{
		self.position = Pixels {
			x: if let Some(v) = x
			{
				v
			}
			else
			{
				GLFW_ANY_POSITION as i32
			},
			y: if let Some(v) = y
			{
				v
			}
			else
			{
				GLFW_ANY_POSITION as i32
			},
		};
		self
	}

	/// Specify the desired bit depth of the red, green, and blue components of
	/// the default framebuffer. A value of `None` means the application has no
	/// preference.
	///
	/// **Default:** `8`, `8`, `8`
	pub fn rgb_bits(&mut self, red: Option<i32>, green: Option<i32>, blue: Option<i32>)
	-> &mut Self
	{
		self.hints.push(IntegerHint(
			GLFW_RED_BITS,
			match red
			{
				| Some(v) => v,
				| None => GLFW_DONT_CARE,
			},
		));
		self.hints.push(IntegerHint(
			GLFW_GREEN_BITS,
			match green
			{
				| Some(v) => v,
				| None => GLFW_DONT_CARE,
			},
		));
		self.hints.push(IntegerHint(
			GLFW_BLUE_BITS,
			match blue
			{
				| Some(v) => v,
				| None => GLFW_DONT_CARE,
			},
		));
		self
	}

	/// Specify the desired bit depth of the alpha component of the default
	/// framebuffer. A value of `None` means the application has no preference.
	///
	/// **Default:** `8`
	pub fn alpha_bits(&mut self, value: Option<i32>) -> &mut Self
	{
		self.hints.push(IntegerHint(
			GLFW_ALPHA_BITS,
			match value
			{
				| Some(v) => v,
				| None => GLFW_DONT_CARE,
			},
		));
		self
	}

	/// Specify the desired bit depth of the depth component of the default
	/// framebuffer. A value of `None` means the application has no preference.
	///
	/// **Default:** `24`
	pub fn depth_bits(&mut self, value: Option<i32>) -> &mut Self
	{
		self.hints.push(IntegerHint(
			GLFW_DEPTH_BITS,
			match value
			{
				| Some(v) => v,
				| None => GLFW_DONT_CARE,
			},
		));
		self
	}

	/// Specify the desired bit depth of the stencil component of the default
	/// framebuffer. A value of `None` means the application has no preference.
	///
	/// **Default:** `8`
	pub fn stencil_bits(&mut self, value: Option<i32>) -> &mut Self
	{
		self.hints.push(IntegerHint(
			GLFW_STENCIL_BITS,
			match value
			{
				| Some(v) => v,
				| None => GLFW_DONT_CARE,
			},
		));
		self
	}

	/// Specifies whether to use OpenGL stereoscopic rendering. This is a hard
	/// constraint.
	///
	/// **Default:** `false`
	pub fn stereo(&mut self, value: bool) -> &mut Self
	{
		self.hint(GLFW_STEREO, value)
	}

	/// Specifies the desired number of samples to use for multisampling. Zero
	/// disables multisampling. A value of `None` means the application has no
	/// preference.
	///
	/// **Default:** `0`
	pub fn samples(&mut self, value: Option<i32>) -> &mut Self
	{
		self.hints.push(IntegerHint(
			GLFW_SAMPLES,
			if let Some(v) = value
			{
				v
			}
			else
			{
				GLFW_DONT_CARE
			},
		));
		self
	}

	/// Specifies whether the framebuffer should be sRGB capable.
	///
	/// **Default:** `false`
	pub fn srgb(&mut self, value: bool) -> &mut Self
	{
		self.hint(GLFW_SRGB_CAPABLE, value)
	}

	/// Specifies whether the framebuffer should be double buffered. You nearly
	/// always want to use double buffering. This is a hard constraint.
	///
	/// **Default:** `true`
	pub fn double_buffer(&mut self, value: bool) -> &mut Self
	{
		self.hint(GLFW_DOUBLEBUFFER, value)
	}

	/// Specifies the desired refresh rate for full screen windows. A value of
	/// `None` means the highest available refresh rate will be used.
	/// This hint is ignored for windowed mode windows.
	///
	/// **Default:** `None`
	pub fn refresh_rate(&mut self, value: Option<i32>) -> &mut Self
	{
		self.hints.push(IntegerHint(
			GLFW_REFRESH_RATE,
			if let Some(v) = value
			{
				v
			}
			else
			{
				GLFW_DONT_CARE
			},
		));
		self
	}

	/// Specifies which client API to create the context for. This is a hard
	/// constraint.
	///
	/// **Default:** [ClientApi::OpenGl]
	pub fn client_api(&mut self, value: ClientApi) -> &mut Self
	{
		self.hints.push(IntegerHint(
			GLFW_CLIENT_API,
			match value
			{
				| ClientApi::OpenGl => GLFW_OPENGL_API as i32,
				| ClientApi::OpenGlEs => GLFW_OPENGL_ES_API as i32,
				| ClientApi::None => GLFW_NO_API as i32,
			},
		));
		self
	}

	/// Specifies which context creation API to use to create the context. This
	/// is a hard constraint. If no client API is requested, this hint is
	/// ignored.
	///
	/// An extension loader library that assumes it knows which API was used to
	/// create the current context may fail if you change this hint.
	///
	/// **Wayland**.
	/// - The EGL API *is* the native context creation API, so this hint will
	///   have no effect.
	///
	/// **X11**.
	/// - On some linux systems, creating contexts via both the native and EGL
	///   APIs in a single process will cause the application to segfault. Stick
	///   to one API or the other on Linux for now.
	///
	/// **OSMesa**.
	/// - As its name implies, and OpenGL context created with OSMesa does not
	///   update the window contents when its buffers are swapped. Use OpenGL
	///   functions to retrieve the framebuffer contents.
	///
	/// **Default:** [ContextCreationApi::Native]
	pub fn context_creation_api(&mut self, value: ContextCreationApi) -> &mut Self
	{
		self.hints.push(IntegerHint(
			GLFW_CONTEXT_CREATION_API,
			match value
			{
				| ContextCreationApi::Native => GLFW_NATIVE_CONTEXT_API as i32,
				| ContextCreationApi::Egl => GLFW_EGL_CONTEXT_API as i32,
				| ContextCreationApi::Osmesa => GLFW_OSMESA_CONTEXT_API as i32,
			},
		));
		self
	}

	/// Specify the client API version that the created context must be
	/// compatible with. The exact behavior of this hint depends on the
	/// requested client API.
	///
	/// While there is no way to ask the driver for a context of the highest
	/// supported version, XWin will attempt to provide this when you ask for a
	/// version 1.0 context, which is the default for this hint.
	///
	/// **Default:** `1`, `0`
	pub fn context_version(&mut self, major: i32, minor: i32) -> &mut Self
	{
		self.hints
			.push(IntegerHint(GLFW_CONTEXT_VERSION_MAJOR, major));
		self.hints
			.push(IntegerHint(GLFW_CONTEXT_VERSION_MINOR, minor));
		self
	}

	/// Specifies whether the OpenGL context should be forward-compatible, i.e.
	/// one where all functionality deprecated in the requested version of
	/// OpenGL is removed. This must only be used if the requested OpenGL
	/// version is 3.0 or above. If OpenGL ES is requested, this hint is
	/// ignored.
	///
	/// Forward-compatibility is described in detail in the OpenGL Reference
	/// Manual.
	///
	/// **Default:** `false`
	pub fn opengl_forward_compatible(&mut self, value: bool) -> &mut Self
	{
		self.hint(GLFW_OPENGL_FORWARD_COMPAT, value)
	}

	/// Specifies whether the context should be created in debug mode, which may
	/// provide additional error and diagnostic reporting functionality from the
	/// underlying native library used by XWin.
	///
	/// **Default:** `false`
	pub fn context_debug(&mut self, value: bool) -> &mut Self
	{
		self.hint(GLFW_CONTEXT_DEBUG, value)
	}

	/// Specifies which OpenGL profile to create the context for. If requesting
	/// OpenGL version below 3.2, [GlProfile::Any] must be used. If OpenGL ES is
	/// requested, this hint is ignored.
	///
	/// OpenGL profiles are described in detail in the OpenGL Reference Manual.
	///
	/// **Default:** [GlProfile::Any]
	pub fn opengl_profile(&mut self, value: GlProfile) -> &mut Self
	{
		self.hints.push(IntegerHint(
			GLFW_OPENGL_PROFILE,
			match value
			{
				| GlProfile::Core => GLFW_OPENGL_CORE_PROFILE as i32,
				| GlProfile::Compat => GLFW_OPENGL_COMPAT_PROFILE as i32,
				| GlProfile::Any => GLFW_OPENGL_ANY_PROFILE as i32,
			},
		));
		self
	}

	/// Specifies the robustness strategy to be used by the context.
	///
	/// **Default:** [Robustness::None]
	pub fn context_robustness(&mut self, value: Robustness) -> &mut Self
	{
		self.hints.push(IntegerHint(
			GLFW_CONTEXT_ROBUSTNESS,
			match value
			{
				| Robustness::NoResetNotification => GLFW_NO_RESET_NOTIFICATION as i32,
				| Robustness::LoseContextOnReset => GLFW_LOSE_CONTEXT_ON_RESET as i32,
				| Robustness::None => GLFW_NO_ROBUSTNESS as i32,
			},
		));
		self
	}

	/// Specifies the release behavior to be used by the context.
	///
	/// If the behavior is [ContextReleaseBehavior::Any], the default behavior
	/// of the context creation API will be used. If the behavior is
	/// [ContextReleaseBehavior::Flush], the pipeline will be flushed whenever
	/// the context is released from being the current one. If the behavior is
	/// [ContextReleaseBehavior::None], the pipeline will not be flushed on
	/// release.
	///
	/// Context release behaviors are described in detail by the
	/// GL_KHR_context_flush_control extension.
	///
	/// **Default:** [ContextReleaseBehavior::Any]
	pub fn context_release_behavior(&mut self, value: ContextReleaseBehavior) -> &mut Self
	{
		self.hints.push(IntegerHint(
			GLFW_CONTEXT_RELEASE_BEHAVIOR,
			match value
			{
				| ContextReleaseBehavior::Any => GLFW_ANY_RELEASE_BEHAVIOR as i32,
				| ContextReleaseBehavior::Flush => GLFW_RELEASE_BEHAVIOR_FLUSH as i32,
				| ContextReleaseBehavior::None => GLFW_RELEASE_BEHAVIOR_NONE as i32,
			},
		));
		self
	}

	/// Specifies whether errors should be generated by the context. If enabled,
	/// situations that would have generated errors instead cause underfined
	/// behavior.
	///
	/// The no error mode for OpenGL and OpenGL ES is described in detail by the
	/// GL_KHR_no_error extension.
	///
	/// **Default:** `false`
	pub fn context_no_error(&mut self, value: bool) -> &mut Self
	{
		self.hint(GLFW_CONTEXT_NO_ERROR, value)
	}

	/// Specifies whether to allow access to the window menu via the Alt+Space
	/// and Alt-and-then-Space keyboard shortcuts. This is ignored on other
	/// platforms.
	///
	/// **Default:** `false`
	pub fn win32_keyboard_menu(&mut self, value: bool) -> &mut Self
	{
		self.hint(GLFW_WIN32_KEYBOARD_MENU, value)
	}

	/// Specifies whether to show the window the way specified in the program's
	/// STARTUPINFO when it is shown for the first time. This is the same
	/// information as the Run option in the shortcut properties window. If this
	/// information was not specified when the program was started, XWin behaves
	/// as if this hint was set to false. This is ignored on other platforms.
	///
	/// **Default:** `false`
	pub fn win32_show_default(&mut self, value: bool) -> &mut Self
	{
		self.hint(GLFW_WIN32_SHOWDEFAULT, value)
	}

	/// Specifies the UTF-8 encoded name to use for autosaving the window frame,
	/// or if empty disables frame autosaving for the window. This is ignored on
	/// other platforms.
	///
	/// **Default:** `""`
	pub fn cocoa_frame_name(&mut self, value: &str) -> &mut Self
	{
		self.string(GLFW_COCOA_FRAME_NAME, value)
	}

	/// Specifies whether to include Automatic Graphics Switching, i.e. to allow
	/// the system to choose the integrated GPU for the OpenGL context and move
	/// it between GPUs if necessary or whether to force it to always run on
	/// the discrete GPU. This only affects systems with both integrated and
	/// discrete GPUs.
	///
	/// Simpler programs and tools may want to enable this to save power, while
	/// games and other applications performing advanced rendering will want to
	/// leave it disabled.
	///
	/// A bundled application that wishes to participate in Automatic Graphics
	/// Switching should also declare this in its `Info.plist` by setting the
	/// `NSSupportsAutomaticGraphicsSwitching` key to true.
	///
	/// **Default:** `false`
	pub fn cocoa_graphics_switching(&mut self, value: bool) -> &mut Self
	{
		self.hint(GLFW_COCOA_GRAPHICS_SWITCHING, value)
	}

	/// Specifies the Wayland app_id for a window, used by window managers to
	/// identify types of windows.
	///
	/// **Default:** `""`
	pub fn wayland_app_id(&mut self, value: &str) -> &mut Self
	{
		self.string(GLFW_WAYLAND_APP_ID, value)
	}

	/// Specifies the desired ASCII encoded class and instance parts of the
	/// ICCCM WM_CLASS window property. Both values need to be set to something
	/// other than an empty string for them to take effect.
	///
	/// **Default:** `""`, `""`
	pub fn x11_class_name(&mut self, class: &str, instance: &str) -> &mut Self
	{
		self.string(GLFW_X11_CLASS_NAME, class);
		self.string(GLFW_X11_INSTANCE_NAME, instance)
	}

	/// Applies the hints stored in this `WindowBuilder`.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized], [XErr::InvalidEnum] and
	/// [XErr::InvalidValue].
	///
	/// # Thread Safety
	/// This function must be called on the main thread.
	pub(crate) fn apply(&self) -> Result<(), XErr>
	{
		unsafe { glfwDefaultWindowHints() };
		XErr::result(|| ())?;
		unsafe { glfwWindowHint(GLFW_POSITION_X as i32, self.position.x) };
		XErr::result(|| ())?;
		unsafe { glfwWindowHint(GLFW_POSITION_Y as i32, self.position.y) };
		XErr::result(|| ())?;

		for hint in &self.hints
		{
			unsafe { glfwWindowHint(hint.0 as i32, hint.1) };
			XErr::result(|| ())?;
		}

		for hint in &self.strings
		{
			let str = CString::new(hint.1.as_str()).map_err(|_| {
				XErr::InvalidValue(String::from("String hint contains a null byte"))
			})?;
			unsafe { glfwWindowHintString(hint.0 as i32, str.as_ptr()) };
			XErr::result(|| ())?;
		}

		Ok(())
	}

	fn hint(&mut self, hint: u32, value: bool) -> &mut Self
	{
		self.hints.push(IntegerHint(
			hint,
			if value
			{
				GLFW_TRUE as i32
			}
			else
			{
				GLFW_FALSE as i32
			},
		));
		self
	}

	fn string(&mut self, hint: u32, value: &str) -> &mut Self
	{
		self.strings.push(StringHint(hint, String::from(value)));
		self
	}
}
