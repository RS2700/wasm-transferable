use wasm_bindgen::JsValue;

use crate::*;

impl<E> Serialize<E> for JsValue {
    fn serialize(self, ser: &mut Serializer) -> Result<(), E> {
        ser.js_value(&self);

        Ok(())
    }
}

impl<E> Deserialize<E> for JsValue
where
    E: From<DeserializeError>
{
    fn deserialize(des: &mut Deserializer) -> Result<Self, E> {
        des.js_value()
            .map_err(Into::into)
    }
}
