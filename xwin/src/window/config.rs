use std::sync::mpsc::channel;

use xch::Sender;

use crate::{
	bind::{
		GLFW_AUTO_ICONIFY,
		GLFW_DECORATED,
		GLFW_DONT_CARE,
		GLFW_FLOATING,
		GLFW_FOCUS_ON_SHOW,
		GLFW_RESIZABLE,
		GLFW_TRANSPARENT_FRAMEBUFFER,
	},
	core::{
		ContentScale,
		Pixels,
		ScreenCoordinates,
		XWin,
		exec::XWinMessage,
		image::Image,
	},
	error::XErr,
	window::{
		Window,
		WindowEvent,
		context::WindowContext,
	},
};

impl Window
{
	// =======================
	//     EVENT FUNCTIONS
	// =======================

	/// Sets the [Sender] that will be used to send window config events. See
	/// [WindowEvent] for the specific conditions under which each event is
	/// sent.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized].
	pub fn set_config_channel<T>(&mut self, tx: T) -> Result<(), XErr>
	where
		T: Sender<WindowEvent> + Send + Sync + 'static,
	{
		self.with_context(
			"Unable to set config channel when XWin is uninitialized",
			|ctx| ctx.set_cfg_tx(tx),
		)
	}

	/// Close the window config event channel. See
	/// [Window::set_config_channel].
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized].
	pub fn clear_config_channel(&mut self) -> Result<(), XErr>
	{
		self.with_context(
			"Unable to clear config channel when XWin is uninitialized",
			|ctx| ctx.remove_cfg_tx(),
		)
	}

	// =======================
	//     QUERY FUNCTIONS
	// =======================

	/// See [Window::try_content_scale].
	pub fn content_scale(&self) -> ContentScale
	{
		self.try_content_scale().unwrap_or_default()
	}

	/// See [Window::try_frame_size].
	pub fn frame_size(&self) -> (u32, u32, u32, u32)
	{
		self.try_frame_size().unwrap_or_default()
	}

	/// See [Window::try_framebuffer_size].
	pub fn framebuffer_size(&self) -> Pixels
	{
		self.try_framebuffer_size().unwrap_or_default()
	}

	/// See [Window::try_has_transparent_framebuffer].
	pub fn has_transparent_framebuffer(&self) -> bool
	{
		self.try_has_transparent_framebuffer().unwrap_or_default()
	}

	/// See [Window::try_is_decorated].
	pub fn is_decorated(&self) -> bool
	{
		self.try_is_decorated().unwrap_or_default()
	}

	/// See [Window::try_is_floating].
	pub fn is_floating(&self) -> bool
	{
		self.try_is_floating().unwrap_or_default()
	}

	/// See [Window::try_is_resizable].
	pub fn is_resizable(&self) -> bool
	{
		self.try_is_resizable().unwrap_or_default()
	}

	/// See [Window::try_opacity].
	pub fn opacity(&self) -> f32
	{
		self.try_opacity().unwrap_or_default()
	}

	/// See [Window::try_title].
	pub fn title(&self) -> String
	{
		self.try_title().unwrap_or_default()
	}

	/// See [Window::try_will_focus].
	pub fn will_focus(&self) -> bool
	{
		self.try_will_focus().unwrap_or_default()
	}

	/// See [Window::try_will_iconify].
	pub fn will_iconify(&self) -> bool
	{
		self.try_will_iconify().unwrap_or_default()
	}

	// =======================
	//    UPDATE FUNCTIONS
	// =======================

	/// See [Window::try_set_aspect_ratio].
	pub fn set_aspect_ratio(&mut self, ratio: Option<(i32, i32)>)
	{
		let _ = self.try_set_aspect_ratio(ratio);
	}

	/// See [Window::try_set_decorated].
	pub fn set_decorated(&mut self, value: bool)
	{
		let _ = self.try_set_decorated(value);
	}

	/// See [Window::try_set_floating].
	pub fn set_floating(&mut self, value: bool)
	{
		let _ = self.try_set_floating(value);
	}

	/// See [Window::try_set_icon].
	pub fn set_icon(&mut self, icons: Vec<Image>)
	{
		let _ = self.try_set_icon(icons);
	}

	/// See [Window::try_set_opacity].
	pub fn set_opacity(&mut self, opacity: f32)
	{
		let _ = self.try_set_opacity(opacity);
	}

	/// See [Window::try_set_resizable].
	pub fn set_resizable(&mut self, value: bool)
	{
		let _ = self.try_set_resizable(value);
	}

	/// See [Window::try_set_size_limits].
	pub fn set_size_limits(&mut self, min: ScreenCoordinates<i32>, max: ScreenCoordinates<i32>)
	{
		let _ = self.try_set_size_limits(min, max);
	}

	/// See [Window::try_set_title].
	pub fn set_title(&mut self, title: &str)
	{
		let _ = self.try_set_title(title);
	}

	/// See [Window::try_set_will_focus].
	pub fn set_will_focus(&mut self, value: bool)
	{
		let _ = self.try_set_will_focus(value);
	}

	/// See [Window::try_set_will_iconify].
	pub fn set_will_iconify(&mut self, value: bool)
	{
		let _ = self.try_set_will_iconify(value);
	}

	// =======================
	//   TRY QUERY FUNCTIONS
	// =======================

	/// Returns the [ContentScale] for the window. The content scale is the
	/// ratio between the current DPI and the platform's default DPI. This is
	/// especially important for text and any UI elements. If the pixel
	/// dimensions of your UI scaled by this look appropriate on your machine
	/// then it should appear at a reasonable size on other machines regardless
	/// of their DPI and scaling settings. This relies on the system DPI and
	/// scaling settings being somewhat correct.
	///
	/// On platforms where each monitors can have its own content scale, the
	/// window content scale will depend on which monitor the system considers
	/// the window to be on.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized] and [XErr::Platform].
	pub fn try_content_scale(&self) -> Result<ContentScale, XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?
			.read()
			.unwrap()
			.post_rcv(XWinMessage::GetWindowContentScale(self.0, tx), rx)?
	}

	/// Returns the size, in Screen Coordinates, of each edge of
	/// the frame of the specified window. This size includes the title bar, if
	/// the window has one. The size of the frame may vary depending on the
	/// window-related hints used to create it.
	///
	/// The values are returned as `(left, top, right, bottom)`.
	///
	/// Because this function retrieves the size of each window frame edge and
	/// not the offset along a particular coordinate axis, the retrieved values
	/// will always be zero or positive.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized] and [XErr::Platform].
	pub fn try_frame_size(&self) -> Result<(u32, u32, u32, u32), XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?
			.read()
			.unwrap()
			.post_rcv(XWinMessage::GetWindowFrameSize(self.0, tx), rx)?
	}

	/// Returns the size, in [Pixels], of the framebuffer of
	/// the window. If you wish to retrieve the size of the window in
	/// screen coordinates, see [Window::size].
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized], [XErr::Platform].
	pub fn try_framebuffer_size(&self) -> Result<Pixels, XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?
			.read()
			.unwrap()
			.post_rcv(XWinMessage::GetFrameBufferSize(self.0, tx), rx)?
	}

	/// Indicates whether the window has a transparent framebuffer, i.e. the
	/// window contents is composited with the background using the window
	/// framebuffer alpha channel.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized] and [XErr::Platform].
	pub fn try_has_transparent_framebuffer(&self) -> Result<bool, XErr>
	{
		self.attr(GLFW_TRANSPARENT_FRAMEBUFFER)
	}

	/// Indicates whether the window has decorations such as a border, a close
	/// widget, etc.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized] and [XErr::Platform].
	pub fn try_is_decorated(&self) -> Result<bool, XErr>
	{
		self.attr(GLFW_DECORATED)
	}

	/// Indicates whether the window is floating, also called topmost or
	/// always-on-top.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized] and [XErr::Platform].
	pub fn try_is_floating(&self) -> Result<bool, XErr>
	{
		self.attr(GLFW_FLOATING)
	}

	/// Indicates whether the window is resizable *by the user*.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized] and [XErr::Platform].
	pub fn try_is_resizable(&self) -> Result<bool, XErr>
	{
		self.attr(GLFW_RESIZABLE)
	}

	/// Returns the opacity of the window, including any decorations.
	///
	/// The opacity (or alpha) value is a positive finite number between zero
	/// and one, where zero is fully transparent and one is fully opaque. If the
	/// system does not support whole window transparency, this function always
	/// returns one.
	///
	/// The initial opacity value for newly created windows is one.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized] and [XErr::Platform].
	pub fn try_opacity(&self) -> Result<f32, XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?
			.read()
			.unwrap()
			.post_rcv(XWinMessage::GetWindowOpacity(self.0, tx), rx)?
	}

	/// This function returns the title of the window. This is the title set
	/// previously by [Window::try_new] or [Window::set_title].
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized].
	///
	/// # Remarks
	/// The returned title is currently a copy of the title last set by
	/// [Window::try_new] or [Window::set_title]. It does not include any
	/// additional text which may be appended by the platform or another
	/// program.
	pub fn try_title(&self) -> Result<String, XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?
			.read()
			.unwrap()
			.post_rcv(XWinMessage::GetWindowTitle(self.0, tx), rx)?
	}

	/// Indicates whether the window will be given input focus when
	/// [Window::show] is called.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized] and [XErr::Platform].
	pub fn try_will_focus(&self) -> Result<bool, XErr>
	{
		self.attr(GLFW_FOCUS_ON_SHOW)
	}

	/// Indicates whether the fullscreen window is iconified on focus loss, a
	/// close widget, etc.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized] and [XErr::Platform].
	pub fn try_will_iconify(&self) -> Result<bool, XErr>
	{
		self.attr(GLFW_AUTO_ICONIFY)
	}

	// =======================
	//  TRY UPDATE FUNCTIONS
	// =======================

	/// Sets the required aspect ratio of the content area of the window. If
	/// the window is full screen, the aspect ratio only takes effect once it
	/// is made windowed. If the window is not resizable, this function does
	/// nothing.
	///
	/// The aspect ratio is specified as `(numerator, denominator)` and both
	/// values must be greater than zero. For example, the common 16:9 aspect
	/// ratio is specified as `(16, 9)`.
	///
	/// If the given ratio is `None`, hen the aspect ratio limit is disabled.
	///
	/// The aspect ratio is applied immediately to a windowed mode window and
	/// may cause it to be resized.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized], [XErr::InvalidValue],
	/// and [XErr::Platform].
	///
	/// # Remarks
	/// - If you set size limits and an aspect ratio that conflict, the results
	///   are undefined.
	/// - **Wayland**: The aspect ratio will not be applied until the window is
	///   actually resized, either by the user or by the compositor.
	pub fn try_set_aspect_ratio(&mut self, ratio: Option<(i32, i32)>) -> Result<(), XErr>
	{
		let (tx, rx) = channel();
		let value = ratio.or(Some((GLFW_DONT_CARE, GLFW_DONT_CARE))).unwrap();
		XWin::get()?.read().unwrap().post_rcv(
			XWinMessage::SetWindowAspectRatio {
				window: self.0,
				numerator: value.0,
				denominator: value.1,
				tx,
			},
			rx,
		)?
	}

	/// Sets whether the window has decorations such as a border, a close
	/// widget, etc.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized] and [XErr::Platform].
	pub fn try_set_decorated(&mut self, value: bool) -> Result<(), XErr>
	{
		self.set_attr(GLFW_DECORATED, value)
	}

	/// Sets whether the window is floating, also called topmost or
	/// always-on-top.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized], [XErr::Platform], and
	/// [XErr::FeatureUnavailable].
	///
	/// # Remarks
	/// - **Wayland**: The floating window attribute is not supported. Calling
	///   this will return [XErr::FeatureUnavailable].
	pub fn try_set_floating(&mut self, value: bool) -> Result<(), XErr>
	{
		self.set_attr(GLFW_FLOATING, value)
	}

	/// This function sets the icon of the window. If passed an array
	/// of candidate images, those of or closest to the sizes desired by the
	/// system are selected. If no images are specified, the window reverts to
	/// its default icon.
	///
	/// The pixels are 32-bit, little-endian, non-premultiplied RGBA, i.e. eight
	/// bits per channel with the red channel first. They are arranged
	/// canonically as packed sequential rows, starting from the top-left
	/// corner.
	///
	/// The desired image sizes varies depending on platform and system
	/// settings. The selected images will be rescaled as needed. Good sizes
	/// include 16x16, 32x32 and 48x48.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized], [XErr::InvalidValue],
	/// [XErr::Platform], and [XErr::FeatureUnavailable].
	///
	/// # Remarks
	/// - **MacOS**: Regular windows do not have icons on macOS. This function
	///   will return [XErr::FeatureUnavailable]. The dock icon will be the same
	///   as the application bundle's icon. For more information on bundles, see
	///   the Bundle Programming Guide in the Mac Developer Library.
	/// - **Wayland**: There is no existing protocol to change an icon, the
	///   window will thus inherit the one defined in the application's desktop
	///   file. This function will return [XErr::FeatureUnavailable].
	pub fn try_set_icon(&mut self, icons: Vec<Image>) -> Result<(), XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?
			.read()
			.unwrap()
			.post_rcv(XWinMessage::SetWindowIcon(self.0, icons, tx), rx)?
	}

	/// Sets the opacity of the window, including any decorations.
	///
	/// The opacity (or alpha) value is a positive finite number between zero
	/// and one, where zero is fully transparent and one is fully opaque.
	///
	/// The initial opacity value for newly created windows is one.
	///
	/// A window created with framebuffer transparency may not use whole window
	/// transparency. The results of doing this are undefined.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized], [XErr::Platform] and
	/// [XErr::FeatureUnavailable].
	///
	/// # Remarks
	/// - **Wayland**: There is no way to set an opacity factor for a window.
	///   This function will return [XErr::FeatureUnavailable].
	pub fn try_set_opacity(&mut self, opacity: f32) -> Result<(), XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?
			.read()
			.unwrap()
			.post_rcv(XWinMessage::SetWindowOpacity(self.0, opacity, tx), rx)?
	}

	/// Sets whether the window is resizable *by the user*.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized] and [XErr::Platform].
	pub fn try_set_resizable(&mut self, value: bool) -> Result<(), XErr>
	{
		self.set_attr(GLFW_RESIZABLE, value)
	}

	/// Sets the size limits of the content area of this. If this
	/// window is full screen, the size limits only take effect once it is made
	/// windowed. If the window is not resizable, this function does nothing.
	///
	/// The size limits are applied immediately to a windowed mode window and
	/// may cause it to be resized.
	///
	/// The maximum dimensions must be greater than or equal to the minimum
	/// dimensions and must be greater than or equal to zero.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized], [XErr::InvalidValue],
	/// and [XErr::Platform]
	///
	/// # Remarks
	/// - If you set size limits and an aspect ratio that conflict, the results
	///   are undefined.
	/// - **Wayland**: The size limits will not be applied until the window is
	///   actually resized, either by the user or by the compositor.
	pub fn try_set_size_limits(
		&mut self,
		min: ScreenCoordinates<i32>,
		max: ScreenCoordinates<i32>,
	) -> Result<(), XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?.read().unwrap().post_rcv(
			XWinMessage::SetWindowSizeLimits {
				window: self.0,
				min,
				max,
				tx,
			},
			rx,
		)?
	}

	/// This function sets the title of the window.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized], [XErr::Platform],
	/// and [XErr::InvalidValue].
	///
	/// # Remarks
	/// - **MacOS**: The window title will not be updated until the next time
	///   you process events.
	pub fn try_set_title(&mut self, title: &str) -> Result<(), XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?.read().unwrap().post_rcv(
			XWinMessage::SetWindowTitle(self.0, String::from(title), tx),
			rx,
		)?
	}

	/// Sets whether the window will be given input focus when [Window::show] is
	/// called.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized] and [XErr::Platform].
	pub fn try_set_will_focus(&mut self, value: bool) -> Result<(), XErr>
	{
		self.set_attr(GLFW_FOCUS_ON_SHOW, value)
	}

	/// Sets whether the full screen window is iconified on focus loss, a close
	/// widget, etc.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized] and [XErr::Platform].
	pub fn try_set_will_iconify(&mut self, value: bool) -> Result<(), XErr>
	{
		self.set_attr(GLFW_AUTO_ICONIFY, value)
	}
}
