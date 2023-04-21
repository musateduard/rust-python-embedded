use bindgen::{self, CargoCallbacks, Bindings, builder};


fn main() -> () {

    // check if bindings.rs has changed
    println!("cargo:rerun-if-changed=./src/bindings.rs");

    // create bindings object that includes all the allowed functions from the header files
    let bindings: Bindings = builder()
        .header("/usr/include/python3.10/Python.h")
        .allowlist_type("PyStatus")
        .allowlist_type("PyPreConfig")
        .allowlist_type("PyConfig")
        .allowlist_function("PyConfig_SetBytesString")
        .allowlist_function("PyPreConfig_InitIsolatedConfig")
        .allowlist_function("Py_PreInitialize")
        .allowlist_function("PyConfig_InitIsolatedConfig")
        .allowlist_function("Py_InitializeFromConfig")
        .allowlist_function("Py_RunMain")
        .parse_callbacks(Box::new(CargoCallbacks))
        .generate()
        .expect("unable to generate bindings");

    // write the bindings to ./src/bindings.rs
    bindings
        .write_to_file("./src/bindings.rs")
        .expect("unable to write bindings");

    return}
