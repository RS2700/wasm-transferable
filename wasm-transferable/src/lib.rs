use std::vec;

use js_sys::{Array, ArrayIntoIter, Uint8Array};
use wasm_bindgen::{JsCast, JsValue};

pub mod extensions;

#[cfg(feature = "derive")]
pub use wasm_transferable_macro::Transferable;

pub struct SerializeResult {
    pub data: JsValue,
    pub transfers: Array
}

pub enum DeserializerInitializationError {
    Format
}

pub enum DeserializeError {
    Format
}

pub struct Serializer {
    binary: Vec<u8>,
    values: Array,
    transfers: Array
}

impl Serializer {
    pub fn new() -> Self {
        Self {
            binary: Vec::new(),
            values: Array::new(),
            transfers: Array::new(),
        }
    }

    pub fn end(self) -> SerializeResult {
        let Self {
            binary,
            values,
            transfers,
        } = self;

        // バイナリをUint8Arrayにして、valuesの先頭に追加
        let binary_buf = Uint8Array::new_from_slice(&binary);
        values.unshift(&binary_buf);
        transfers.push(&binary_buf.buffer());

        SerializeResult {
            data: values.into(),
            transfers,
        }
    }
}

impl Serializer {
    pub fn transfer(&mut self, value: JsValue) {
        self.transfers.push(&value);
    }
    
    pub fn js_value(&mut self, value: &JsValue) {
        self.values.push(&value);
    }

    pub fn transferable<T, E>(&mut self, value: T) -> Result<(), E>
    where
        T: Serialize<E>
    {
        value.serialize(self)
    }

    pub fn byte(&mut self, byte: u8) {
        self.binary.push(byte);
    }

    pub fn fixed_binary<const N: usize>(&mut self, bin: [u8; N]) {
        self.binary.extend(bin);
    }
}

pub struct Deserializer {
    binary: vec::IntoIter<u8>,
    values: ArrayIntoIter<JsValue>
}

impl Deserializer {
    pub fn new(data: JsValue) -> Result<Self, DeserializerInitializationError> {
        let message: Array = data.dyn_into()
            .map_err(|_| DeserializerInitializationError::Format)?;

        let mut values = message.into_iter();

        // 先頭にある、シリアライズされたvaluesのUInt8Arrayを引き出す
        let binary_buf = values.next()
            .ok_or(DeserializerInitializationError::Format)?;
        let binary_buf: Uint8Array = binary_buf.dyn_into()
            .map_err(|_| DeserializerInitializationError::Format)?;

        let binary = binary_buf.to_vec().into_iter();

        Ok(Self {
            binary,
            values
        })
    }
}

impl Deserializer {
    pub fn js_value(&mut self) -> Result<JsValue, DeserializeError> {
        self.values.next()
            .ok_or_else(|| DeserializeError::Format)
    }

    pub fn transferable<T, E>(&mut self) -> Result<T, E>
    where
        T: Deserialize<E>
    {
        T::deserialize(self)
    }

    pub fn byte(&mut self) -> Result<u8, DeserializeError> {
        self.binary.next()
            .ok_or(DeserializeError::Format)
    }

    pub fn fixed_binary<const N: usize>(&mut self) -> Result<[u8; N], DeserializeError> {
        array_init::from_iter(self.binary.by_ref())
            .ok_or(DeserializeError::Format)
    }
}

pub trait Serialize<E> {
    fn serialize(self, ser: &mut Serializer) -> Result<(), E>;
}

pub trait Deserialize<E>
where
    Self: Sized
{
    fn deserialize(des: &mut Deserializer) -> Result<Self, E>;
}

pub fn serialize<T, E>(value: T) -> Result<SerializeResult, E>
where
    T: Serialize<E>
{
    let mut ser = Serializer::new();
    ser.transferable(value)?;
    Ok(ser.end())
}

pub fn deserialize<T, E>(data: JsValue) -> Result<T, E>
where
    T: Deserialize<E>,
    E: From<DeserializerInitializationError>
{
    let mut des = Deserializer::new(data)?;
    let value = des.transferable::<T, E>()?;
    Ok(value)
}
