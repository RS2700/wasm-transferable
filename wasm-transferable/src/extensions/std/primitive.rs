use crate::{Deserialize, DeserializeError, Deserializer, Serialize, Serializer};

macro_rules! numeric {
    ($ty:ty) => {
        impl<E> Serialize<E> for $ty {
            fn serialize(self, ser: &mut Serializer) -> Result<(), E> {
                ser.fixed_binary(self.to_be_bytes());
                Ok(())
            }
        }

        impl<E> Deserialize<E> for $ty
        where
            E: From<DeserializeError>
        {
            fn deserialize(des: &mut Deserializer) -> Result<Self, E> {
                let bin = des.fixed_binary()?;
                let x = Self::from_be_bytes(bin);
                Ok(x)
            }
        }
    };
}

numeric!(f32);
numeric!(f64);

numeric!(i8);
numeric!(i16);
numeric!(i32);
numeric!(i64);
numeric!(i128);

numeric!(u8);
numeric!(u16);
numeric!(u32);
numeric!(u64);
numeric!(u128);

impl<E> Serialize<E> for bool {
    fn serialize(self, ser: &mut Serializer) -> Result<(), E> {
        ser.byte(self as u8);
        Ok(())
    }
}

impl<E> Deserialize<E> for bool
where
    E: From<DeserializeError>
{
    fn deserialize(des: &mut Deserializer) -> Result<Self, E> {
        let x = des.byte()?;
        Ok(x != 0)
    }
}

impl<E> Serialize<E> for char {
    fn serialize(self, ser: &mut Serializer) -> Result<(), E> {
        Serialize::serialize(self as u32, ser)
    }
}

impl<E> Deserialize<E> for char
where
    E: From<DeserializeError>
{
    fn deserialize(des: &mut Deserializer) -> Result<Self, E> {
        let n: u32 = Deserialize::deserialize(des)?;
        char::from_u32(n)
            .ok_or(DeserializeError::Format.into())
    }
}
