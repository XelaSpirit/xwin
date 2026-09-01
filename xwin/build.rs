use std::path::PathBuf;

use cmake::Config;
use xb::{
	CargoCmd,
	SearchKind,
	dep::deploy,
	out_dir,
};

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
