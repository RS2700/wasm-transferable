use wasm_bindgen::JsCast;

use crate::*;

pub enum Error {
    TypeMismatch
}

impl<E, T> Serialize<E> for T
where
    T: JsCast
{
    fn serialize(self, ser: &mut Serializer) -> Result<(), E> {
        let js_value = self.unchecked_ref();

        ser.js_value(js_value);

        Ok(())
    }
}

impl<E, T> Deserialize<E> for T
where
    E: From<DeserializeError> + From<Error>,
    T: JsCast
{
    fn deserialize(des: &mut Deserializer) -> Result<Self, E> {
        let js_value = des.js_value()?;

        js_value.dyn_into()
            .map_err(|_| Error::TypeMismatch.into())
    }
}
