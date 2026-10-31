use wasm_bindgen::JsValue;

use crate::SerializeResult;

pub trait WorkerPostExt {
    fn post_transferable_message(&self, message: SerializeResult) -> Result<(), JsValue>;
}

impl WorkerPostExt for web_sys::Worker {
    fn post_transferable_message(&self, message: SerializeResult) -> Result<(), JsValue> {
        let SerializeResult { data, transfers } = message;

        self.post_message_with_transfer(&data, transfers.as_ref())
    }
}
