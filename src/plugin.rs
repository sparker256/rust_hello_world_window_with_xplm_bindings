#![allow(non_snake_case)]
#![allow(unused_imports)]
#[allow(dead_code)]
use crate::bindings;
use bindings::{XPLMDrawString, XPLMFontID, XPLMSetGraphicsState, xplmFont_Proportional};


use enum_primitive_derive::Primitive;

use std::ffi::{c_char, c_int, c_void, CString};
use std::ptr;
use std::ptr::null_mut;

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
    println!("[hello-plugin-rust] Enabled");
    const BUF_NAME: &str = "Rust Hello World Enabled!\n";
    let name = CString::new(BUF_NAME).expect("");
    bindings::XPLMDebugString(name.as_ptr());
    1
}

#[no_mangle]
unsafe extern "C" fn XPluginDisable() {
    println!("[hello-plugin-rust] Disabled");
    const BUF_NAME: &str = "Rust Hello World Disabled!\n";
    let name = CString::new(BUF_NAME).expect("");
    bindings::XPLMDebugString(name.as_ptr());
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


// This now seems to match my Hello-World-SDK-4 draw_hello_world function
unsafe extern "C" fn draw_hello_world(
    hd_window_id: bindings::XPLMWindowID,
    hw_refcon: *mut ::std::os::raw::c_void,
) {
    // Mandatory: We *must* set the OpenGL state before drawing
    // (we can't make any assumptions about it)
    XPLMSetGraphicsState(
        0,
        0,
        0,
        0,
        1,
        2,
        0
        );

    let mut l: i32 = 0;
    let mut t: i32 = 0;
    let mut r: i32 = 0;
    let mut b: i32 = 0;

    bindings::XPLMGetWindowGeometry(
        hd_window_id,
        &mut l,
        &mut t,
        &mut r,
        &mut b,
    );

    const WINDOW_TEXT: &str = "Rust Hello World!\n";

    let line_c = CString::new(WINDOW_TEXT).unwrap();
    let color_white: [f32; 3] = [1.0, 1.0, 1.0];
    
    unsafe {
        XPLMDrawString(
            color_white.as_ptr() as *mut f32,
            l + 10,
            t + 20,
            line_c.as_ptr() as *mut i8,
            null_mut(),
            xplmFont_Proportional.try_into().unwrap(),
        )
    };

  }