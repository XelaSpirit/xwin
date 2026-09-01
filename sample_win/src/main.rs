use xwin::{
	core::XWin,
	monitor::{
		Monitor,
		MonitorEvent,
		monitor_callback,
	},
};

fn main()
{
	let _xwin = XWin::new();
}

#[monitor_callback]
fn cb(monitor: &Monitor, ev: MonitorEvent)
{
	println!("Monitor event received: {:?}", ev);
}
