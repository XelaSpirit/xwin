use std::str::FromStr;

use proc_macro::TokenStream;

/// This attribute indicates that a function is to be called when a monitor is
/// connected to or disconnected from the system.
///
/// # Callback signature
/// `fn monitor_callback(monitor: &Monitor, event: MonitorEvent)`
#[proc_macro_attribute]
pub fn monitor_callback(_attr: TokenStream, item: TokenStream) -> TokenStream
{
	let str = format!(
		"#[::xwin::__linkme::distributed_slice(::xwin::monitor::MONITOR_CALLBACKS)]\n#\
		 [linkme(crate = xwin::__linkme)]\n{}",
		item.to_string()
	);
	TokenStream::from_str(&str).expect("Failed to parse monitor callback")
}
