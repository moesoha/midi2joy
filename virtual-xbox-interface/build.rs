use std::env;
use std::path::Path;

fn main() {
	let target=env::var("TARGET").unwrap();
	let lib_dir=format!("{}/lib/{}", env!("CARGO_MANIFEST_DIR"), target);
	if !Path::new(&lib_dir).exists() {
		panic!(format!("Target \"{}\" is not supported.", target));
	}
	println!("cargo:rustc-link-lib=static=vXboxInterface");
	println!("cargo:rustc-link-search={}", lib_dir);
}
