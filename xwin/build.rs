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
const DEF_NATIVE_COCOA: &str = "GLFW_EXPOSE_NATIVE_COCOA";
const DEF_NATIVE_EGL: &str = "GLFW_EXPOSE_NATIVE_EGL";
const DEF_NATIVE_GLX: &str = "GLFW_EXPOSE_NATIVE_GLX";
const DEF_NATIVE_NONE: &str = "GLFW_NATIVE_INCLUDE_NONE";
const DEF_NATIVE_OSMESA: &str = "GLFW_EXPOSE_NATIVE_OSMESA";
const DEF_NATIVE_NSGL: &str = "GLFW_EXPOSE_NATIVE_NSGL";
const DEF_NATIVE_WAYLAND: &str = "GLFW_EXPOSE_NATIVE_WAYLAND";
const DEF_NATIVE_WGL: &str = "GLFW_EXPOSE_NATIVE_WGL";
const DEF_NATIVE_WIN32: &str = "GLFW_EXPOSE_NATIVE_WIN32";
const DEF_NATIVE_X11: &str = "GLFW_EXPOSE_NATIVE_X11";
const DEF_VULKAN: &str = "GLFW_INCLUDE_VULKAN";

// Features
const COCOA: &str = "cocoa";
const EGL: &str = "egl";
const GLX: &str = "glx";
const NSGL: &str = "nsgl";
const OSMESA: &str = "osmesa";
const VULKAN: &str = "vulkan";
const WAYLAND: &str = "wayland";
const WGL: &str = "wgl";
const WIN32: &str = "win32";
const X11: &str = "x11";

// Additional config environment variables
const VULKAN_SDK_PATH: &str = "VULKAN_SDK_PATH";

fn if_define(mut bindings: bindgen::Builder, env: &str, def: &str) -> bindgen::Builder
{
	if is_feature(env)
	{
		bindings = bindings.clang_arg(format!("-D{}", def));
	}
	bindings
}

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

	bindings = if_define(bindings, COCOA, DEF_NATIVE_COCOA);
	bindings = if_define(bindings, EGL, DEF_NATIVE_EGL);
	bindings = if_define(bindings, GLX, DEF_NATIVE_GLX);
	bindings = if_define(bindings, NSGL, DEF_NATIVE_NSGL);
	bindings = if_define(bindings, OSMESA, DEF_NATIVE_OSMESA);
	bindings = if_define(bindings, WAYLAND, DEF_NATIVE_WAYLAND);
	bindings = if_define(bindings, WGL, DEF_NATIVE_WGL);
	bindings = if_define(bindings, WIN32, DEF_NATIVE_WIN32);
	bindings = if_define(bindings, X11, DEF_NATIVE_X11);

	if !(is_feature(COCOA)
		|| is_feature(EGL)
		|| is_feature(GLX)
		|| is_feature(NSGL)
		|| is_feature(OSMESA)
		|| is_feature(WAYLAND)
		|| is_feature(WGL)
		|| is_feature(WIN32)
		|| is_feature(X11))
	{
		bindings = bindings.clang_arg(format!("-D{}", DEF_NATIVE_NONE));
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
