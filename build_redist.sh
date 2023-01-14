#!/bin/sh -e

PROJECT="rust_hello_world_window"

PLUGIN_TARGET="target/dist/${PROJECT}"

cargo clean
cargo build --release --target=aarch64-apple-darwin
cargo build --release --target=x86_64-apple-darwin
cargo build --release --target=x86_64-pc-windows-gnu
cargo build --release --target=x86_64-unknown-linux-gnu

lipo -create -output "${PROJECT}/mac_x64/${PROJECT}.xpl" "target/x86_64-apple-darwin/release/lib${PROJECT}.dylib" "target/aarch64-apple-darwin/release/lib${PROJECT}.dylib"

cp -v "target/x86_64-unknown-linux-gnu/release/lib${PROJECT}.so" "${PROJECT}/lin_x64/${PROJECT}.xpl"
cp -v "target/x86_64-pc-windows-gnu/release/${PROJECT}.dll" "${PROJECT}/win_x64/${PROJECT}.xpl"

echo "Distribution built at ${PLUGIN_TARGET}"

