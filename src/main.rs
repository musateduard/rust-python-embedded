// declaration for bindgen to include the generated bindings
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unused_braces)]

use std::ffi::{CString};
use std::mem::{MaybeUninit};

include!("./bindings.rs");


// main function
fn main() -> () {

    let mut python_status: PyStatus = unsafe {MaybeUninit::<PyStatus>::zeroed().assume_init()};
    let mut python_preconfig: PyPreConfig = unsafe {MaybeUninit::<PyPreConfig>::zeroed().assume_init()};
    let mut python_config: PyConfig = unsafe {MaybeUninit::<PyConfig>::zeroed().assume_init()};

    // PyConfig fields
    let run_command: CString = CString::new(include_str!("../python/code.py")).unwrap();
    let program_name: CString = CString::new("my python program").unwrap();
    let home: CString = CString::new("./python/build").unwrap();

    // todo: configure sys.path in python to only search ./python/build/lib

    // initialize PyPreConfig
    unsafe {PyPreConfig_InitIsolatedConfig(&mut python_preconfig)};
    python_preconfig.isolated = 1;
    python_preconfig.utf8_mode = 1;
    python_status = unsafe {Py_PreInitialize(&mut python_preconfig)};
    println!("preinitialized with exit code: {}", python_status.exitcode);

    // initialize PyConfig
    unsafe {PyConfig_InitIsolatedConfig(&mut python_config)};
    python_config.install_signal_handlers = 1;
    unsafe {
        PyConfig_SetBytesString(&mut python_config, &mut python_config.program_name, program_name.as_ptr());
        PyConfig_SetBytesString(&mut python_config, &mut python_config.home, home.as_ptr());
        PyConfig_SetBytesString(&mut python_config, &mut python_config.run_command, run_command.as_ptr());
        Py_InitializeFromConfig(&mut python_config);}
    println!("initialized with exit code: {}", python_status.exitcode);

    // run python
    unsafe {Py_RunMain()};

    println!("finished running Py_RunMain");

    return ();}
