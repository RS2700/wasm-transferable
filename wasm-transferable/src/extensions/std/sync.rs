use std::sync::*;

use crate::{Deserialize, DeserializeError, Deserializer, Serialize, Serializer};

mod atomic;

impl<T, E> Serialize<E> for OnceLock<T>
where
    T: Serialize<E>
{ 
    fn serialize(self, ser: &mut Serializer) -> Result<(), E> {
        let inner: Option<T> = self.into_inner();
        ser.transferable(inner)
    }
}

impl<T, E> Deserialize<E> for OnceLock<T>
where
    T: Deserialize<E>,
    E: From<DeserializeError> + From<E>
{
    fn deserialize(des: &mut Deserializer) -> Result<Self, E> {
        let inner: Option<T> = des.transferable()?;

        let x = Self::new();
        if let Some(inner) = inner {
            x.set(inner)
                .unwrap_or_else(|_| panic!());
        }
        Ok(x)
    }
}
