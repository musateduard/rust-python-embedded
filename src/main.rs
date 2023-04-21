// declaration for bindgen to include the generated bindings
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_variables)]


use std::ffi::{CString, c_int};
use std::mem::{MaybeUninit};


include!("./bindings.rs");


// python rust architecture
// python interpreter embedded into rust complete with modules and script
// no interpreter or scrips in the folder
// no writing to file
// dlls are allowed


// main function
fn main() -> () {

    let mut python_status: PyStatus = unsafe {MaybeUninit::<PyStatus>::zeroed().assume_init()};
    let mut python_preconfig: PyPreConfig = unsafe {MaybeUninit::<PyPreConfig>::zeroed().assume_init()};
    let mut python_config: PyConfig = unsafe {MaybeUninit::<PyConfig>::zeroed().assume_init()};

    let python_code: CString = CString::new(include_str!("../python/code.py")).unwrap();
    let python_name: CString = CString::new("my python program").unwrap();
    let python_home: CString = CString::new("./python/build").unwrap();

    // initialize preconfig struct
    unsafe {PyPreConfig_InitIsolatedConfig(&mut python_preconfig)};
    // python_preconfig.configure_locale = 1;
    // python_preconfig.coerce_c_locale = 2;
    python_preconfig.isolated = 1;
    python_preconfig.utf8_mode = 1;
    python_status = unsafe {Py_PreInitialize(&mut python_preconfig)};
    println!("preinitialized with exit code: {}", python_status.exitcode);

    // initialize config struct
    unsafe {PyConfig_InitIsolatedConfig(&mut python_config)};
    // python_config.isolated = 1;
    // python_config.interactive = 1;
    python_config.install_signal_handlers = 1;
    python_config.verbose = 0;
    unsafe {
        PyConfig_SetBytesString(&mut python_config, &mut python_config.program_name, python_name.as_ptr());
        PyConfig_SetBytesString(&mut python_config, &mut python_config.home, python_home.as_ptr());
        PyConfig_SetBytesString(&mut python_config, &mut python_config.run_command, python_code.as_ptr());
        Py_InitializeFromConfig(&mut python_config);}
    println!("initialized with exit code: {}", python_status.exitcode);

    // run python
    unsafe {Py_RunMain()};

    println!("finished running Py_RunMain");

    return}
