#!/bin/sh -e

PROJECT="rust_hello_world"

PLUGIN_TARGET="target/dist/${PROJECT}"

cargo clean
cargo build --release --target=aarch64-apple-darwin
cargo build --release --target=x86_64-apple-darwin
cargo build --release --target=x86_64-pc-windows-gnu
cargo build --release --target=x86_64-unknown-linux-gnu

mkdir -p "${PLUGIN_TARGET}/lin_x64" "${PLUGIN_TARGET}/mac_x64" "${PLUGIN_TARGET}/win_x64"

lipo -create -output "target/dist/${PROJECT}/mac_x64/${PROJECT}.xpl" "target/x86_64-apple-darwin/release/lib${PROJECT}.dylib" "target/aarch64-apple-darwin/release/lib${PROJECT}.dylib"

cp -v "target/x86_64-unknown-linux-gnu/release/lib${PROJECT}.so" "target/dist/${PROJECT}/lin_x64/${PROJECT}.xpl"
cp -v "target/x86_64-pc-windows-gnu/release/${PROJECT}.dll" "target/dist/${PROJECT}/win_x64/${PROJECT}.xpl"

echo "Distribution built at ${PLUGIN_TARGET}"

