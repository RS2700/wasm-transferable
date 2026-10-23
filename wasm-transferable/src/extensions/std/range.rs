use std::range::*;

use crate::{Deserialize, Deserializer, Serialize, Serializer};

mod legacy;

impl<E, Idx> Serialize<E> for Range<Idx>
where
    Idx: Serialize<E>
{
    fn serialize(self, ser: &mut Serializer) -> Result<(), E> {
        let Self {
            start,
            end,
        } = self;

        ser.transferable(start)?;
        ser.transferable(end)?;

        Ok(())
    }
}

impl<E, Idx> Deserialize<E> for Range<Idx>
where
    Idx: Deserialize<E>
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
    Idx: Serialize<E>
{
    fn serialize(self, ser: &mut Serializer) -> Result<(), E> {
        let Self {
            start
        } = self;

        ser.transferable(start)
    }
}

impl<E, Idx> Deserialize<E> for RangeFrom<Idx>
where
    Idx: Deserialize<E>
{
    fn deserialize(des: &mut Deserializer) -> Result<Self, E> {
        let x = Self {
            start: des.transferable()?
        };

        Ok(x)
    }
}

impl<E> Serialize<E> for RangeFull {
    fn serialize(self, _ser: &mut Serializer) -> Result<(), E> {
        Ok(())
    }
}

impl<E> Deserialize<E> for RangeFull {
    fn deserialize(_des: &mut Deserializer) -> Result<Self, E> {
        Ok(Self)
    }
}

impl<E, Idx> Serialize<E> for RangeInclusive<Idx>
where
    Idx: Serialize<E>
{
    fn serialize(self, ser: &mut Serializer) -> Result<(), E> {
        let Self {
            start,
            last,
        } = self;

        ser.transferable(start)?;
        ser.transferable(last)?;

        Ok(())
    }
}

impl<E, Idx> Deserialize<E> for RangeInclusive<Idx>
where
    Idx: Deserialize<E>
{
    fn deserialize(des: &mut Deserializer) -> Result<Self, E> {
        let x = Self {
            start: des.transferable()?,
            last: des.transferable()?,
        };

        Ok(x)
    }
}

impl<E, Idx> Serialize<E> for RangeTo<Idx>
where
    Idx: Serialize<E>
{
    fn serialize(self, ser: &mut Serializer) -> Result<(), E> {
        let Self {
            end,
        } = self;

        ser.transferable(end)
    }
}

impl<E, Idx> Deserialize<E> for RangeTo<Idx>
where
    Idx: Deserialize<E>
{
    fn deserialize(des: &mut Deserializer) -> Result<Self, E> {
        let x = Self {
            end: des.transferable()?,
        };

        Ok(x)
    }
}

impl<E, Idx> Serialize<E> for RangeToInclusive<Idx>
where
    Idx: Serialize<E>
{
    fn serialize(self, ser: &mut Serializer) -> Result<(), E> {
        let Self {
            last,
        } = self;

        ser.transferable(last)
    }
}

impl<E, Idx> Deserialize<E> for RangeToInclusive<Idx>
where
    Idx: Deserialize<E>
{
    fn deserialize(des: &mut Deserializer) -> Result<Self, E> {
        let x = Self {
            last: des.transferable()?,
        };

        Ok(x)
    }
}
