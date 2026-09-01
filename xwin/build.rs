use std::{
	env,
	path::PathBuf,
};

use cmake::Config;

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

	// Link GLFW
	println!("cargo:rustc-link-search=native={}/lib", glfw);
	println!("cargo:rustc-link-lib=dylib=glfw3dll");

	// Generate rust bindings
	let bindings = bindgen::Builder::default()
		.clang_arg(format!("-I{}/include", glfw))
		.header("lib/xwin.h")
		.parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
		.generate()
		.expect("Unable to generate bindings");

	// Write bindings
	let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
	bindings
		.write_to_file(out_path.join("bindings.rs"))
		.expect("Couldn't write bindings!");
}
