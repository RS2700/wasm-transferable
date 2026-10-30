# wasm-transferable

Rustの構造体を `JsValue` へシリアライズすると同時に、それに含まれる [`Transferable Objects`](https://developer.mozilla.org/docs/Web/API/Web_Workers_API/Transferable_objects) を収集するフレームワーク。

> [!WARNING]
> **現在、本ライブラリはプロトタイプの段階です。**
> 
> 計画中の全ての機能が実装されていますが、予期せぬ不具合が発生する可能性があり、後方互換性は保証されていません。
> 
> 本番環境での使用は推奨しません。

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
