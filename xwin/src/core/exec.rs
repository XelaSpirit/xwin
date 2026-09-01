mod input;
mod monitor;
mod window;

use std::sync::mpsc::{
	Receiver,
	Sender,
	TryRecvError,
};

use monitor::*;
use window::*;

use crate::{
	bind::{
		GLFWcursor,
		GLFWmonitor,
		GLFWwindow,
		glfwPostEmptyEvent,
		glfwWaitEvents,
	},
	core::{
		ContentScale,
		ScreenCoordinates,
		XWin,
		exec::input::{
			create_cursor,
			destroy_cursor,
			input_mode,
			raw_mouse_supported,
			set_input_mode,
		},
		image::Image,
	},
	error::XErr,
	input::mouse::CursorShape,
	monitor::{
		GammaRamp,
		Millimeters,
		Monitor,
		VideoMode,
		WorkArea,
	},
	window::{
		Pixels,
		WindowBuilder,
	},
};

/// Used internally by XWin for sending messages to the main thread, for GLFW
/// functions that must be called on that thread.
pub(crate) enum XWinMessage
{
	// Core
	Terminate,

	// Monitor
	GetMonitors(Sender<Result<Vec<*mut GLFWmonitor>, XErr>>),
	GetPrimaryMonitor(Sender<Result<*mut GLFWmonitor, XErr>>),
	GetMonitorPos(*mut GLFWmonitor, Sender<Result<ScreenCoordinates, XErr>>),
	GetMonitorWorkArea(*mut GLFWmonitor, Sender<Result<WorkArea, XErr>>),
	GetMonitorPhysicalSize(*mut GLFWmonitor, Sender<Result<Millimeters, XErr>>),
	GetMonitorContentScale(*mut GLFWmonitor, Sender<Result<ContentScale, XErr>>),
	GetMonitorName(*mut GLFWmonitor, Sender<Result<String, XErr>>),
	GetMonitorVideoModes(*mut GLFWmonitor, Sender<Result<Vec<VideoMode>, XErr>>),
	GetMonitorVideoMode(*mut GLFWmonitor, Sender<Result<VideoMode, XErr>>),
	SetGamma(*mut GLFWmonitor, f32, Sender<Result<(), XErr>>),
	GammaRamp(*mut GLFWmonitor, Sender<Result<GammaRamp, XErr>>),
	SetGammaRamp(*mut GLFWmonitor, GammaRamp, Sender<Result<(), XErr>>),

	// Window
	CreateWindow
	{
		width:   i32,
		height:  i32,
		title:   String,
		monitor: Option<Monitor>,
		builder: Option<WindowBuilder>,
		tx:      Sender<Result<*mut GLFWwindow, XErr>>,
	},
	DestroyWindow(*mut GLFWwindow, Sender<Result<(), XErr>>),
	GetWindowTitle(*mut GLFWwindow, Sender<Result<String, XErr>>),
	SetWindowTitle(*mut GLFWwindow, String, Sender<Result<(), XErr>>),
	SetWindowIcon(*mut GLFWwindow, Vec<Image>, Sender<Result<(), XErr>>),
	GetWindowPos(*mut GLFWwindow, Sender<Result<ScreenCoordinates, XErr>>),
	SetWindowPos(*mut GLFWwindow, ScreenCoordinates, Sender<Result<(), XErr>>),
	GetWindowSize(*mut GLFWwindow, Sender<Result<ScreenCoordinates, XErr>>),
	SetWindowSizeLimits
	{
		window: *mut GLFWwindow,
		min:    ScreenCoordinates,
		max:    ScreenCoordinates,
		tx:     Sender<Result<(), XErr>>,
	},
	SetWindowAspectRatio
	{
		window:      *mut GLFWwindow,
		numerator:   i32,
		denominator: i32,
		tx:          Sender<Result<(), XErr>>,
	},
	SetWindowSize(*mut GLFWwindow, ScreenCoordinates, Sender<Result<(), XErr>>),
	GetFrameBufferSize(*mut GLFWwindow, Sender<Result<Pixels, XErr>>),
	GetWindowFrameSize(*mut GLFWwindow, Sender<Result<(u32, u32, u32, u32), XErr>>),
	GetWindowContentScale(*mut GLFWwindow, Sender<Result<ContentScale, XErr>>),
	GetWindowOpacity(*mut GLFWwindow, Sender<Result<f32, XErr>>),
	SetWindowOpacity(*mut GLFWwindow, f32, Sender<Result<(), XErr>>),
	IconifyWindow(*mut GLFWwindow, Sender<Result<(), XErr>>),
	RestoreWindow(*mut GLFWwindow, Sender<Result<(), XErr>>),
	MaximizeWindow(*mut GLFWwindow, Sender<Result<(), XErr>>),
	ShowWindow(*mut GLFWwindow, Sender<Result<(), XErr>>),
	HideWindow(*mut GLFWwindow, Sender<Result<(), XErr>>),
	FocusWindow(*mut GLFWwindow, Sender<Result<(), XErr>>),
	RequestWindowAttention(*mut GLFWwindow, Sender<Result<(), XErr>>),
	GetWindowMonitor(*mut GLFWwindow, Sender<Result<Option<Monitor>, XErr>>),
	SetWindowFullscreen
	{
		window:       *mut GLFWwindow,
		monitor:      Monitor,
		size:         ScreenCoordinates,
		refresh_rate: i32,
		tx:           Sender<Result<(), XErr>>,
	},
	SetWindowWindowed
	{
		window:   *mut GLFWwindow,
		position: ScreenCoordinates,
		size:     ScreenCoordinates,
		tx:       Sender<Result<(), XErr>>,
	},
	GetWindowAttribute(*mut GLFWwindow, i32, Sender<Result<i32, XErr>>),
	SetWindowAttribute(*mut GLFWwindow, i32, i32, Sender<Result<(), XErr>>),

	// Input
	CreateCursor(CursorShape, Sender<Result<*mut GLFWcursor, XErr>>),
	DestroyCursor(*mut GLFWcursor, Sender<Result<(), XErr>>),
	GetInputMode(*mut GLFWwindow, i32, Sender<Result<i32, XErr>>),
	SetInputMode(*mut GLFWwindow, i32, i32, Sender<Result<(), XErr>>),
	RawMouseSupported(Sender<Result<bool, XErr>>),
}
unsafe impl Send for XWinMessage {}

impl XWin
{
	/// Send an [XWinMessage] to the main thread. Message is received by
	/// [XWin::run].
	pub(crate) fn post(&self, msg: XWinMessage) -> Result<(), XErr>
	{
		self.xwin_tx.send(msg).or_else(|_| {
			Err(XErr::NotInitialized(String::from(
				"XWin has not been initialized",
			)))
		})
	}

	/// Send an [XWinMessage] to the main thread, and wait for a response.
	/// Message is received by [XWin::run].
	pub(crate) fn post_rcv<T>(&self, msg: XWinMessage, rcv: Receiver<T>) -> Result<T, XErr>
	{
		self.post(msg)?;
		unsafe { glfwPostEmptyEvent() };
		rcv.recv()
			.map_err(|_| XErr::NotInitialized(String::from("XWin has not been initialized")))
	}
}

/// Run the main loop of XWin. Will block until `rx.recv()` returns `Err` or
/// an [XWinMessage::Terminate] message is received.
pub(crate) fn run(rx: Receiver<XWinMessage>)
{
	unsafe { glfwWaitEvents() };

	let rcv = rx.try_recv();
	match rcv
	{
		| Ok(msg) => handle_msg(msg),
		| Err(err) =>
		{
			match err
			{
				| TryRecvError::Disconnected => return,
				| TryRecvError::Empty => (),
			}
		},
	};
}

fn handle_msg(msg: XWinMessage)
{
	match msg
	{
		// Core
		| XWinMessage::Terminate => return,

		// Monitor
		| XWinMessage::GetMonitors(tx) => monitors(tx),
		| XWinMessage::GetPrimaryMonitor(tx) => primary_monitor(tx),
		| XWinMessage::GetMonitorPos(mon, tx) => monitor_pos(mon, tx),
		| XWinMessage::GetMonitorWorkArea(mon, tx) => monitor_work_area(mon, tx),
		| XWinMessage::GetMonitorPhysicalSize(mon, tx) => monitor_physical_size(mon, tx),
		| XWinMessage::GetMonitorContentScale(mon, tx) => monitor_content_scale(mon, tx),
		| XWinMessage::GetMonitorName(mon, tx) => monitor_name(mon, tx),
		| XWinMessage::GetMonitorVideoModes(mon, tx) => monitor_video_modes(mon, tx),
		| XWinMessage::GetMonitorVideoMode(mon, tx) => monitor_video_mode(mon, tx),
		| XWinMessage::SetGamma(mon, gamma, tx) => set_gamma(mon, gamma, tx),
		| XWinMessage::GammaRamp(mon, tx) => gamma_ramp(mon, tx),
		| XWinMessage::SetGammaRamp(mon, ramp, tx) => set_gamma_ramp(mon, ramp, tx),

		// Window
		| XWinMessage::CreateWindow {
			width,
			height,
			title,
			monitor,
			builder,
			tx,
		} => create_window(width, height, title, monitor, builder, tx),
		| XWinMessage::DestroyWindow(win, tx) => destroy_window(win, tx),
		| XWinMessage::GetWindowTitle(win, tx) => window_title(win, tx),
		| XWinMessage::SetWindowTitle(win, title, tx) => set_window_title(win, title, tx),
		| XWinMessage::SetWindowIcon(win, icons, tx) => set_window_icon(win, icons, tx),
		| XWinMessage::GetWindowPos(win, tx) => window_pos(win, tx),
		| XWinMessage::SetWindowPos(win, pos, tx) => set_window_pos(win, pos, tx),
		| XWinMessage::GetWindowSize(win, tx) => window_size(win, tx),
		| XWinMessage::SetWindowSizeLimits {
			window,
			min,
			max,
			tx,
		} => set_window_size_limits(window, min, max, tx),
		| XWinMessage::SetWindowAspectRatio {
			window,
			numerator,
			denominator,
			tx,
		} => set_window_aspect_ratio(window, numerator, denominator, tx),
		| XWinMessage::SetWindowSize(win, size, tx) => set_window_size(win, size, tx),
		| XWinMessage::GetFrameBufferSize(win, tx) => framebuffer_size(win, tx),
		| XWinMessage::GetWindowFrameSize(win, tx) => window_frame_size(win, tx),
		| XWinMessage::GetWindowContentScale(win, tx) => window_content_scale(win, tx),
		| XWinMessage::GetWindowOpacity(win, tx) => window_opacity(win, tx),
		| XWinMessage::SetWindowOpacity(win, opacity, tx) => set_window_opacity(win, opacity, tx),
		| XWinMessage::IconifyWindow(win, tx) => iconify_window(win, tx),
		| XWinMessage::RestoreWindow(win, tx) => restore_window(win, tx),
		| XWinMessage::MaximizeWindow(win, tx) => maximize_window(win, tx),
		| XWinMessage::ShowWindow(win, tx) => show_window(win, tx),
		| XWinMessage::HideWindow(win, tx) => hide_window(win, tx),
		| XWinMessage::FocusWindow(win, tx) => focus_window(win, tx),
		| XWinMessage::RequestWindowAttention(win, tx) => request_window_attention(win, tx),
		| XWinMessage::GetWindowMonitor(win, tx) => window_monitor(win, tx),
		| XWinMessage::SetWindowFullscreen {
			window,
			monitor,
			size,
			refresh_rate,
			tx,
		} =>
		{
			set_window_monitor(
				window,
				Some(monitor),
				ScreenCoordinates::default(),
				size,
				refresh_rate,
				tx,
			)
		},
		| XWinMessage::SetWindowWindowed {
			window,
			position,
			size,
			tx,
		} => set_window_monitor(window, None, position, size, 0, tx),
		| XWinMessage::GetWindowAttribute(win, attr, tx) => window_attribute(win, attr, tx),
		| XWinMessage::SetWindowAttribute(win, attr, value, tx) =>
		{
			set_window_attribute(win, attr, value, tx)
		},

		// Input
		| XWinMessage::CreateCursor(shape, tx) => create_cursor(shape, tx),
		| XWinMessage::DestroyCursor(cursor, tx) => destroy_cursor(cursor, tx),
		| XWinMessage::GetInputMode(window, mode, tx) => input_mode(window, mode, tx),
		| XWinMessage::SetInputMode(window, mode, value, tx) =>
		{
			set_input_mode(window, mode, value, tx)
		},
		| XWinMessage::RawMouseSupported(tx) => raw_mouse_supported(tx),
	};
}
