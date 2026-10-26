use wasm_bindgen::JsValue;

use crate::SerializeResult;

pub trait WindowPostExt {
    fn post_transferable_message(&self, message: SerializeResult, target_origin: &str) -> Result<(), JsValue>;
}

impl WindowPostExt for web_sys::Window {
    fn post_transferable_message(&self, message: SerializeResult, target_origin: &str) -> Result<(), JsValue> {
        let SerializeResult {
            data,
            transfers,
        } = message;

        self.post_message_with_transfer(&data, target_origin, transfers.as_ref())
    }
}
