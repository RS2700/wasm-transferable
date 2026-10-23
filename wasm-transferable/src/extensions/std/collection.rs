use std::{collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque}, hash::Hash};

use crate::{Deserialize, DeserializeError, Deserializer, Serialize, Serializer};

impl<T, const N: usize, E> Serialize<E> for [T; N]
where
    T: Serialize<E>
{
    fn serialize(self, ser: &mut Serializer) -> Result<(), E> {
        for ele in self {
            ele.serialize(ser)?;
        }

        Ok(())
    }
}

impl<T, const N: usize, E> Deserialize<E> for [T; N]
where
    T: Deserialize<E>
{
    fn deserialize(des: &mut Deserializer) -> Result<Self, E> {
        array_init::try_array_init(|_| {
            T::deserialize(des)
        })
    }
}

impl<T, E> Serialize<E> for Box<[T]>
where
    T: Serialize<E>
{
    fn serialize(self, ser: &mut Serializer) -> Result<(), E> {
        let len = self.len();
        ser.fixed_binary(len.to_be_bytes());

        for ele in self {
            ele.serialize(ser)?;
        }

        Ok(())
    }
}

impl<T, E> Deserialize<E> for Box<[T]>
where
    T: Deserialize<E>,
    E: From<DeserializeError>
{
    fn deserialize(des: &mut Deserializer) -> Result<Self, E> {
        let len_bytes = des.fixed_binary()?;
        let len = u32::from_be_bytes(len_bytes) as usize;

        std::iter::repeat_with(|| T::deserialize(des))
            .take(len)
            .collect()
    }
}

impl<T, E> Serialize<E> for Vec<T>
where
    T: Serialize<E>
{
    fn serialize(self, ser: &mut Serializer) -> Result<(), E> {
        let len = self.len() as u32;
        ser.fixed_binary(len.to_be_bytes());

        for ele in self {
            ele.serialize(ser)?;
        }

        Ok(())
    }
}

impl<T, E> Deserialize<E> for Vec<T>
where
    T: Deserialize<E>,
    E: From<DeserializeError>
{
    fn deserialize(des: &mut Deserializer) -> Result<Self, E> {
        let len_bytes = des.fixed_binary()?;
        let len = u32::from_be_bytes(len_bytes) as usize;

        std::iter::repeat_with(|| T::deserialize(des))
            .take(len)
            .collect()
    }
}

impl<T, E> Serialize<E> for VecDeque<T>
where
    T: Serialize<E>
{
    fn serialize(self, ser: &mut Serializer) -> Result<(), E> {
        let len = self.len() as u32;
        ser.fixed_binary(len.to_be_bytes());

        for ele in self {
            ele.serialize(ser)?;
        }

        Ok(())
    }
}

impl<T, E> Deserialize<E> for VecDeque<T>
where
    T: Deserialize<E>,
    E: From<DeserializeError>
{
    fn deserialize(des: &mut Deserializer) -> Result<Self, E> {
        let len_bytes = des.fixed_binary()?;
        let len = u32::from_be_bytes(len_bytes) as usize;

        std::iter::repeat_with(|| T::deserialize(des))
            .take(len)
            .collect()
    }
}

impl<T, E> Serialize<E> for LinkedList<T>
where
    T: Serialize<E>
{
    fn serialize(self, ser: &mut Serializer) -> Result<(), E> {
        let len = self.len() as u32;
        ser.fixed_binary(len.to_be_bytes());

        for ele in self {
            ele.serialize(ser)?;
        }

        Ok(())
    }
}

impl<T, E> Deserialize<E> for LinkedList<T>
where
    T: Deserialize<E>,
    E: From<DeserializeError>
{
    fn deserialize(des: &mut Deserializer) -> Result<Self, E> {
        let len_bytes = des.fixed_binary()?;
        let len = u32::from_be_bytes(len_bytes) as usize;

        std::iter::repeat_with(|| T::deserialize(des))
            .take(len)
            .collect()
    }
}

impl<K, V, E> Serialize<E> for HashMap<K, V>
where
    K: Serialize<E>,
    V: Serialize<E>
{
    fn serialize(self, ser: &mut Serializer) -> Result<(), E> {
        let len = self.len() as u32;
        ser.fixed_binary(len.to_be_bytes());

        for (k, v) in self {
            k.serialize(ser)?;
            v.serialize(ser)?;
        }

        Ok(())
    }
}

impl<K, V, E> Deserialize<E> for HashMap<K, V>
where
    K: Deserialize<E> + Eq + Hash,
    V: Deserialize<E>,
    E: From<DeserializeError>
{
    fn deserialize(des: &mut Deserializer) -> Result<Self, E> {
        let len_bytes = des.fixed_binary()?;
        let len = u32::from_be_bytes(len_bytes) as usize;

        std::iter::repeat_with(|| Ok((K::deserialize(des)?, V::deserialize(des)?)))
            .take(len)
            .collect()
    }
}

impl<K, V, E> Serialize<E> for BTreeMap<K, V>
where
    K: Serialize<E>,
    V: Serialize<E>
{
    fn serialize(self, ser: &mut Serializer) -> Result<(), E> {
        let len = self.len() as u32;
        ser.fixed_binary(len.to_be_bytes());

        for (k, v) in self {
            k.serialize(ser)?;
            v.serialize(ser)?;
        }

        Ok(())
    }
}

impl<K, V, E> Deserialize<E> for BTreeMap<K, V>
where
    K: Deserialize<E> + Ord,
    V: Deserialize<E>,
    E: From<DeserializeError>
{
    fn deserialize(des: &mut Deserializer) -> Result<Self, E> {
        let len_bytes = des.fixed_binary()?;
        let len = u32::from_be_bytes(len_bytes) as usize;

        std::iter::repeat_with(|| Ok((K::deserialize(des)?, V::deserialize(des)?)))
            .take(len)
            .collect()
    }
}

impl<T, E> Serialize<E> for HashSet<T>
where
    T: Serialize<E>
{
    fn serialize(self, ser: &mut Serializer) -> Result<(), E> {
        let len = self.len() as u32;
        ser.fixed_binary(len.to_be_bytes());

        for ele in self {
            ele.serialize(ser)?;
        }

        Ok(())
    }
}

impl<T, E> Deserialize<E> for HashSet<T>
where
    T: Deserialize<E> + Eq + Hash,
    E: From<DeserializeError>
{
    fn deserialize(des: &mut Deserializer) -> Result<Self, E> {
        let len_bytes = des.fixed_binary()?;
        let len = u32::from_be_bytes(len_bytes) as usize;

        std::iter::repeat_with(|| T::deserialize(des))
            .take(len)
            .collect()
    }
}

impl<T, E> Serialize<E> for BTreeSet<T>
where
    T: Serialize<E>
{
    fn serialize(self, ser: &mut Serializer) -> Result<(), E> {
        let len = self.len() as u32;
        ser.fixed_binary(len.to_be_bytes());

        for ele in self {
            ele.serialize(ser)?;
        }

        Ok(())
    }
}

impl<T, E> Deserialize<E> for BTreeSet<T>
where
    T: Deserialize<E> + Ord,
    E: From<DeserializeError>
{
    fn deserialize(des: &mut Deserializer) -> Result<Self, E> {
        let len_bytes = des.fixed_binary()?;
        let len = u32::from_be_bytes(len_bytes) as usize;

        std::iter::repeat_with(|| T::deserialize(des))
            .take(len)
            .collect()
    }
}

impl<T, E> Serialize<E> for BinaryHeap<T>
where
    T: Serialize<E>
{
    fn serialize(self, ser: &mut Serializer) -> Result<(), E> {
        let len = self.len() as u32;
        ser.fixed_binary(len.to_be_bytes());

        for ele in self {
            ele.serialize(ser)?;
        }

        Ok(())
    }
}

impl<T, E> Deserialize<E> for BinaryHeap<T>
where
    T: Deserialize<E> + Ord,
    E: From<DeserializeError>
{
    fn deserialize(des: &mut Deserializer) -> Result<Self, E> {
        let len_bytes = des.fixed_binary()?;
        let len = u32::from_be_bytes(len_bytes) as usize;

        std::iter::repeat_with(|| T::deserialize(des))
            .take(len)
            .collect()
    }
}
