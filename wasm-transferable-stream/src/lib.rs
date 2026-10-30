//! A [`wasm_transferable`] ecosystem for streaming Transferable Objects with a Web Stream API-like interface.
//! 
//! <div class="warning">
//! <p><strong>WARNING:</strong> This library is currently in a prototype stage.</p>
//! <p>
//! While all planned features have been implemented, unexpected bugs may occur, and backwards compatibility is not guaranteed.<br/>
//! Use in production environments is not recommended.
//! </p>
//! </div>

mod utils;

pub mod readable;
pub mod writable;

pub use readable::ReadableTransferStream;
pub use writable::WritableTransferStream;
