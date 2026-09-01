use std::str::FromStr;

use proc_macro::TokenStream;

/// This attribute indicates that a function is to be called when a monitor is
/// connected to or disconnected from the system.
///
/// # Callback signature
/// `fn monitor_callback(monitor: &Monitor, event: MonitorEvent)`
///
/// # Remarks
/// This macro requires xwin to be built with the feature "linkme" (enabled by
/// default), as it currently relies on the [linkme] crate to function. If you
/// prefer to reduce dependencies, you can disable the "linkme" feature and
/// manually set monitor callbacks.
#[proc_macro_attribute]
pub fn monitor_callback(_attr: TokenStream, item: TokenStream) -> TokenStream
{
	let str = format!(
		"#[::xwin::__linkme::distributed_slice(::xwin::monitor::__MONITOR_CALLBACKS)]\n#\
		 [linkme(crate = xwin::__linkme)]\n{}",
		item.to_string()
	);
	TokenStream::from_str(&str).expect("Failed to parse monitor callback")
}
