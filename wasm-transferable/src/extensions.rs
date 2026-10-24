#[cfg(feature = "std-ext")]
mod std;

cfg_select! {
    feature = "wasm-bindgen-ext" => {
        pub mod wasm_bindgen;
    }
    _ => {}
}
