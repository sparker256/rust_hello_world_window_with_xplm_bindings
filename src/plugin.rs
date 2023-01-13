#![allow(non_snake_case)]
#![allow(unused_imports)]
#[allow(dead_code)]
use crate::bindings;
use bindings::{XPLMDrawString, XPLMFontID, XPLMSetGraphicsState, xplmFont_Proportional, XPLMDebugString,
    XPLMWindowID, XPLMCreateWindow_t, XPLMKeyFlags, xplm_WindowLayerFloatingWindows,
    xplm_WindowDecorationRoundRectangle, XPLMCreateWindowEx, xplm_CursorDefault, XPLMGetScreenBoundsGlobal,
    XPLMGetWindowGeometry, XPLMSetWindowPositioningMode, XPLMSetWindowResizingLimits, XPLMSetWindowTitle,
    xplm_WindowPositionFree};


use std::ffi::{c_char, c_int, c_void, CString};
use std::ptr;
use std::ptr::null_mut;


#[allow(unused_variables)]
#[no_mangle]
unsafe extern "C" fn dummy_mouse_handler(_in_window_id: XPLMWindowID,
    x: ::std::os::raw::c_int,
    y: ::std::os::raw::c_int,
    is_down: ::std::os::raw::c_int,
    in_refcon: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int
{
    return 0;
}


#[allow(unused_variables)]
#[no_mangle]
unsafe extern "C" fn dummy_cursor_status_handler(_in_window_id: XPLMWindowID,
    x: ::std::os::raw::c_int,
    y: ::std::os::raw::c_int,
    in_refcon: *mut ::std::os::raw::c_void) -> i32
{
    return xplm_CursorDefault.try_into().unwrap();
}


#[allow(unused_variables)]
#[no_mangle]
unsafe extern "C" fn dummy_wheel_handler(_in_window_id: XPLMWindowID,
    x: ::std::os::raw::c_int,
    y: ::std::os::raw::c_int,
    wheel: ::std::os::raw::c_int,
    clicks: ::std::os::raw::c_int,
    in_refcon: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int
{
     return 0;
}

#[allow(unused_variables)]
#[no_mangle]
unsafe extern "C" fn dummy_key_handler(_in_window_id: XPLMWindowID,
    key: i8,
    flags: XPLMKeyFlags,
    virtual_key: i8,
    in_refcon: *mut ::std::os::raw::c_void ,
    losing_focus: ::std::os::raw::c_int)
{

}


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
    copy_to_c_buffer(String::from("Rust Hello World Window"), name);
    copy_to_c_buffer(String::from("org.SparkerInVR.rust.hello.world.window"), signature);
    copy_to_c_buffer(String::from("Rust Hello World Window Plugin"), description);

    let mut window_params = XPLMCreateWindow_t {
        structSize: std::mem::size_of::<XPLMCreateWindow_t>() as i32,
        visible: true as i32,
        left: 150,
        top: 600,
        right: 650,
        bottom: 300,
        drawWindowFunc: Some(draw_hello_world_window),
        // Note on "dummy" handlers:
        // Even if we don't want to handle these events, we have to register a "do-nothing" callback for them
        handleMouseClickFunc: Some(dummy_mouse_handler),
        handleRightClickFunc: Some(dummy_mouse_handler),
        handleMouseWheelFunc: Some(dummy_wheel_handler),
        handleKeyFunc: Some(dummy_key_handler),
        handleCursorFunc: Some(dummy_cursor_status_handler),
        refcon: null_mut(),
        layer: xplm_WindowLayerFloatingWindows as i32,
        // Opt-in to styling our window like an X-Plane 11 native window
        // If you're on XPLM300, not XPLM301, swap this enum for the literal value 1.
        decorateAsFloatingWindow: xplm_WindowDecorationRoundRectangle as i32,
     };

    // Set the window's initial bounds
	// Note that we're not guaranteed that the main monitor's lower left is at (0, 0)...
	// We'll need to query for the global desktop bounds!
    let mut left: i32 = 150;
    let mut top: i32 = 600;
    let mut right: i32 = 650;
    let mut bottom: i32 = 300;

    XPLMGetScreenBoundsGlobal(
        &mut left,
        &mut top,
        &mut right,
        &mut bottom,
    );

    window_params.left = left + 350;
    window_params.bottom = bottom + 150;
    window_params.right = window_params.left +200;
    window_params.top = window_params.bottom + 200;
    
    let window_id: XPLMWindowID = unsafe { XPLMCreateWindowEx(&mut window_params) };

    // Position the window as a "free" floating window, which the user can drag around
	XPLMSetWindowPositioningMode(window_id,
        xplm_WindowPositionFree.try_into().unwrap(),
        -1);


    // Limit resizing our window: maintain a minimum width/height of 100 boxels and a max width/height of 300 boxels
    XPLMSetWindowResizingLimits(window_id,
        250,
        200,
        350,
        300);


    const BUF_NAME: &str = "Rust Hello World Window\n";
    let name = CString::new(BUF_NAME).expect("");
    XPLMSetWindowTitle(window_id, name.as_ptr());
      

    1
}

#[no_mangle]
unsafe extern "C" fn XPluginEnable() -> c_int {
    println!("[hello-plugin-rust] Enabled");
    const BUF_NAME: &str = "Rust Hello World Window Enabled!\n";
    let name = CString::new(BUF_NAME).expect("");
    XPLMDebugString(name.as_ptr());
    1
}

#[no_mangle]
unsafe extern "C" fn XPluginDisable() {
    println!("[hello-plugin-rust] Disabled");
    const BUF_NAME: &str = "Rust Hello World Window Disabled!\n";
    let name = CString::new(BUF_NAME).expect("");
    XPLMDebugString(name.as_ptr());
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


#[allow(unused_variables)]
#[no_mangle]
// This now seems to match my Hello-World-SDK-4 draw_hello_world function
unsafe extern "C" fn draw_hello_world_window(
    hd_window_id: XPLMWindowID,
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

    XPLMGetWindowGeometry(
        hd_window_id,
        &mut l,
        &mut t,
        &mut r,
        &mut b,
    );

    const WINDOW_TEXT: &str = "Rust Hello World Window!\n";

    let line_c = CString::new(WINDOW_TEXT).unwrap();
    let color_white: [f32; 3] = [1.0, 1.0, 1.0];
    
    unsafe {
        XPLMDrawString(
            color_white.as_ptr() as *mut f32,
            l + 50,
            t - 20,
            line_c.as_ptr() as *mut i8,
            null_mut(),
            xplmFont_Proportional.try_into().unwrap(),
        )
    };

  }
