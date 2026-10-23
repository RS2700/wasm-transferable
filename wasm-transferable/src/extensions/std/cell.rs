use std::cell::{Cell, OnceCell, UnsafeCell};

use crate::{Deserialize, DeserializeError, Deserializer, Serialize, Serializer};

impl<T, E> Serialize<E> for Cell<T>
where
    T: Serialize<E>
{ 
    fn serialize(self, ser: &mut Serializer) -> Result<(), E> {
        self.into_inner().serialize(ser)
    }
}

impl<T, E> Deserialize<E> for Cell<T>
where
    T: Deserialize<E>
{
    fn deserialize(des: &mut Deserializer) -> Result<Self, E> {
        let inner = T::deserialize(des)?;
        Ok(Self::new(inner))
    }
}

impl<T, E> Serialize<E> for OnceCell<T>
where
    T: Serialize<E>
{ 
    fn serialize(self, ser: &mut Serializer) -> Result<(), E> {
        Option::<T>::serialize(self.into_inner(), ser)
    }
}

impl<T, E> Deserialize<E> for OnceCell<T>
where
    T: Deserialize<E>,
    E: From<DeserializeError>
{
    fn deserialize(des: &mut Deserializer) -> Result<Self, E> {
        let cell = Self::new();
        if let Some(inner) = Option::<T>::deserialize(des)? {
            cell.set(inner)
                .unwrap_or_else(|_| panic!());
        }
        Ok(cell)
    }
}

impl<T, E> Serialize<E> for UnsafeCell<T>
where
    T: Serialize<E>
{ 
    fn serialize(self, ser: &mut Serializer) -> Result<(), E> {
        self.into_inner().serialize(ser)
    }
}

impl<T, E> Deserialize<E> for UnsafeCell<T>
where
    T: Deserialize<E>
{
    fn deserialize(des: &mut Deserializer) -> Result<Self, E> {
        let inner = T::deserialize(des)?;
        Ok(Self::new(inner))
    }
}
