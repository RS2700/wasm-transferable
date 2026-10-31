use std::convert::Infallible;

use thiserror::Error;
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};
use wasm_transferable::{DeserializeError, DeserializerInitializationError};
use wasm_transferable_stream::*;

wasm_bindgen_test_configure!(run_in_browser);

#[derive(Debug, Error)]
enum MyDeserializeError {
    #[error("init ({0})")]
    Init(#[from] DeserializerInitializationError),
    #[error("des ({0})")]
    Des(#[from] DeserializeError)
}

#[wasm_bindgen_test]
async fn readable() {
    type MyStream = ReadableTransferStream<i32>;

    let items = 0..5;

    let serial = {
        let source = items.clone();

        let stream = MyStream::new::<_, Infallible>(async move |controller| {
            for item in source {
                let result = controller.send(item).await;

                result.unwrap();
            }
        });

        let result = wasm_transferable::serialize(stream)
            .unwrap_or_else(|x: Infallible| match x {});

        result.data
    };

    let stream = wasm_transferable::deserialize::<MyStream, MyDeserializeError>(serial)
        .unwrap();

    let mut reader = stream.into_reader::<MyDeserializeError>();

    for item in items {
        let readed = reader.read().await
            .unwrap();

        assert_eq!(readed, item)
    }

    assert!(matches!(
        reader.read().await,
        Err(readable::ReadError::StreamClosed)
    ));
}

#[wasm_bindgen_test]
async fn writable() {
    type MyStream = WritableTransferStream<i32>;

    let items = 0..5;

    let serial = {
        let items = items.clone();

        let stream = MyStream::new::<_, MyDeserializeError>(async move |controller| {
            for item in items {
                let getted = controller.recv().await
                    .unwrap();

                assert_eq!(getted, item)
            }

            assert!(matches!(
                controller.recv().await,
                Err(writable::RecvError::StreamClosed)
            ));
        });

        let result = wasm_transferable::serialize(stream)
            .unwrap_or_else(|x: Infallible| match x {});

        result.data
    };

    let stream = wasm_transferable::deserialize::<MyStream, MyDeserializeError>(serial)
        .unwrap();

    let mut writer = stream.into_writer::<Infallible>();

    for item in items {
        writer.write(item).await
            .unwrap();
    }
}
