use wasm_bindgen::JsValue;

use crate::SerializeResult;

pub trait MessagePortPostExt {
    fn post_transferable_message(&self, message: SerializeResult) -> Result<(), JsValue>;
}

impl MessagePortPostExt for web_sys::MessagePort {
    fn post_transferable_message(&self, message: SerializeResult) -> Result<(), JsValue> {
        let SerializeResult { data, transfers } = message;

        self.post_message_with_transferable(&data, transfers.as_ref())
    }
}
