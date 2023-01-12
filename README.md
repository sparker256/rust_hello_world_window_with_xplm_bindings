
# Rust Hello World Window With XPLM Bindings plugin

This is my demonstration of how to convert a X-Plane C++ plugin to Rust.
In this case I started with my https://github.com/sparker256/Hello-World-SDK-4 


Using my first fork I was able to see messages when X-Plane was run in a terminal
if you enabeled or disabled this plugin and did not need any bindings.


I then started using bindings with ([bindgen](https://rust-lang.github.io/rust-bindgen/) directly),
and could then use XPLMDebugString so the message would be put into the Log.txt and also
if you opened the dev console you would see the messages when the plugin was enabled or disabled.


I next added a window and put some text on it. I did this by using XPLMCreateWindow_t, XPLMCreateWindowEx,
XPLMSetGraphicsState, XPLMGetWindowGeometry and XPLMDrawString.


I have also added GitHub Actions to when I do a push it will automatcily build a new versio.


This build has been tested on Intel MacOS 12.6, Ubuntu 22.04 and Windows 10.

![Alt text](Rust_Hello_World_Window.jpg?raw=true "Rust_Hello_World_Window")


## Building on Linux

   ```
   cargo clean
   cargo build --release --target=x86_64-unknown-linux-gnu
   mkdir -p target/dist/rust_hello_world_window/lin_x64
   cp -v "target/x86_64-unknown-linux-gnu/release/librust_hello_world_window.so" "target/dist/rust_hello_world_window/lin_x64/rust_hello_world_window.xpl"
   ```

## Building on Windows

   ```
   cargo clean
   cargo build --release --target=x86_64-pc-windows-gnu
   mkdir -p target/dist/rust_hello_world_window/win_x64
   cp -v "target/x86_64-pc-windows-gnu/release/rust_hello_world_window.dll" "target/dist/rust_hello_world_window/win_x64/rust_hello_world_window.xpl"
   ```

## Building on Mac x86_64

   ```
   cargo clean
   cargo build --release --target=x86_64-apple-darwin
   mkdir -p target/dist/rust_hello_world_window/mac_x64
   cp -v "target/x86_64-apple-darwin/release/librust_hello_world_window.dylib" "target/dist/rust_hello_world_window/mac_x64/rust_hello_world_window.xpl"
   ```


## Building on Mac aarch64

   ```
   cargo clean
   cargo build --release --target=aarch64-apple-darwin
   mkdir -p target/dist/rust_hello_world_window/mac_x64
   cp -v "target/aarch64-apple-darwin/release/librust_hello_world_window.dylib" "target/dist/rust_hello_world_window/mac_x64/rust_hello_world_window.xpl"
   ```


## bindings

I am using ([bindgen](https://rust-lang.github.io/rust-bindgen/) directly), to
generate the bindings.

These can be invoked by this example:

```rust
    const BUF_NAME: &str = "Rust Hello World WindowEnabled!\n";
    let name = CString::new(BUF_NAME).expect("");
    bindings::XPLMDebugString(name.as_ptr());
```
