use std::convert::Infallible;

use js_sys::Array;
use thiserror::Error;
use wasm_bindgen::prelude::*;
use wasm_transferable::{
    DeserializeError, DeserializerInitializationError, Transferable, deserialize,
    post_ext::MessagePortPostExt, serialize,
};
use web_sys::{MessageChannel, MessageEvent, MessagePort};

#[derive(Debug, Error)]
enum AnyDeserializeError {
    #[error("initialization error ({0})")]
    Initialization(#[from] DeserializerInitializationError),
    #[error("deserialize error ({0})")]
    Deserialize(#[from] DeserializeError),
}

#[derive(Transferable)]
#[transferable(ser)]
#[transferable(des(where_error(From<DeserializeError>)))]
pub enum CloseAwareMessageEvent {
    Message(JsValue),
    Close,
}

pub struct CloseAwareMessagePort {
    inner: MessagePort,
}

impl CloseAwareMessagePort {
    pub fn new_channel() -> (Self, Self) {
        let channel = MessageChannel::new().unwrap();

        (Self::new(channel.port1()), Self::new(channel.port2()))
    }

    fn new(inner: MessagePort) -> Self {
        Self { inner }
    }

    pub fn listen<F>(self, mut onmessage: F) -> CloseAwareMessagePortListener
    where
        F: FnMut(CloseAwareMessageEvent) + 'static,
    {
        let Self { inner } = self;

        let onmessage = Closure::new(move |e: MessageEvent| {
            let data: CloseAwareMessageEvent =
                deserialize::<_, AnyDeserializeError>(e.data()).unwrap();

            onmessage(data)
        });

        inner.set_onmessage(Some(onmessage.as_js_value().unchecked_ref()));

        CloseAwareMessagePortListener {
            inner,
            _onmessage: Some(onmessage),
        }
    }
}

impl<E> wasm_transferable::Serialize<E> for CloseAwareMessagePort {
    fn serialize(self, ser: &mut wasm_transferable::Serializer) -> Result<(), E> {
        let inner = self.inner.into();
        ser.js_value(&inner);
        ser.transfer(&inner);

        Ok(())
    }
}

impl<E> wasm_transferable::Deserialize<E> for CloseAwareMessagePort
where
    E: From<DeserializeError>,
{
    fn deserialize(des: &mut wasm_transferable::Deserializer) -> Result<Self, E> {
        let inner = des.js_value()?;
        let inner: MessagePort = inner.dyn_into().map_err(|_| DeserializeError::Format)?;

        Ok(Self { inner })
    }
}

/// Dropされたら、ポートを閉じます。
pub struct CloseAwareMessagePortListener {
    inner: MessagePort,
    _onmessage: Option<Closure<dyn FnMut(MessageEvent)>>,
}

impl CloseAwareMessagePortListener {
    /*
        pub fn set_onmessage<F>(&mut self, onmessage: Option<F>)
        where
            F: FnMut(CloseAwareMessageEvent) + 'static
        {
            let onmessage = onmessage.map(|mut onmessage| Closure::new(move |e: MessageEvent| {
                let data: CloseAwareMessageEvent = serde_wasm_bindgen::from_value(e.data())
                    .unwrap();

                onmessage(data)
            }));

            self.onmessage = onmessage;
        }
    */
    pub fn post_message(&self, message: &JsValue) {
        let message = CloseAwareMessageEvent::Message(message.clone());
        let message = serialize::<_, Infallible>(message).unwrap_or_else(|x| match x {});

        self.inner.post_transferable_message(message).unwrap()
    }

    pub fn post_message_with_transferable(&self, message: &JsValue, transferable: &Array) {
        let message = CloseAwareMessageEvent::Message(message.clone());
        let mut message = serialize::<_, Infallible>(message).unwrap_or_else(|x| match x {});

        message.transfers.extend(transferable.iter());

        self.inner.post_transferable_message(message).unwrap()
    }
}

impl Drop for CloseAwareMessagePortListener {
    fn drop(&mut self) {
        // Closeを通知
        let message = serialize::<_, Infallible>(CloseAwareMessageEvent::Close)
            .unwrap_or_else(|x| match x {});
        self.inner.post_transferable_message(message).unwrap();

        self.inner.close();
    }
}
