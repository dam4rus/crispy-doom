fn main() {
    cbindgen::Builder::new()
        .with_crate(".")
        .with_config(cbindgen::Config::from_root_or_default("."))
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("../src/doom/rs/am_map.h");
}
