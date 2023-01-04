#![allow(non_snake_case)]
#![allow(unused_imports)]
#[allow(dead_code)]
use crate::bindings;

use enum_primitive_derive::Primitive;

use std::ffi::{c_char, c_int, c_void, CString};
use std::ptr;

unsafe fn copy_to_c_buffer(src: String, dest: *mut c_char) {
    let src_c = CString::new(src).unwrap_or_else(|_| CString::new("<invalid>").unwrap());
    let src_c_length = src_c.to_bytes_with_nul().len();
    ptr::copy_nonoverlapping(src_c.as_ptr(), dest, src_c_length);
}

#[no_mangle]
unsafe extern "C" fn XPluginStart(
    name: *mut c_char,
    signature: *mut c_char,
    description: *mut c_char,
) -> c_int {
    copy_to_c_buffer(String::from("Rust Hello World"), name);
    copy_to_c_buffer(String::from("org.SparkerInVR.rust.hello.world"), signature);
    copy_to_c_buffer(String::from("Rust Hello World Plugin"), description);
    1
}

#[no_mangle]
unsafe extern "C" fn XPluginEnable() -> c_int {
    println!("[rust-hello-world] Enabled");
    1
}

#[no_mangle]
unsafe extern "C" fn XPluginDisable() {
    println!("[rust-hello-world] Disabled");
}

#[no_mangle]
unsafe extern "C" fn XPluginStop() {}


#[allow(unused_variables)]
#[no_mangle]
unsafe extern "C" fn XPluginReceiveMessage(
    from: c_int,
    message: c_int,
    param: *mut c_void,
) {}
