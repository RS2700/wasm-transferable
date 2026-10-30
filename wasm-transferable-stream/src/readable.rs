use std::{convert::Infallible, marker::PhantomData, pin::Pin, task::Poll, };

use futures::{Stream, StreamExt, channel::mpsc, stream::{self, LocalBoxStream}};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use wasm_transferable::*;

use crate::utils::{CloseAwareMessageEvent, CloseAwareMessagePort, CloseAwareMessagePortListener};

#[derive(Transferable)]
#[transferable(ser)]
#[transferable(des(where_error(From<DeserializeError>)))]
pub struct ReadableTransferStream<T> {
    _phantom: PhantomData<T>,
    port: CloseAwareMessagePort
}

pub struct Controller<T, E>
where
    T: Serialize<E>
{
    _phantom_t: PhantomData<T>,
    _phantom_e: PhantomData<E>,
    listen_port: CloseAwareMessagePortListener,
    pull_request_rx: mpsc::Receiver<()>
}

pub struct Reader<T, E>
where
    T: Deserialize<E>,
    E: From<DeserializerInitializationError>
{
    _phantom_t: PhantomData<T>,
    _phantom_e: PhantomData<E>,
    listen_port: CloseAwareMessagePortListener,
    rx: mpsc::Receiver<JsValue>
}

pub struct ReadStream<'a, T, E>(LocalBoxStream<'a, Result<T, E>>)
where
    T: Deserialize<E> + 'a,
    E: From<DeserializerInitializationError> + 'a;

#[derive(Debug)]
pub enum SendError<S> {
    Serialize(S),
    StreamClosed
}

#[derive(Debug)]
pub enum ReadError<D> {
    Deserialize(D),
    StreamClosed
}

impl<T> ReadableTransferStream<T> {
    pub fn new<'a, F, E>(producer: F) -> Self
    where
        T: Serialize<E> + 'static,
        F: AsyncFnOnce(&mut Controller<T, E>) + 'static,
        E: 'static
    {
        // sending -> recving: ストリームのアイテムを送る
        // recving -> sending: pullリクエストを送る
        let (sending, recving) = CloseAwareMessagePort::new_channel();

        let mut controller = Controller::new(sending);
        spawn_local(async move {
            producer(&mut controller).await
        });

        Self {
            _phantom: PhantomData,
            port: recving,
        }
    }

    pub fn into_reader<E>(self) -> Reader<T, E>
    where
        T: Deserialize<E>,
        E: From<DeserializerInitializationError>
    {
        let Self {
            _phantom,
            port,
        } = self;

        Reader::new(port)
    }
}

impl<T, E> Controller<T, E>
where
    T: Serialize<E>
{
    fn new(port: CloseAwareMessagePort) -> Self {
        let (mut tx, rx) = mpsc::channel(0);

        let listen_port = port.listen(move |e: CloseAwareMessageEvent| {
            match e {
                CloseAwareMessageEvent::Message(_) => {
                    _ = tx.try_send(());
                },
                CloseAwareMessageEvent::Close => {
                    tx.disconnect();
                },
            }
        });

        Controller {
            _phantom_t: PhantomData,
            _phantom_e: PhantomData,
            listen_port,
            pull_request_rx: rx,
        }
    }

    pub async fn send(&mut self, value: T) -> Result<(), SendError<E>> {
        let SerializeResult {
            data,
            transfers,
        } = serialize(value).map_err(SendError::Serialize)?;

        self.listen_port.post_message_with_transferable(&data, transfers.as_ref());

        // pullリクエストを待機
        self.pull_request_rx.recv().await
            .map_err(|_| SendError::StreamClosed)
    }
}

impl<T, E> Reader<T, E>
where
    T: Deserialize<E>,
    E: From<DeserializerInitializationError>
{
    fn new(port: CloseAwareMessagePort) -> Self {
        let (mut tx, rx) = mpsc::channel(0);

        let listen_port = port.listen(move |e: CloseAwareMessageEvent| {
            match e {
                CloseAwareMessageEvent::Message(x) => {
                    _ = tx.try_send(x);
                },
                CloseAwareMessageEvent::Close => {
                    tx.disconnect();
                },
            }
        });

        Self {
            _phantom_t: PhantomData,
            _phantom_e: PhantomData,
            listen_port,
            rx,
        }
    }

    pub async fn read(&mut self) -> Result<T, ReadError<E>> {
        // 受信
        let js_value = self.rx.recv().await
            .map_err(|_| ReadError::StreamClosed)?;

        // 受信済みであると通知
        self.listen_port.post_message(&JsValue::NULL);

        deserialize(js_value)
            .map_err(ReadError::Deserialize)
    }
}

impl<S, T> From<S> for ReadableTransferStream<T>
where
    S: Stream<Item = T> + Unpin + 'static,
    T: Serialize<Infallible> + 'static
{
    fn from(mut value: S) -> Self {
        Self::new(async move |controller| {
            while let Some(item) = value.next().await {
                let result = controller.send(item).await;

                if let Some(err) = result.err() {
                    match err {
                        SendError::Serialize(x) => match x {},
                        SendError::StreamClosed => {
                            break;
                        },
                    }
                }
            }
        })
    }
}

impl<'a, T, E> Stream for ReadStream<'a, T, E>
where
    T: Deserialize<E> + 'a,
    E: From<DeserializerInitializationError> + 'a
{
    type Item = Result<T, E>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.0).poll_next(cx)
    }
}

impl<'a, T, E> From<Reader<T, E>> for ReadStream<'a, T, E>
where
    T: Deserialize<E> + 'a,
    E: From<DeserializerInitializationError> + 'a
{
    fn from(value: Reader<T, E>) -> Self {
        let inner = stream::unfold(value, async |mut reader| {
            let result = {
                match reader.read().await {
                    Ok(x) => Some(Ok(x)),
                    Err(ReadError::Deserialize(e)) => Some(Err(e)),
                    Err(ReadError::StreamClosed) => None
                }
            };

            result.map(|x| (x, reader))
        }).boxed_local();

        Self(inner)
    }
}
