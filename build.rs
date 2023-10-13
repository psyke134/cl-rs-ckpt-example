fn main() {
	let asp_lib_dir: &str = "/opt/clovis/sdk-6.0/target/x86_64/linux-6.2.16/lib";

	println!("cargo:rustc-link-search={}", asp_lib_dir);

	// Add link to SAFplus's libmw.so
	println!("cargo:rustc-link-lib=mw");
}
