use std::sync::atomic::*;

use crate::{Deserialize, DeserializeError, Deserializer, Serialize, Serializer};

macro_rules! atomic {
    ($ty:ty$(, $($des_bound:tt)*)?) => {
        impl<E> Serialize<E> for $ty {
            fn serialize(self, ser: &mut Serializer) -> Result<(), E> {
                let inner = self.into_inner();
                ser.transferable(inner)
            }
        }

        impl<E> Deserialize<E> for $ty
        where
            E: From<E>$( + $($des_bound)*)?
        {
            fn deserialize(des: &mut Deserializer) -> Result<Self, E> {
                let inner = des.transferable::<_, E>()?;
                Ok(Self::new(inner))
            }
        }
    };
}

atomic!(AtomicBool, From<DeserializeError>);

atomic!(AtomicI8, From<DeserializeError>);
atomic!(AtomicI16, From<DeserializeError>);
atomic!(AtomicI32, From<DeserializeError>);
atomic!(AtomicI64, From<DeserializeError>);

atomic!(AtomicU8, From<DeserializeError>);
atomic!(AtomicU16, From<DeserializeError>);
atomic!(AtomicU32, From<DeserializeError>);
atomic!(AtomicU64, From<DeserializeError>);
