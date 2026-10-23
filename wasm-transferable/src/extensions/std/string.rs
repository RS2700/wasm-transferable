use crate::{Deserialize, DeserializeError, Deserializer, Serialize, Serializer};

impl<E> Serialize<E> for String {
    fn serialize(self, ser: &mut Serializer) -> Result<(), E> {
        let bytes: Vec<u8> = self.into_bytes();
        ser.transferable(bytes)?;

        Ok(())
    }
}

impl<E> Deserialize<E> for String
where
    E: From<DeserializeError>
{
    fn deserialize(des: &mut Deserializer) -> Result<Self, E> {
        let bytes: Vec<u8> = des.transferable()?;
        Self::from_utf8(bytes)
            .map_err(|_| DeserializeError::Format.into())
    }
}

impl<E> Serialize<E> for Box<str>
{ 
    fn serialize(self, ser: &mut Serializer) -> Result<(), E> {
        let bytes: Box<[u8]> = self.into_boxed_bytes();
        ser.transferable(bytes)?;

        Ok(())
    }
}

impl<E> Deserialize<E> for Box<str>
where
    E: From<DeserializeError>
{
    fn deserialize(des: &mut Deserializer) -> Result<Self, E> {
        let bytes: Box<[u8]> = des.transferable()?;
        let string = String::from_utf8(bytes.into_vec())
            .map_err(|_| DeserializeError::Format)?;
        
        Ok(string.into_boxed_str())
    }
}
