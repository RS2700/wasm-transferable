use std::marker::PhantomData;

use futures::channel::mpsc;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use wasm_transferable::*;

use crate::utils::{CloseAwareMessageEvent, CloseAwareMessagePort, CloseAwareMessagePortListener};

#[derive(Transferable)]
#[transferable(ser)]
#[transferable(des(where_error(From<DeserializeError>)))]
pub struct WritableTransferStream<T> {
    _phantom: PhantomData<T>,
    port: CloseAwareMessagePort,
}

pub struct Controller<T, E>
where
    T: Deserialize<E>,
    E: From<DeserializerInitializationError>,
{
    _phantom_t: PhantomData<T>,
    _phantom_e: PhantomData<E>,
    listen_port: CloseAwareMessagePortListener,
    rx: mpsc::Receiver<JsValue>,
}

pub struct Writer<T, E>
where
    T: Serialize<E>,
{
    _phantom_t: PhantomData<T>,
    _phantom_e: PhantomData<E>,
    listen_port: CloseAwareMessagePortListener,
    pull_request_rx: mpsc::Receiver<()>,
}

#[derive(Debug)]
pub enum RecvError<D> {
    Deserialize(D),
    StreamClosed,
}

#[derive(Debug)]
pub enum WriteError<S> {
    Serialize(S),
    StreamClosed,
}

impl<T> WritableTransferStream<T> {
    pub fn new<F, E>(consumer: F) -> Self
    where
        T: Deserialize<E> + 'static,
        F: AsyncFnOnce(&mut Controller<T, E>) + 'static,
        E: From<DeserializerInitializationError> + 'static,
    {
        // sending -> recving: ストリームのアイテムを送る
        // recving -> sending: pullリクエストを送る
        let (sending, recving) = CloseAwareMessagePort::new_channel();

        let mut controller = Controller::new(recving);
        spawn_local(async move { consumer(&mut controller).await });

        Self {
            _phantom: PhantomData,
            port: sending,
        }
    }

    pub fn into_writer<E>(self) -> Writer<T, E>
    where
        T: Serialize<E>,
    {
        let Self { _phantom, port } = self;

        Writer::new(port)
    }
}

impl<T, E> Controller<T, E>
where
    T: Deserialize<E>,
    E: From<DeserializerInitializationError>,
{
    fn new(port: CloseAwareMessagePort) -> Self {
        let (mut tx, rx) = mpsc::channel(0);

        let listen_port = port.listen(move |e: CloseAwareMessageEvent| match e {
            CloseAwareMessageEvent::Message(x) => {
                _ = tx.try_send(x);
            }
            CloseAwareMessageEvent::Close => {
                tx.disconnect();
            }
        });

        Self {
            _phantom_t: PhantomData,
            _phantom_e: PhantomData,
            listen_port,
            rx,
        }
    }

    pub async fn recv(&mut self) -> Result<T, RecvError<E>> {
        // 受信を待機
        let js_value = self.rx.recv().await.map_err(|_| RecvError::StreamClosed)?;

        // 受信済みであることを通知
        self.listen_port.post_message(&JsValue::NULL);

        deserialize(js_value).map_err(RecvError::Deserialize)
    }
}

impl<T, E> Writer<T, E>
where
    T: Serialize<E>,
{
    fn new(port: CloseAwareMessagePort) -> Self {
        let (mut tx, rx) = mpsc::channel(0);

        let listen_port = port.listen(move |e: CloseAwareMessageEvent| match e {
            CloseAwareMessageEvent::Message(_) => {
                _ = tx.try_send(());
            }
            CloseAwareMessageEvent::Close => {
                tx.disconnect();
            }
        });

        Self {
            _phantom_t: PhantomData,
            _phantom_e: PhantomData,
            listen_port,
            pull_request_rx: rx,
        }
    }

    pub async fn write(&mut self, value: T) -> Result<(), WriteError<E>> {
        let SerializeResult { data, transfers } =
            serialize(value).map_err(WriteError::Serialize)?;

        self.listen_port
            .post_message_with_transferable(&data, transfers.as_ref());

        // pullリクエストを待機
        self.pull_request_rx
            .recv()
            .await
            .map_err(|_| WriteError::StreamClosed)
    }
}
