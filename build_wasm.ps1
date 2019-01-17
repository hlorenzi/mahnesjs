pushd rust

# Build wasm binary
cargo +nightly build --target wasm32-unknown-unknown --release

popd

# Reduce binary size
wasm-gc "./rust/target/wasm32-unknown-unknown/release/mahnes_rs.wasm" -o "./rust/target/wasm32-unknown-unknown/release/mahnes_rs.gc.wasm"

# Copy to correct folder
Copy-Item -Path "./rust/target/wasm32-unknown-unknown/release/mahnes_rs.gc.wasm" -Destination "./mahnes_rs.gc.wasm"

