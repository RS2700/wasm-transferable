use std::time::*;

use crate::{Deserialize, DeserializeError, Deserializer, Serialize, Serializer};

impl<E> Serialize<E> for Duration {
    fn serialize(self, ser: &mut Serializer) -> Result<(), E> {
        ser.transferable(self.as_secs())?;
        ser.transferable(self.subsec_nanos())?;

        Ok(())
    }
}

impl<E> Deserialize<E> for Duration
where
    E: From<DeserializeError>,
{
    fn deserialize(des: &mut Deserializer) -> Result<Self, E> {
        let x = Self::new(des.transferable()?, des.transferable()?);
        Ok(x)
    }
}
