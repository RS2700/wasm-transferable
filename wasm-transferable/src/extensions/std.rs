use std::panic::AssertUnwindSafe;

use crate::{Deserialize, DeserializeError, Deserializer, Serialize, Serializer};

mod cell;
mod collection;
mod marker;
mod num;
mod ops;
mod primitive;
mod range;
mod string;
mod sync;
mod time;
mod tuple;

impl<T, E> Serialize<E> for Box<T>
where
    T: Serialize<E>,
{
    fn serialize(self, ser: &mut Serializer) -> Result<(), E> {
        (*self).serialize(ser)
    }
}

impl<T, E> Deserialize<E> for Box<T>
where
    T: Deserialize<E>,
{
    fn deserialize(des: &mut Deserializer) -> Result<Self, E> {
        let inner = T::deserialize(des)?;
        Ok(Box::new(inner))
    }
}

impl<E, T> Serialize<E> for AssertUnwindSafe<T>
where
    T: Serialize<E>,
{
    fn serialize(self, ser: &mut Serializer) -> Result<(), E> {
        self.0.serialize(ser)
    }
}

impl<E, T> Deserialize<E> for AssertUnwindSafe<T>
where
    T: Deserialize<E>,
{
    fn deserialize(des: &mut Deserializer) -> Result<Self, E> {
        let inner = T::deserialize(des)?;
        Ok(Self(inner))
    }
}

impl<T, E> Serialize<E> for Option<T>
where
    T: Serialize<E>,
{
    fn serialize(self, ser: &mut Serializer) -> Result<(), E> {
        match self {
            Some(x) => {
                ser.transferable(true)?;
                ser.transferable(x)?;
            }
            None => {
                ser.transferable(false)?;
            }
        }

        Ok(())
    }
}

impl<T, E> Deserialize<E> for Option<T>
where
    T: Deserialize<E>,
    E: From<DeserializeError> + From<E>,
{
    fn deserialize(des: &mut Deserializer) -> Result<Self, E> {
        let is_some = des.transferable::<bool, E>()?;

        let x = if is_some {
            Some(des.transferable()?)
        } else {
            None
        };

        Ok(x)
    }
}

impl<T, E, SerError> Serialize<SerError> for Result<T, E>
where
    T: Serialize<SerError>,
    E: Serialize<SerError>,
{
    fn serialize(self, ser: &mut Serializer) -> Result<(), SerError> {
        match self {
            Ok(x) => {
                ser.transferable(true)?;
                ser.transferable(x)?;
            }
            Err(err) => {
                ser.transferable(false)?;
                ser.transferable(err)?;
            }
        }

        Ok(())
    }
}

impl<T, E, DesError> Deserialize<DesError> for Result<T, E>
where
    T: Deserialize<DesError>,
    E: Deserialize<DesError>,
    DesError: From<DeserializeError> + From<DesError>,
{
    fn deserialize(des: &mut Deserializer) -> Result<Self, DesError> {
        let is_ok = des.transferable::<bool, DesError>()?;

        let x = if is_ok {
            Ok(des.transferable()?)
        } else {
            Err(des.transferable()?)
        };

        Ok(x)
    }
}
