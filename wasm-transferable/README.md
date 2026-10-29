# wasm-transferable

Rustの構造体を `JsValue` へシリアライズすると同時に、それに含まれる [`Transferable Objects`](https://developer.mozilla.org/docs/Web/API/Web_Workers_API/Transferable_objects) を収集するフレームワーク。

## features
### derive

`Transferable` deriveマクロを提供します。

### std-ext

`std` ライブラリの型に、`Serialize`, `Deserialize` トレイトを実装します。

### wasm-bindgen-ext

`wasm-bindgen` クレートの型に、`Serialize`, `Deserialize` トレイトを実装します。

### MessagePort-post-ext

`web_sys::MessagePort` に、直接 `SerializeResult` をポストできる拡張トレイトを実装します。

### Window-post-ext

`web_sys::Window` に、直接 `SerializeResult` をポストできる拡張トレイトを実装します。

### Worker-post-ext

`web_sys::Worker` に、直接 `SerializeResult` をポストできる拡張トレイトを実装します。
