use std::{
	ffi::CStr,
	sync::mpsc::Sender,
};

use crate::{
	bind::{
		GLFWmonitor,
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
	},
	err::XErr,
	monitor::{
		GammaRamp,
		Millimeters,
		VideoMode,
		WorkArea,
	},
};

pub(super) fn set_gamma_ramp(mon: *mut GLFWmonitor, ramp: GammaRamp, tx: Sender<Result<(), XErr>>)
{
	let mut ramp = ramp;
	ramp.with_glfw(|ramp| unsafe { glfwSetGammaRamp(mon, ramp) });
	let _ = tx.send(XErr::result(|| ()));
}

pub(super) fn gamma_ramp(mon: *mut GLFWmonitor, tx: Sender<Result<GammaRamp, XErr>>)
{
	let ramp = unsafe { glfwGetGammaRamp(mon) };
	let _ = tx.send(match unsafe { ramp.as_ref() }
	{
		| None => Err(XErr::get()),
		| Some(gr) => Ok(GammaRamp::from_glfw(gr)),
	});
}

pub(super) fn set_gamma(mon: *mut GLFWmonitor, gamma: f32, tx: Sender<Result<(), XErr>>)
{
	unsafe { glfwSetGamma(mon, gamma) };
	let _ = tx.send(XErr::result(|| ()));
}

pub(super) fn monitor_video_mode(mon: *mut GLFWmonitor, tx: Sender<Result<VideoMode, XErr>>)
{
	let vm_ptr = unsafe { glfwGetVideoMode(mon) };
	let _ = tx.send(match unsafe { vm_ptr.as_ref() }
	{
		| None => Err(XErr::get()),
		| Some(vm) => Ok(VideoMode::from_glfw(vm)),
	});
}

pub(super) fn monitor_video_modes(mon: *mut GLFWmonitor, tx: Sender<Result<Vec<VideoMode>, XErr>>)
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

pub(super) fn monitor_name(mon: *mut GLFWmonitor, tx: Sender<Result<String, XErr>>)
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

pub(super) fn monitor_content_scale(mon: *mut GLFWmonitor, tx: Sender<Result<ContentScale, XErr>>)
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

pub(super) fn monitor_physical_size(mon: *mut GLFWmonitor, tx: Sender<Result<Millimeters, XErr>>)
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

pub(super) fn monitor_work_area(mon: *mut GLFWmonitor, tx: Sender<Result<WorkArea, XErr>>)
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

pub(super) fn monitor_pos(mon: *mut GLFWmonitor, tx: Sender<Result<ScreenCoordinates, XErr>>)
{
	let mut pos = ScreenCoordinates::default();
	unsafe { glfwGetMonitorPos(mon, &mut pos.x, &mut pos.y) };
	let _ = tx.send(XErr::result(|| pos));
}

pub(super) fn primary_monitor(tx: Sender<Result<*mut GLFWmonitor, XErr>>)
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

pub(super) fn monitors(tx: Sender<Result<Vec<*mut GLFWmonitor>, XErr>>)
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
