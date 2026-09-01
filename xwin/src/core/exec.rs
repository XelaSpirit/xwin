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
		glfwCreateWindow,
		glfwDefaultWindowHints,
		glfwDestroyWindow,
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
		glfwSetGamma,
		glfwSetGammaRamp,
	},
	core::{
		ContentScale,
		ScreenCoordinates,
		XWin,
	},
	err::XErr,
	monitor::{
		GammaRamp,
		Millimeters,
		Monitor,
		VideoMode,
		WorkArea,
	},
	window::{
		Window,
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
	GetMonitors(Sender<Result<Vec<Monitor>, XErr>>),
	GetPrimaryMonitor(Sender<Result<Monitor, XErr>>),
	GetMonitorPos(Monitor, Sender<Result<ScreenCoordinates, XErr>>),
	GetMonitorWorkArea(Monitor, Sender<Result<WorkArea, XErr>>),
	GetMonitorPhysicalSize(Monitor, Sender<Result<Millimeters, XErr>>),
	GetMonitorContentScale(Monitor, Sender<Result<ContentScale, XErr>>),
	GetMonitorName(Monitor, Sender<Result<String, XErr>>),
	GetMonitorVideoModes(Monitor, Sender<Result<Vec<VideoMode>, XErr>>),
	GetMonitorVideoMode(Monitor, Sender<Result<VideoMode, XErr>>),
	SetGamma(Monitor, f32, Sender<Result<(), XErr>>),
	GammaRamp(Monitor, Sender<Result<GammaRamp, XErr>>),
	SetGammaRamp(Monitor, GammaRamp, Sender<Result<(), XErr>>),

	// Window
	CreateWindow
	{
		width:   i32,
		height:  i32,
		title:   String,
		monitor: Option<Monitor>,
		share:   Option<Window>,
		builder: Option<WindowBuilder>,
		tx:      Sender<Result<Window, XErr>>,
	},
	DestroyWindow(Window, Sender<Result<(), XErr>>),
}

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
					share,
					builder,
					tx,
				} => create_window(width, height, title, monitor, share, builder, tx),
				| XWinMessage::DestroyWindow(win, tx) => destroy_window(win, tx),
			};
		}
	}
}

fn destroy_window(win: Window, tx: Sender<Result<(), XErr>>)
{
	unsafe { glfwDestroyWindow(win.get_glfw()) };
	let _ = tx.send(XErr::result(|| ()));
}

fn create_window(
	width: i32,
	height: i32,
	title: String,
	monitor: Option<Monitor>,
	share: Option<Window>,
	builder: Option<WindowBuilder>,
	tx: Sender<Result<Window, XErr>>,
)
{
	unsafe { glfwDefaultWindowHints() };
	if let Err(err) = XErr::result(|| ())
	{
		let _ = tx.send(Err(err));
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

	let str = CString::new(title).expect("Title contains a null byte");
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
			match share
			{
				| Some(win) => win.get_glfw(),
				| None => null_mut(),
			},
		)
	};

	let _ = if win.is_null()
	{
		tx.send(Err(XErr::get()))
	}
	else
	{
		tx.send(Ok(Window::from_glfw(win)))
	};
}

fn set_gamma_ramp(mon: Monitor, ramp: GammaRamp, tx: Sender<Result<(), XErr>>)
{
	let mut ramp = ramp;
	ramp.with_glfw(|ramp| unsafe { glfwSetGammaRamp(mon.get_glfw(), ramp) });
	let _ = tx.send(XErr::result(|| ()));
}

fn gamma_ramp(mon: Monitor, tx: Sender<Result<GammaRamp, XErr>>)
{
	let ramp = unsafe { glfwGetGammaRamp(mon.get_glfw()) };
	let _ = tx.send(match unsafe { ramp.as_ref() }
	{
		| None => Err(XErr::get()),
		| Some(gr) => Ok(GammaRamp::from_glfw(gr)),
	});
}

fn set_gamma(mon: Monitor, gamma: f32, tx: Sender<Result<(), XErr>>)
{
	unsafe { glfwSetGamma(mon.get_glfw(), gamma) };
	let _ = tx.send(XErr::result(|| ()));
}

fn monitor_video_mode(mon: Monitor, tx: Sender<Result<VideoMode, XErr>>)
{
	let vm_ptr = unsafe { glfwGetVideoMode(mon.get_glfw()) };
	let _ = tx.send(match unsafe { vm_ptr.as_ref() }
	{
		| None => Err(XErr::get()),
		| Some(vm) => Ok(VideoMode::from_glfw(vm)),
	});
}

fn monitor_video_modes(mon: Monitor, tx: Sender<Result<Vec<VideoMode>, XErr>>)
{
	let mut count = 0i32;
	let vms = unsafe { glfwGetVideoModes(mon.get_glfw(), &mut count) };
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

fn monitor_name(mon: Monitor, tx: Sender<Result<String, XErr>>)
{
	let title = unsafe { glfwGetMonitorName(mon.get_glfw()) };
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

fn monitor_content_scale(mon: Monitor, tx: Sender<Result<ContentScale, XErr>>)
{
	let mut xscale = 0.0f32;
	let mut yscale = 0.0f32;
	unsafe { glfwGetMonitorContentScale(mon.get_glfw(), &mut xscale, &mut yscale) };
	let _ = tx.send(XErr::result(|| {
		ContentScale {
			x: xscale,
			y: yscale,
		}
	}));
}

fn monitor_physical_size(mon: Monitor, tx: Sender<Result<Millimeters, XErr>>)
{
	let mut width = 0i32;
	let mut height = 0i32;
	unsafe { glfwGetMonitorPhysicalSize(mon.get_glfw(), &mut width, &mut height) };
	let _ = tx.send(XErr::result(|| {
		Millimeters {
			x: width,
			y: height,
		}
	}));
}

fn monitor_work_area(mon: Monitor, tx: Sender<Result<WorkArea, XErr>>)
{
	let mut area = WorkArea::default();
	unsafe {
		glfwGetMonitorWorkarea(
			mon.get_glfw(),
			&mut area.pos.x,
			&mut area.pos.y,
			&mut area.size.x,
			&mut area.size.y,
		)
	};
	let _ = tx.send(XErr::result(|| area));
}

fn monitor_pos(mon: Monitor, tx: Sender<Result<ScreenCoordinates, XErr>>)
{
	let mut pos = ScreenCoordinates::default();
	unsafe { glfwGetMonitorPos(mon.get_glfw(), &mut pos.x, &mut pos.y) };
	let _ = tx.send(XErr::result(|| pos));
}

fn primary_monitor(tx: Sender<Result<Monitor, XErr>>)
{
	let monitor = unsafe { glfwGetPrimaryMonitor() };
	let _ = tx.send(
		if monitor.is_null()
		{
			Err(XErr::get())
		}
		else
		{
			Ok(Monitor::from_glfw(monitor))
		},
	);
}

fn monitors(tx: Sender<Result<Vec<Monitor>, XErr>>)
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
			let mut vec = Vec::<Monitor>::with_capacity(count as usize);
			for idx in 0..count as usize
			{
				vec.push(Monitor::from_glfw(unsafe { *monitors.add(idx) }));
			}
			Ok(vec)
		},
	);
}
