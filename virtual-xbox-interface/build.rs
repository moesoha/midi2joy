fn main() {
	let lib_dir=concat!(env!("CARGO_MANIFEST_DIR"),"/lib");
	println!("cargo:rustc-link-lib=static=vXboxInterface");
	println!("cargo:rustc-link-search={}",lib_dir);
	// TODO: copy dll to dest
}
