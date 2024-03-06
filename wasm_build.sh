 cargo build --target wasm32-unknown-unknown --release
 wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/cat_ball_wow_bevy.wasm
 cp index.html out
 cp -R assets out
