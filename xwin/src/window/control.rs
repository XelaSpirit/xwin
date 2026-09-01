use std::sync::mpsc::channel;

use crate::{
	bind::{
		GLFW_DONT_CARE,
		GLFW_FALSE,
		GLFW_FOCUSED,
		GLFW_HOVERED,
		GLFW_ICONIFIED,
		GLFW_MAXIMIZED,
		GLFW_TRUE,
		GLFW_VISIBLE,
		glfwSetWindowShouldClose,
		glfwWindowShouldClose,
	},
	core::{
		ScreenCoordinates,
		XWin,
		exec::XWinMessage,
	},
	error::XErr,
	monitor::Monitor,
	window::{
		Window,
		WindowBuilder,
	},
};

impl Window
{
	// =======================
	//     QUERY FUNCTIONS
	// =======================

	/// See [Window::try_is_focused].
	pub fn is_focused(&self) -> bool
	{
		self.try_is_focused().unwrap_or_default()
	}

	/// See [Window::try_is_hovered].
	pub fn is_hovered(&self) -> bool
	{
		self.try_is_hovered().unwrap_or_default()
	}

	/// See [Window::try_is_iconified]
	pub fn is_iconified(&self) -> bool
	{
		self.try_is_iconified().unwrap_or_default()
	}

	/// See [Window::try_is_maximized].
	pub fn is_maximized(&self) -> bool
	{
		self.try_is_maximized().unwrap_or_default()
	}

	/// See [Window::try_is_visible].
	pub fn is_visible(&self) -> bool
	{
		self.try_is_visible().unwrap_or_default()
	}

	/// See [Window::try_monitor].
	pub fn monitor(&self) -> Option<Monitor>
	{
		self.try_monitor().unwrap_or_default()
	}

	/// See [Window::try_position]
	pub fn position(&self) -> ScreenCoordinates<i32>
	{
		self.try_position().unwrap_or_default()
	}

	/// See [Window::try_should_close].
	pub fn should_close(&self) -> bool
	{
		self.try_should_close().unwrap_or_default()
	}

	/// See [Window::try_size].
	pub fn size(&self) -> ScreenCoordinates<i32>
	{
		self.try_size().unwrap_or_default()
	}

	// =======================
	//    UPDATE FUNCTIONS
	// =======================

	/// See [Window::try_focus].
	pub fn focus(&self)
	{
		self.try_focus().unwrap_or_default()
	}

	/// See [Window::try_hide].
	pub fn hide(&self)
	{
		self.try_hide().unwrap_or_default()
	}

	/// See [Window::try_iconify].
	pub fn iconify(&self)
	{
		self.try_iconify().unwrap_or_default()
	}

	/// See [Window::try_maximize].
	pub fn maximize(&self)
	{
		self.try_maximize().unwrap_or_default()
	}

	/// See [Window::try_request_attention].
	pub fn request_attention(&self)
	{
		self.try_request_attention().unwrap_or_default()
	}

	/// See [Window::try_restore].
	pub fn restore(&mut self)
	{
		self.try_restore().unwrap_or_default()
	}

	/// See [Window::try_set_fullscreen].
	pub fn set_fullscreen(
		&mut self,
		monitor: Monitor,
		size: ScreenCoordinates<i32>,
		refresh_hz: Option<i32>,
	)
	{
		let _ = self.try_set_fullscreen(monitor, size, refresh_hz);
	}

	/// See [Window::try_set_position].
	pub fn set_position(&mut self, position: ScreenCoordinates<i32>)
	{
		let _ = self.try_set_position(position);
	}

	/// See [Window::try_set_should_close].
	pub fn set_should_close(&mut self, value: bool)
	{
		let _ = self.try_set_should_close(value);
	}

	/// See [Window::try_set_size].
	pub fn set_size(&mut self, size: ScreenCoordinates<i32>)
	{
		let _ = self.try_set_size(size);
	}

	/// See [Window::try_set_windowed].
	pub fn set_windowed(&mut self, position: ScreenCoordinates<i32>, size: ScreenCoordinates<i32>)
	{
		let _ = self.try_set_windowed(position, size);
	}

	/// See [Window::try_show].
	pub fn show(&self)
	{
		self.try_show().unwrap_or_default()
	}

	// =======================
	//   TRY QUERY FUNCTIONS
	// =======================

	/// Indicates whether the window has input focus.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized] and [XErr::Platform].
	pub fn try_is_focused(&self) -> Result<bool, XErr>
	{
		self.attr(GLFW_FOCUSED)
	}

	/// Indicates whether the cursor is currently directly over the content area
	/// of the window, with no other windows between.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized] and [XErr::Platform].
	pub fn try_is_hovered(&self) -> Result<bool, XErr>
	{
		self.attr(GLFW_HOVERED)
	}

	/// Indicates whether the window is iconified.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized] and [XErr::Platform].
	///
	/// # Remarks
	/// - **Wayland**: The Wayland protocol provides no way to check whether a
	///   window is iconified, so this function always returns `false`.
	pub fn try_is_iconified(&self) -> Result<bool, XErr>
	{
		self.attr(GLFW_ICONIFIED)
	}

	/// Indicates whether the window is maximized.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized] and [XErr::Platform].
	pub fn try_is_maximized(&self) -> Result<bool, XErr>
	{
		self.attr(GLFW_MAXIMIZED)
	}

	/// Indicates whether the window is visible.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized] and [XErr::Platform].
	pub fn try_is_visible(&self) -> Result<bool, XErr>
	{
		self.attr(GLFW_VISIBLE)
	}

	/// Returns the [Monitor] that the window is full screen on, or `None` if
	/// the window is in windowed mode.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized].
	pub fn try_monitor(&self) -> Result<Option<Monitor>, XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?
			.read()
			.unwrap()
			.post_rcv(XWinMessage::GetWindowMonitor(self.0, tx), rx)?
	}

	/// This function retrieves the position, in [ScreenCoordinates], of the
	/// upper-left corner of the content area of the window.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized], [XErr::Platform], and
	/// [XErr::FeatureUnavailable].
	///
	/// # Remarks
	/// - **Wayland**: There is no way for an application to retrieve the global
	///   position of its windows. This function will return
	///   [XErr::FeatureUnavailable].
	pub fn try_position(&self) -> Result<ScreenCoordinates<i32>, XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?
			.read()
			.unwrap()
			.post_rcv(XWinMessage::GetWindowPos(self.0, tx), rx)?
	}

	/// Returns the value of the close flag of the window.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized].
	pub fn try_should_close(&self) -> Result<bool, XErr>
	{
		let close = unsafe { glfwWindowShouldClose(self.0) == GLFW_TRUE as i32 };
		XErr::result(|| close)
	}

	/// Returns the size, in [ScreenCoordinates], of the content area of this
	/// window. If you wish to get the size of the framebuffer of the window in
	/// pixels, see [Window::framebuffer_size].
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized] and [XErr::Platform].
	pub fn try_size(&self) -> Result<ScreenCoordinates<i32>, XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?
			.read()
			.unwrap()
			.post_rcv(XWinMessage::GetWindowSize(self.0, tx), rx)?
	}

	// =======================
	//  TRY UPDATE FUNCTIONS
	// =======================

	/// Brings the window to front and sets input focus. The window should
	/// already be visible and not iconified.
	///
	/// By default, both windowed and full screen mode windows are focused when
	/// initially created. Set the [WindowBuilder::focused] window hint to
	/// `false` to disable this behavior.
	///
	/// Also by default, windowed mode windows are focused when shown with
	/// [Window::show]. Set the [WindowBuilder::focus_on_show] window hint
	/// to `false` to disable this behavior.
	///
	/// **Do not use this function** to steal focus from other applications
	/// unless you are certain that is what the user wants. Focus stealing can
	/// be extremely disruptive.
	///
	/// For a less disruptive way of getting the user's attention, see
	/// [Window::request_attention].
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized] and [XErr::Platform].
	///
	/// # Remarks
	/// - **Wayland**: The compositor will likely ignore focus requests unless
	///   another window created by the same application already has input
	///   focus.
	pub fn try_focus(&self) -> Result<(), XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?
			.read()
			.unwrap()
			.post_rcv(XWinMessage::FocusWindow(self.0, tx), rx)?
	}

	/// Hides the window if it was previously visible. If the window is already
	/// hidden or is in full screen mode, this function does nothing.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized] and [XErr::Platform].
	pub fn try_hide(&self) -> Result<(), XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?
			.read()
			.unwrap()
			.post_rcv(XWinMessage::HideWindow(self.0, tx), rx)?
	}

	/// Iconifies (minimizes) the window if it was previously restored. If the
	/// window is already iconified, this function does nothing.
	///
	/// If the window is a full screen window, XWin restores the original video
	/// mode of the monitor. the window's desired video mode is set again when
	/// the window is restored.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized] and [XErr::Platform].
	///
	/// # Remarks
	/// - **Wayland**: Once a window is iconified, [Window::restore] won't be
	///   able to restore it. This is a design decision of the xdg-shell
	///   protocol.
	pub fn try_iconify(&self) -> Result<(), XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?
			.read()
			.unwrap()
			.post_rcv(XWinMessage::IconifyWindow(self.0, tx), rx)?
	}

	/// Maximizes the window if it was previously not maximized. If
	/// this is already maximized, this function does nothing.
	///
	/// If the window is a full screen window, this function does
	/// nothing.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized] and [XErr::Platform].
	pub fn try_maximize(&self) -> Result<(), XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?
			.read()
			.unwrap()
			.post_rcv(XWinMessage::MaximizeWindow(self.0, tx), rx)?
	}

	/// Requests user attention to the window. On platforms where this is not
	/// supported, attention is requested to the application as a whole.
	///
	/// Once the user has given attention, usually by focusing the window or
	/// application, the system will end the request automatically.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized] and [XErr::Platform].
	///
	/// # Remarks
	/// - **MacOS**: Attention is requested to the application as a whole not
	///   the specific window.
	pub fn try_request_attention(&self) -> Result<(), XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?
			.read()
			.unwrap()
			.post_rcv(XWinMessage::RequestWindowAttention(self.0, tx), rx)?
	}

	/// Restores this window if it was previously iconified (minimized)
	/// or maximized. If the window is already restored, this function does
	/// nothing.
	///
	/// If the window is an iconified full screen window, its desired
	/// video mode is set again for its monitor when this is restored.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized] and [XErr::Platform].
	pub fn try_restore(&mut self) -> Result<(), XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?
			.read()
			.unwrap()
			.post_rcv(XWinMessage::RestoreWindow(self.0, tx), rx)?
	}

	/// Sets the window to fullscreen mode on a given monitor.
	///
	/// This function updates the width, height, and refresh rate of the desired
	/// video mode and switches to the video mode closest to it.
	///
	/// If you only wish to update the resolution of a full screen window, see
	/// [Window::set_size].
	///
	/// # Parameters
	/// - `monitor`: The desired monitor for full screen mode.
	/// - `size`: The desired size, in [ScreenCoordinates], of the video mode.
	/// - `refresh_hz`: The desired rate, in Hz, of the video mode, or `None`
	///   for no preference.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized] and [XErr::Platform].
	pub fn try_set_fullscreen(
		&mut self,
		monitor: Monitor,
		size: ScreenCoordinates<i32>,
		refresh_hz: Option<i32>,
	) -> Result<(), XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?.read().unwrap().post_rcv(
			XWinMessage::SetWindowFullscreen {
				window: self.0,
				monitor,
				size,
				refresh_rate: match refresh_hz
				{
					| Some(rate) => rate,
					| None => GLFW_DONT_CARE,
				},
				tx,
			},
			rx,
		)?
	}

	/// This function sets the position, in [ScreenCoordinates], of the
	/// upper-left corner of the content area of the window. If the window is
	/// a full screen window, this function does nothing.
	///
	/// **Do not use this function** to move an already visible window unless
	/// you have very good reasons for doing so, as it will confuse and annoy
	/// the user.
	///
	/// The window manager may put limits on what positions are allowed. XWin
	/// cannot and should not override these limits.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized], [XErr::Platform],
	/// [XErr::FeatureUnavailable].
	///
	/// # Remarks
	/// - **Wayland**: There is no way for an application to set the global
	///   position of its windows. This function will return
	///   [XErr::FeatureUnavailable].
	pub fn try_set_position(&mut self, position: ScreenCoordinates<i32>) -> Result<(), XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?
			.read()
			.unwrap()
			.post_rcv(XWinMessage::SetWindowPos(self.0, position, tx), rx)?
	}

	/// Sets the value of the close flag of the window. This can be used to
	/// override the user's attempt to close the window, or to signal that it
	/// should be closed.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized].
	pub fn try_set_should_close(&mut self, value: bool) -> Result<(), XErr>
	{
		unsafe {
			glfwSetWindowShouldClose(
				self.0,
				if value
				{
					GLFW_TRUE as i32
				}
				else
				{
					GLFW_FALSE as i32
				},
			)
		};

		XErr::result(|| {})
	}

	/// Sets the size, in [ScreenCoordinates], of the content area
	/// of the window.
	///
	/// For full screen windows, this function updates the resolution of its
	/// desired video mode and switches to the video mode closest to it.
	///
	/// If you wish to update the refresh rate of the desired video mode in
	/// addition to its resolution, see [Window::try_set_fullscreen].
	///
	/// The window manager may put limits on what sizes are allowed. XWin cannot
	/// and should not override these limits.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized], [XErr::Platform].
	pub fn try_set_size(&mut self, size: ScreenCoordinates<i32>) -> Result<(), XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?
			.read()
			.unwrap()
			.post_rcv(XWinMessage::SetWindowSize(self.0, size, tx), rx)?
	}

	/// Sets the window to windowed mode.
	///
	/// `position` and `size` are used to place the window content area.
	///
	/// If you only wish to update the size of a windowed mode window, see
	/// [Window::set_size].
	///
	/// # Parameters
	/// - `position`: The desired [ScreenCoordinates] of the upper-left corner
	///   of the content area.
	/// - `size`: The desired size, in [ScreenCoordinates], of the content area.
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized] and [XErr::Platform].
	///
	/// # Remarks
	/// - **Wayland**: The desired window position is ignored, as there is no
	///   way for an application to set this property.
	pub fn try_set_windowed(
		&mut self,
		position: ScreenCoordinates<i32>,
		size: ScreenCoordinates<i32>,
	) -> Result<(), XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?.read().unwrap().post_rcv(
			XWinMessage::SetWindowWindowed {
				window: self.0,
				position,
				size,
				tx,
			},
			rx,
		)?
	}

	/// Makes the window visible if it was previously hidden. If this
	/// window is already visible or is in full screen mode, this function does
	/// nothing.
	///
	/// By default, windowed mode windows are focused when shown. Set the
	/// [WindowBuilder::focus_on_show] window hint to `true` to change this
	/// behavior a newly created window, or change the behavior for an
	/// existing window with [Window::set_will_focus].
	///
	/// # Errors
	/// Possible errors include [XErr::NotInitialized] and [XErr::Platform].
	///
	/// # Remarks
	/// - **Wayland**: Because Wayland wants every frame of the desktop to be
	///   complete, this function dow not immediately make the window visible.
	///   Instead, it will become visible the next time the window framebuffer
	///   is updated after this call.
	pub fn try_show(&self) -> Result<(), XErr>
	{
		let (tx, rx) = channel();
		XWin::get()?
			.read()
			.unwrap()
			.post_rcv(XWinMessage::ShowWindow(self.0, tx), rx)?
	}
}
