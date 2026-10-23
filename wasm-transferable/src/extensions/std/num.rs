use std::num::*;

use crate::{Deserialize, DeserializeError, Deserializer, Serialize, Serializer};

macro_rules! non_zero {
    ($ty:ty) => {
        impl<E> Serialize<E> for $ty {
            fn serialize(self, ser: &mut Serializer) -> Result<(), E> {
                Serialize::serialize(self.get(), ser)
            }
        }

        impl<E> Deserialize<E> for $ty
        where
            E: From<DeserializeError>
        {
            fn deserialize(des: &mut Deserializer) -> Result<Self, E> {
                let inner = Deserialize::deserialize(des)?;
                NonZero::new(inner)
                    .ok_or(DeserializeError::Format.into())
            }
        }
    };
}

non_zero!(NonZeroI8);
non_zero!(NonZeroI16);
non_zero!(NonZeroI32);
non_zero!(NonZeroI64);
non_zero!(NonZeroI128);

non_zero!(NonZeroU8);
non_zero!(NonZeroU16);
non_zero!(NonZeroU32);
non_zero!(NonZeroU64);
non_zero!(NonZeroU128);

impl<E, T> Serialize<E> for Saturating<T>
where
    T: Serialize<E>
{
    fn serialize(self, ser: &mut Serializer) -> Result<(), E> {
        T::serialize(self.0, ser)
    }
}

impl<E, T> Deserialize<E> for Saturating<T>
where
    T: Deserialize<E>
{
    fn deserialize(des: &mut Deserializer) -> Result<Self, E> {
        let inner = T::deserialize(des)?;
        Ok(Self(inner))
    }
}

impl<E, T> Serialize<E> for Wrapping<T>
where
    T: Serialize<E>
{
    fn serialize(self, ser: &mut Serializer) -> Result<(), E> {
        T::serialize(self.0, ser)
    }
}

impl<E, T> Deserialize<E> for Wrapping<T>
where
    T: Deserialize<E>
{
    fn deserialize(des: &mut Deserializer) -> Result<Self, E> {
        let inner = T::deserialize(des)?;
        Ok(Self(inner))
    }
}

impl<E> Serialize<E> for FpCategory {
    fn serialize(self, ser: &mut Serializer) -> Result<(), E> {
        let n: u8 = match self {
            Self::Nan => 0,
            Self::Infinite => 1,
            Self::Zero => 2,
            Self::Subnormal => 3,
            Self::Normal => 4,
        };

        ser.transferable(n)
    }
}

impl<E> Deserialize<E> for FpCategory
where
    E: From<DeserializeError>
{
    fn deserialize(des: &mut Deserializer) -> Result<Self, E> {
        let n: u8 = des.transferable()?;

        let x = match n {
            0 => Self::Nan,
            1 => Self::Infinite,
            2 => Self::Zero,
            3 => Self::Subnormal,
            4 => Self::Normal,
            _ => return Err(DeserializeError::Format.into())
        };

        Ok(x)
    }
}
