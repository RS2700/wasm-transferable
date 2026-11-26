use wasm_bindgen::JsValue;

use crate::SerializeResult;

pub trait DedicatedWorkerGlobalScopePostExt {
    fn post_transferable_message(&self, message: SerializeResult) -> Result<(), JsValue>;
}

impl DedicatedWorkerGlobalScopePostExt for web_sys::DedicatedWorkerGlobalScope {
    fn post_transferable_message(&self, message: SerializeResult) -> Result<(), JsValue> {
        let SerializeResult { data, transfers } = message;

        self.post_message_with_transfer(&data, transfers.as_ref())
    }
}
