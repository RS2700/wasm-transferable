use std::marker::{PhantomData, PhantomPinned};

use crate::{Deserialize, Deserializer, Serialize, Serializer};

impl<T, E> Serialize<E> for PhantomData<T>
where
    T: ?Sized
{
    fn serialize(self, _ser: &mut Serializer) -> Result<(), E> {
        Ok(())
    }
}

impl<T, E> Deserialize<E> for PhantomData<T>
where
    T: ?Sized
{
    fn deserialize(_des: &mut Deserializer) -> Result<Self, E> {
        Ok(PhantomData)
    }
}

impl<E> Serialize<E> for PhantomPinned {
    fn serialize(self, _ser: &mut Serializer) -> Result<(), E> {
        Ok(())
    }
}

impl<E> Deserialize<E> for PhantomPinned {
    fn deserialize(_des: &mut Deserializer) -> Result<Self, E> {
        Ok(PhantomPinned)
    }
}
