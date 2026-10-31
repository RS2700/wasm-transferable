use std::range::legacy::*;

use crate::{Deserialize, Deserializer, Serialize, Serializer};

impl<E, Idx> Serialize<E> for Range<Idx>
where
    Idx: Serialize<E>,
{
    fn serialize(self, ser: &mut Serializer) -> Result<(), E> {
        let Self { start, end } = self;

        ser.transferable(start)?;
        ser.transferable(end)?;

        Ok(())
    }
}

impl<E, Idx> Deserialize<E> for Range<Idx>
where
    Idx: Deserialize<E>,
{
    fn deserialize(des: &mut Deserializer) -> Result<Self, E> {
        let x = Self {
            start: des.transferable()?,
            end: des.transferable()?,
        };

        Ok(x)
    }
}

impl<E, Idx> Serialize<E> for RangeFrom<Idx>
where
    Idx: Serialize<E>,
{
    fn serialize(self, ser: &mut Serializer) -> Result<(), E> {
        let Self { start } = self;

        ser.transferable(start)
    }
}

impl<E, Idx> Deserialize<E> for RangeFrom<Idx>
where
    Idx: Deserialize<E>,
{
    fn deserialize(des: &mut Deserializer) -> Result<Self, E> {
        let x = Self {
            start: des.transferable()?,
        };

        Ok(x)
    }
}

impl<E, Idx> Serialize<E> for RangeInclusive<Idx>
where
    Idx: Serialize<E>,
{
    fn serialize(self, ser: &mut Serializer) -> Result<(), E> {
        let (start, end) = self.into_inner();

        ser.transferable(start)?;
        ser.transferable(end)?;

        Ok(())
    }
}

impl<E, Idx> Deserialize<E> for RangeInclusive<Idx>
where
    Idx: Deserialize<E>,
{
    fn deserialize(des: &mut Deserializer) -> Result<Self, E> {
        let x = Self::new(des.transferable()?, des.transferable()?);

        Ok(x)
    }
}

impl<E, Idx> Serialize<E> for RangeToInclusive<Idx>
where
    Idx: Serialize<E>,
{
    fn serialize(self, ser: &mut Serializer) -> Result<(), E> {
        let Self { end } = self;

        ser.transferable(end)
    }
}

impl<E, Idx> Deserialize<E> for RangeToInclusive<Idx>
where
    Idx: Deserialize<E>,
{
    fn deserialize(des: &mut Deserializer) -> Result<Self, E> {
        let x = Self {
            end: des.transferable()?,
        };

        Ok(x)
    }
}
