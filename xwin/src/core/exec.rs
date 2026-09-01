use std::{
	ffi::{
		CStr,
		CString,
	},
	ptr::null_mut,
	sync::mpsc::{
		Receiver,
		Sender,
	},
};

use crate::{
	bind::{
		GLFW_CLIENT_API,
		GLFW_NO_API,
		GLFWimage,
		GLFWmonitor,
		GLFWwindow,
		glfwCreateWindow,
		glfwDefaultWindowHints,
		glfwDestroyWindow,
		glfwFocusWindow,
		glfwGetFramebufferSize,
		glfwGetGammaRamp,
		glfwGetMonitorContentScale,
		glfwGetMonitorName,
		glfwGetMonitorPhysicalSize,
		glfwGetMonitorPos,
		glfwGetMonitorWorkarea,
		glfwGetMonitors,
		glfwGetPrimaryMonitor,
		glfwGetVideoMode,
		glfwGetVideoModes,
		glfwGetWindowContentScale,
		glfwGetWindowFrameSize,
		glfwGetWindowOpacity,
		glfwGetWindowPos,
		glfwGetWindowSize,
		glfwGetWindowTitle,
		glfwHideWindow,
		glfwIconifyWindow,
		glfwMaximizeWindow,
		glfwRequestWindowAttention,
		glfwRestoreWindow,
		glfwSetGamma,
		glfwSetGammaRamp,
		glfwSetWindowAspectRatio,
		glfwSetWindowIcon,
		glfwSetWindowOpacity,
		glfwSetWindowPos,
		glfwSetWindowSize,
		glfwSetWindowSizeLimits,
		glfwSetWindowTitle,
		glfwShowWindow,
		glfwWindowHint,
	},
	core::{
		ContentScale,
		ScreenCoordinates,
		XWin,
		image::Image,
	},
	err::XErr,
	monitor::{
		GammaRamp,
		Millimeters,
		Monitor,
		VideoMode,
		WorkArea,
	},
	window::WindowBuilder,
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
	GetFrameBufferSize(*mut GLFWwindow, Sender<Result<(i32, i32), XErr>>),
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
}
unsafe impl Send for XWinMessage {}

impl XWin
{
	/// Send an [XWinMessage] to the main thread. Message is received by
	/// [XWin::run].
	pub(crate) fn post(&self, msg: XWinMessage) -> Result<(), XErr>
	{
		self.tx.send(msg).or_else(|_| {
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
		rcv.recv()
			.map_err(|_| XErr::NotInitialized(String::from("XWin has not been initialized")))
	}

	/// Run the main loop of XWin. Will block until `rx.recv()` returns `Err` or
	/// an [XWinMessage::Terminate] message is received.
	pub(crate) fn run(rx: Receiver<XWinMessage>)
	{
		while let Ok(msg) = rx.recv()
		{
			match msg
			{
				// Core
				| XWinMessage::Terminate => break,

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
				| XWinMessage::SetWindowOpacity(win, opacity, tx) =>
				{
					set_window_opacity(win, opacity, tx)
				},
				| XWinMessage::IconifyWindow(win, tx) => iconify_window(win, tx),
				| XWinMessage::RestoreWindow(win, tx) => restore_window(win, tx),
				| XWinMessage::MaximizeWindow(win, tx) => maximize_window(win, tx),
				| XWinMessage::ShowWindow(win, tx) => show_window(win, tx),
				| XWinMessage::HideWindow(win, tx) => hide_window(win, tx),
				| XWinMessage::FocusWindow(win, tx) => focus_window(win, tx),
				| XWinMessage::RequestWindowAttention(win, tx) => request_window_attention(win, tx),
			};
		}
	}
}

fn request_window_attention(win: *mut GLFWwindow, tx: Sender<Result<(), XErr>>)
{
	unsafe { glfwRequestWindowAttention(win) };
	let _ = tx.send(XErr::result(|| ()));
}

fn focus_window(win: *mut GLFWwindow, tx: Sender<Result<(), XErr>>)
{
	unsafe { glfwFocusWindow(win) };
	let _ = tx.send(XErr::result(|| ()));
}

fn hide_window(win: *mut GLFWwindow, tx: Sender<Result<(), XErr>>)
{
	unsafe { glfwHideWindow(win) };
	let _ = tx.send(XErr::result(|| ()));
}

fn show_window(win: *mut GLFWwindow, tx: Sender<Result<(), XErr>>)
{
	unsafe { glfwShowWindow(win) };
	let _ = tx.send(XErr::result(|| ()));
}

fn maximize_window(win: *mut GLFWwindow, tx: Sender<Result<(), XErr>>)
{
	unsafe { glfwMaximizeWindow(win) };
	let _ = tx.send(XErr::result(|| ()));
}

fn restore_window(win: *mut GLFWwindow, tx: Sender<Result<(), XErr>>)
{
	unsafe { glfwRestoreWindow(win) };
	let _ = tx.send(XErr::result(|| ()));
}

fn iconify_window(win: *mut GLFWwindow, tx: Sender<Result<(), XErr>>)
{
	unsafe { glfwIconifyWindow(win) };
	let _ = tx.send(XErr::result(|| ()));
}

fn set_window_opacity(win: *mut GLFWwindow, opacity: f32, tx: Sender<Result<(), XErr>>)
{
	unsafe { glfwSetWindowOpacity(win, opacity) };
	let _ = tx.send(XErr::result(|| ()));
}

fn window_opacity(win: *mut GLFWwindow, tx: Sender<Result<f32, XErr>>)
{
	let opacity = unsafe { glfwGetWindowOpacity(win) };
	let _ = tx.send(XErr::result(|| opacity));
}

fn window_content_scale(win: *mut GLFWwindow, tx: Sender<Result<ContentScale, XErr>>)
{
	let mut xscale = 0.0f32;
	let mut yscale = 0.0f32;
	unsafe { glfwGetWindowContentScale(win, &mut xscale, &mut yscale) };
	let _ = tx.send(XErr::result(|| {
		ContentScale {
			x: xscale,
			y: yscale,
		}
	}));
}

fn window_frame_size(win: *mut GLFWwindow, tx: Sender<Result<(u32, u32, u32, u32), XErr>>)
{
	let mut left = 0i32;
	let mut top = 0i32;
	let mut right = 0i32;
	let mut bottom = 0i32;
	unsafe { glfwGetWindowFrameSize(win, &mut left, &mut top, &mut right, &mut bottom) };
	let _ = tx.send(XErr::result(|| {
		(left as u32, top as u32, right as u32, bottom as u32)
	}));
}

fn framebuffer_size(win: *mut GLFWwindow, tx: Sender<Result<(i32, i32), XErr>>)
{
	let mut width = 0i32;
	let mut height = 0i32;
	unsafe { glfwGetFramebufferSize(win, &mut width, &mut height) };
	let _ = tx.send(XErr::result(|| (width, height)));
}

fn set_window_size(win: *mut GLFWwindow, size: ScreenCoordinates, tx: Sender<Result<(), XErr>>)
{
	unsafe { glfwSetWindowSize(win, size.x, size.y) };
	let _ = tx.send(XErr::result(|| ()));
}

fn set_window_aspect_ratio(
	win: *mut GLFWwindow,
	numer: i32,
	denom: i32,
	tx: Sender<Result<(), XErr>>,
)
{
	unsafe { glfwSetWindowAspectRatio(win, numer, denom) };
	let _ = tx.send(XErr::result(|| ()));
}

fn set_window_size_limits(
	win: *mut GLFWwindow,
	min: ScreenCoordinates,
	max: ScreenCoordinates,
	tx: Sender<Result<(), XErr>>,
)
{
	unsafe { glfwSetWindowSizeLimits(win, min.x, min.y, max.x, max.y) };
	let _ = tx.send(XErr::result(|| ()));
}

fn window_size(win: *mut GLFWwindow, tx: Sender<Result<ScreenCoordinates, XErr>>)
{
	let mut size = ScreenCoordinates::default();
	unsafe { glfwGetWindowSize(win, &mut size.x, &mut size.y) };
	let _ = tx.send(XErr::result(|| size));
}

fn set_window_pos(win: *mut GLFWwindow, pos: ScreenCoordinates, tx: Sender<Result<(), XErr>>)
{
	unsafe { glfwSetWindowPos(win, pos.x, pos.y) };
	let _ = tx.send(XErr::result(|| ()));
}

fn window_pos(win: *mut GLFWwindow, tx: Sender<Result<ScreenCoordinates, XErr>>)
{
	let mut pos = ScreenCoordinates::default();
	unsafe { glfwGetWindowPos(win, &mut pos.x, &mut pos.y) };
	let _ = tx.send(XErr::result(|| pos));
}

fn set_window_icon(win: *mut GLFWwindow, icons: Vec<Image>, tx: Sender<Result<(), XErr>>)
{
	let glfw_icons: Vec<GLFWimage> = icons.iter().map(Image::as_glfw).collect();

	unsafe {
		glfwSetWindowIcon(
			win,
			glfw_icons.len() as i32,
			if glfw_icons.is_empty()
			{
				null_mut()
			}
			else
			{
				glfw_icons.as_ptr()
			},
		)
	};
	let _ = tx.send(XErr::result(|| ()));
}

fn set_window_title(win: *mut GLFWwindow, title: String, tx: Sender<Result<(), XErr>>)
{
	let str = CString::new(title)
		.map_err(|_| XErr::InvalidValue(String::from("Window title may not contain null bytes")));
	if let Err(err) = str
	{
		let _ = tx.send(Err(err));
		return;
	}
	let str = str.unwrap();

	unsafe { glfwSetWindowTitle(win, str.as_ptr()) };
	let _ = tx.send(XErr::result(|| ()));
}

fn window_title(win: *mut GLFWwindow, tx: Sender<Result<String, XErr>>)
{
	let title = unsafe { glfwGetWindowTitle(win) };
	let _ = tx.send(XErr::result(|| {
		unsafe { CStr::from_ptr(title) }
			.to_str()
			.unwrap_or_else(|_| "")
			.to_owned()
	}));
}

fn destroy_window(win: *mut GLFWwindow, tx: Sender<Result<(), XErr>>)
{
	unsafe { glfwDestroyWindow(win) };
	let _ = tx.send(XErr::result(|| ()));
}

fn check_err<T>(tx: &Sender<Result<T, XErr>>) -> bool
{
	if let Err(err) = XErr::result(|| ())
	{
		let _ = tx.send(Err(err));
		true
	}
	else
	{
		false
	}
}

fn create_window(
	width: i32,
	height: i32,
	title: String,
	monitor: Option<Monitor>,
	builder: Option<WindowBuilder>,
	tx: Sender<Result<*mut GLFWwindow, XErr>>,
)
{
	unsafe { glfwDefaultWindowHints() };
	if check_err(&tx)
	{
		return;
	}

	if let Some(bld) = builder
	{
		if let Err(err) = bld.apply()
		{
			let _ = tx.send(Err(err));
			return;
		}
	}

	unsafe { glfwWindowHint(GLFW_CLIENT_API as i32, GLFW_NO_API as i32) };
	if check_err(&tx)
	{
		return;
	}

	let str = CString::new(title)
		.map_err(|_| XErr::InvalidValue(String::from("Title contains a null byte")));
	if let Err(err) = str
	{
		let _ = tx.send(Err(err));
		return;
	}

	let str = str.unwrap();

	let win = unsafe {
		glfwCreateWindow(
			width,
			height,
			str.as_ptr(),
			match monitor
			{
				| Some(mon) => mon.get_glfw(),
				| None => null_mut(),
			},
			null_mut(),
		)
	};

	let _ = if win.is_null()
	{
		tx.send(Err(XErr::get()))
	}
	else
	{
		tx.send(Ok(win))
	};
}

fn set_gamma_ramp(mon: *mut GLFWmonitor, ramp: GammaRamp, tx: Sender<Result<(), XErr>>)
{
	let mut ramp = ramp;
	ramp.with_glfw(|ramp| unsafe { glfwSetGammaRamp(mon, ramp) });
	let _ = tx.send(XErr::result(|| ()));
}

fn gamma_ramp(mon: *mut GLFWmonitor, tx: Sender<Result<GammaRamp, XErr>>)
{
	let ramp = unsafe { glfwGetGammaRamp(mon) };
	let _ = tx.send(match unsafe { ramp.as_ref() }
	{
		| None => Err(XErr::get()),
		| Some(gr) => Ok(GammaRamp::from_glfw(gr)),
	});
}

fn set_gamma(mon: *mut GLFWmonitor, gamma: f32, tx: Sender<Result<(), XErr>>)
{
	unsafe { glfwSetGamma(mon, gamma) };
	let _ = tx.send(XErr::result(|| ()));
}

fn monitor_video_mode(mon: *mut GLFWmonitor, tx: Sender<Result<VideoMode, XErr>>)
{
	let vm_ptr = unsafe { glfwGetVideoMode(mon) };
	let _ = tx.send(match unsafe { vm_ptr.as_ref() }
	{
		| None => Err(XErr::get()),
		| Some(vm) => Ok(VideoMode::from_glfw(vm)),
	});
}

fn monitor_video_modes(mon: *mut GLFWmonitor, tx: Sender<Result<Vec<VideoMode>, XErr>>)
{
	let mut count = 0i32;
	let vms = unsafe { glfwGetVideoModes(mon, &mut count) };
	let _ = tx.send(
		if vms.is_null()
		{
			Err(XErr::get())
		}
		else
		{
			let mut vec = Vec::<VideoMode>::with_capacity(count as usize);
			for idx in 0..count as usize
			{
				vec.push(VideoMode::from_glfw(unsafe { &*vms.add(idx) }));
			}
			Ok(vec)
		},
	);
}

fn monitor_name(mon: *mut GLFWmonitor, tx: Sender<Result<String, XErr>>)
{
	let title = unsafe { glfwGetMonitorName(mon) };
	let _ = tx.send(
		if title.is_null()
		{
			Err(XErr::get())
		}
		else
		{
			Ok(unsafe { CStr::from_ptr(title) }
				.to_str()
				.unwrap_or_else(|_| "")
				.to_owned())
		},
	);
}

fn monitor_content_scale(mon: *mut GLFWmonitor, tx: Sender<Result<ContentScale, XErr>>)
{
	let mut xscale = 0.0f32;
	let mut yscale = 0.0f32;
	unsafe { glfwGetMonitorContentScale(mon, &mut xscale, &mut yscale) };
	let _ = tx.send(XErr::result(|| {
		ContentScale {
			x: xscale,
			y: yscale,
		}
	}));
}

fn monitor_physical_size(mon: *mut GLFWmonitor, tx: Sender<Result<Millimeters, XErr>>)
{
	let mut width = 0i32;
	let mut height = 0i32;
	unsafe { glfwGetMonitorPhysicalSize(mon, &mut width, &mut height) };
	let _ = tx.send(XErr::result(|| {
		Millimeters {
			x: width,
			y: height,
		}
	}));
}

fn monitor_work_area(mon: *mut GLFWmonitor, tx: Sender<Result<WorkArea, XErr>>)
{
	let mut area = WorkArea::default();
	unsafe {
		glfwGetMonitorWorkarea(
			mon,
			&mut area.pos.x,
			&mut area.pos.y,
			&mut area.size.x,
			&mut area.size.y,
		)
	};
	let _ = tx.send(XErr::result(|| area));
}

fn monitor_pos(mon: *mut GLFWmonitor, tx: Sender<Result<ScreenCoordinates, XErr>>)
{
	let mut pos = ScreenCoordinates::default();
	unsafe { glfwGetMonitorPos(mon, &mut pos.x, &mut pos.y) };
	let _ = tx.send(XErr::result(|| pos));
}

fn primary_monitor(tx: Sender<Result<*mut GLFWmonitor, XErr>>)
{
	let monitor = unsafe { glfwGetPrimaryMonitor() };
	let _ = tx.send(
		if monitor.is_null()
		{
			Err(XErr::get())
		}
		else
		{
			Ok(monitor)
		},
	);
}

fn monitors(tx: Sender<Result<Vec<*mut GLFWmonitor>, XErr>>)
{
	let mut count = 0i32;
	let monitors = unsafe { glfwGetMonitors(&mut count) };

	let _ = tx.send(
		if monitors.is_null()
		{
			Err(XErr::get())
		}
		else
		{
			let mut vec = Vec::with_capacity(count as usize);
			for idx in 0..count as usize
			{
				vec.push(unsafe { *monitors.add(idx) });
			}
			Ok(vec)
		},
	);
}
