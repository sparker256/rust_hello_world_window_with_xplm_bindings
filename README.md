
# Rust Hello World With XPLM Bindings plugin

This is a trivial plugin which just prints to stdout when
enabled or disabled.

This build has only been tested on Intel MacOS 12.6 and Ubuntu 22.04.

## Installing into X-Plane

The built `dylib` should be copied into the appropriate plugin
structure within X-Plane, for example:
```shell
cp target/debug/libhello_plugin_rust.dylib \ 
  <aircraft>/hello-plugin-rust/mac_x64/hello-plugin-rust.xpl
```

## Adding more bindings

You can use [xplm-sys](https://crates.io/crates/xplm-sys) (or [bindgen](https://rust-lang.github.io/rust-bindgen/) directly), to
generate the bindings, or you can create them manually, for example:

```rust
type XPLMCommandRef = *mut c_void;

extern "C" {
    pub fn XPLMFindCommand(inName: *const c_char) -> XPLMCommandRef;
}

extern "C" {
    pub fn XPLMCommandBegin(inCommand: XPLMCommandRef);
}

extern "C" {
    pub fn XPLMCommandEnd(inCommand: XPLMCommandRef);
}
```

Once added, these can be invoked once types are converted, for example:

```rust
    const DREF_NAME: &str = "sim/flight_controls/pitch_trim_up";

    let name = CString::new(DREF_NAME).expect("");
    let data_ref = XPLMFindCommand(name.as_ptr());
    XPLMCommandBegin(data_ref);
    // ...
    XPLMCommandEnd(data_ref);
```
