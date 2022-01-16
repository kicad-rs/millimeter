fn main() {
	if let Some(true) = rustc::supports_feature("const_float_classify") {
		println!("cargo:rustc-cfg=has_const_float_classify");
	}
}
