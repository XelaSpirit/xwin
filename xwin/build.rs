use std::{
	env,
	path::PathBuf,
};

use cmake::Config;
use xb::{
	CargoCmd,
	SearchKind,
	dep::deploy,
	is_feature,
	out_dir,
};

// GLFW clang args
const DEF_GLFW_DLL: &str = "GLFW_DLL";
const DEF_GLFW_INCLUDE_NONE: &str = "GLFW_INCLUDE_NONE";

// Feature clang args
const DEF_VULKAN: &str = "GLFW_INCLUDE_VULKAN";

// Features
const VULKAN: &str = "vulkan";

// Additional config environment variables
const VULKAN_SDK_PATH: &str = "VULKAN_SDK_PATH";

fn config_features(mut bindings: bindgen::Builder) -> bindgen::Builder
{
	if is_feature(VULKAN)
	{
		if let Ok(vk) = env::var(VULKAN_SDK_PATH)
		{
			CargoCmd()
				.link_lib("dylib=vulkan-1")
				.rerun_if_env_changed(VULKAN_SDK_PATH);

			bindings = bindings
				.clang_arg(format!("-D{}", DEF_VULKAN))
				.clang_arg(format!("-I{}/Include", vk))
		}
	}

	bindings
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

	let out_path = PathBuf::from(out_dir().unwrap());

	// Generate rust bindings
	config_features(bindgen::Builder::default())
		.clang_arg(format!("-I{}/include", glfw))
		.clang_arg(format!("-D{}", DEF_GLFW_DLL))
		.clang_arg(format!("-D{}", DEF_GLFW_INCLUDE_NONE))
		.header("lib/xwin.h")
		.parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
		.generate()
		.expect("Unable to generate bindings")
		.write_to_file(out_path.join("bindings.rs"))
		.expect("Couldn't write bindings!");
}
