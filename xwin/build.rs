use std::{
	env,
	path::PathBuf,
};

use cmake::Config;
use xb::{
	CargoCmd,
	SearchKind,
	dep::deploy,
	out_dir,
};

const VK_SDK_PATH: &str = "VULKAN_SDK_PATH";

fn setup_vulkan(bindings: bindgen::Builder) -> bindgen::Builder
{
	if let Ok(vk) = env::var(VK_SDK_PATH)
	{
		CargoCmd()
			.link_lib("dylib=vulkan-1")
			.rerun_if_env_changed(VK_SDK_PATH);

		bindings
			.clang_arg("-DGLFW_INCLUDE_VULKAN")
			.clang_arg(format!("-I{}/Include", vk))
	}
	else
	{
		bindings
	}
}

fn main()
{
	// Build GLFW
	let dst = Config::new("lib/glfw")
		.define("GLFW_LIBRARY_TYPE", "SHARED")
		.define("GLFW_BUILD_EXAMPLES", "OFF")
		.define("GLFW_BUILD_TESTS", "OFF")
		.define("GLFW_BUILD_DOCS", "OFF")
		.build();
	let glfw = dst.display();

	// Link glfw
	CargoCmd()
		.link_search(format!("{}/lib", glfw), SearchKind::Native)
		.link_search(format!("{}/bin", glfw), SearchKind::Native)
		.link_lib("dylib=glfw3dll");

	// Put glfw dll in the output directory
	let dll = PathBuf::from(format!("{}/bin/glfw3.dll", glfw));
	deploy(dll).expect("Unable to copy glfw3.dll");

	// Generate rust bindings
	let bindings = bindgen::Builder::default()
		.clang_arg(format!("-I{}/include", glfw))
		.clang_arg("-DGLFW_DLL")
		.clang_arg("-DGLFW_INCLUDE_NONE"); // TODO add support for OpenGL

	#[cfg(feature = "vulkan")]
	let bindings = setup_vulkan(bindings);

	let bindings = bindings
		.header("lib/xwin.h")
		.parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
		.generate()
		.expect("Unable to generate bindings");

	// Write bindings
	let out_path = PathBuf::from(out_dir().unwrap());
	bindings
		.write_to_file(out_path.join("bindings.rs"))
		.expect("Couldn't write bindings!");
}
