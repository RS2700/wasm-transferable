# wasm-transferable

A framework that serializes Rust structs into `JsValue`
while simultaneously collecting any contained [`Transferable Objects`](https://developer.mozilla.org/docs/Web/API/Web_Workers_API/Transferable_objects).

> [!WARNING]
> **This library is currently in a prototype stage.**
> 
> While all planned features have been implemented, unexpected bugs may occur, and backwards compatibility is not guaranteed.
> 
> Use in production environments is not recommended.

## features
### derive

Provides the `Transferable` derive macro.

### std-ext

Implements the `Serialize` and `Deserialize` traits for standard library (std) types.

### wasm-bindgen-ext

Implements the `Serialize` and `Deserialize` traits for types provided by the `wasm-bindgen` crate.

### MessagePort-post-ext

Provides an extension trait to post a `SerializeResult` directly to a `web_sys::MessagePort`.

### Window-post-ext

Provides an extension trait to post a `SerializeResult` directly to a `web_sys::Window`.

### Worker-post-ext

Provides an extension trait to post a `SerializeResult` directly to a `web_sys::Worker`.
