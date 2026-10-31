use std::ops::Bound;

use crate::{Deserialize, DeserializeError, Deserializer, Serialize, Serializer};

impl<E, T> Serialize<E> for Bound<T>
where
    T: Serialize<E>,
{
    fn serialize(self, ser: &mut Serializer) -> Result<(), E> {
        match self {
            Self::Included(x) => {
                ser.transferable(0u8)?;
                ser.transferable(x)?;
            }
            Self::Excluded(x) => {
                ser.transferable(1u8)?;
                ser.transferable(x)?;
            }
            Self::Unbounded => {
                ser.transferable(2u8)?;
            }
        }

        Ok(())
    }
}

impl<E, T> Deserialize<E> for Bound<T>
where
    T: Deserialize<E>,
    E: From<DeserializeError> + From<E>,
{
    fn deserialize(des: &mut Deserializer) -> Result<Self, E> {
        let n: u8 = des.transferable::<u8, E>()?;
        let x = match n {
            0 => Self::Included(des.transferable()?),
            1 => Self::Excluded(des.transferable()?),
            2 => Self::Unbounded,
            _ => return Err(DeserializeError::Format.into()),
        };

        Ok(x)
    }
}
