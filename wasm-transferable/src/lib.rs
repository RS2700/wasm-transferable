//! # wasm-transferable
//! 
//! Rustの構造体を [`JsValue`] へシリアライズすると同時に、それに含まれる [`Transferable Objects`](https://developer.mozilla.org/docs/Web/API/Web_Workers_API/Transferable_objects) を収集するフレームワーク。
//! 
//! `Worker` や `MessagePort` の `postMessage()` メソッドを使ってRustの構造体のやり取りする場合に、`Transferable Objects` の抽出の手間なくその恩恵を受けることを可能にします。
//! 
//! # 転送と所有権の消費
//! 
//! オブジェクトは転送されると、中身が抜き取られて無効な状態になります。
//! 
//! javascriptでは、無効なオブジェクトを使用しようとするとランタイムエラーが発生しますが、
//! このクレートでは　[`Serialize::serialize`] が所有権を消費することで、ランタイムエラーが発生しないことを保証することができる枠組みを提供しています。
//! 
//! # エラー
//! 
//! ジェネリクスを利用することで、シリアライズ・デシリアライズで発生したエラーを、型安全に取り扱えます。
//! 
//! シリアライズを実行する際は、[`thiserror`] クレートなどを利用して独自のエラー型を定義することが、設計として想定されています。<br/>
//! 独自のエラーを返す [`Serialize`], [`Deserialize`] を実装する場合は、エラー型に対するトレイト境界に `From<MyError>` のような [`From`] トレイトを指定すると、型のエラーハンドリングをスムーズに統合しやすくなります。

#![cfg_attr(docsrs, feature(doc_auto_cfg))]

use std::vec;

use js_sys::{Array, ArrayIntoIter, Uint8Array};
use thiserror::Error;
use wasm_bindgen::{JsCast, JsValue};

mod extensions;

/// [`web_sys::MessagePort`] や [`web_sys::Worker`] などに、直接 [`SerializeResult`] をポストできる拡張トレイトとその実装を提供します。
pub mod post_ext;

/// [`Serialize`] と [`Deserialize`] を実装することができるderiveマクロ。
/// 
/// # ヘルパー属性
/// #### 構文の全体像
/// ```ignore
/// #[transferable(
///     ser(
///         where_predicates(WherePredicates),
///         error_type_bounds(TypeParamBounds)
///     ),
///     des(
///         where_predicates(WherePredicates),
///         error_type_bounds(TypeParamBounds)
///     ),
///     where_predicates(WherePredicates)
///  )]
/// ```
/// 
/// #### transferable ヘルパー属性
/// 
/// | 属性名 | 型 | 必須/任意 | 説明 |
/// | :--- | :--- | :--- | :--- |
/// | `ser` | [SerializeOptions](#serializeoptions) | 任意 | **[`Serialize`] を実装します。**<br>指定する場合、すべてのフィールドの型が [`Serialize`] を実装している必要があります。 |
/// | `des` | [DeserializeOptions](#deserializeoptions) | 任意 | **[`Deserialize`] を実装します。**<br>指定する場合、すべてのフィールドの型が [`Deserialize`] を実装している必要があります。 |
/// | `where_predicates` | [WherePredicates](#wherepredicates) | 任意 | このマクロで出力される全ての `impl` ブロックに対して、共通のトレイト境界を付与します。 |
/// 
/// #### SerializeOptions
/// 
/// | 属性名 | 型 | 必須/任意 | 説明 |
/// | :--- | :--- | :--- | :--- |
/// | `where_predicates` | [WherePredicates](#wherepredicates) | 任意 | [`Serialize`] の実装において、**型本体が持つ型引数**に対してトレイト境界を付与します。 |
/// | `error_type_bounds` | [TypeParamBounds](#typeparambounds) | 任意 | [`Serialize<E>`] **の型引数 `E`** に対して、トレイト境界を付与します。 |
/// 
/// #### DeserializeOptions
/// 
/// | 属性名 | 型 | 必須/任意 | 説明 |
/// | :--- | :--- | :--- | :--- |
/// | `where_predicates` | [WherePredicates](#wherepredicates) | 任意 | [`Deserialize`] の実装において、**型本体が持つ型引数**に対してトレイト境界を付与します。 |
/// | `error_type_bounds` | [TypeParamBounds](#typeparambounds) | 任意 | [`Deserialize<E>`] **の型引数 `E`** に対して、トレイト境界を付与します。 |
/// 
/// #### WherePredicates
/// 
/// `where` 句の内部に記述するトレイト境界のリスト（末尾のカンマは任意）。
/// 
/// ```ignore
/// T: MyTrait1 + MyTrait2,
/// U: MyTrait3,
/// ```
/// 
/// #### TypeParamBounds
/// 
/// 型パラメータに付与するトレイト境界です（`+` で結合）。
/// 
/// ```ignore
/// From<MyError> + MyTrait1 + MyTrait2
/// ```
/// 
/// # Examples
/// 
/// ```no_run
/// # use wasm_transferable::*;
/// # 
/// # #[cfg(feature = "std-ext")]
/// #[derive(Transferable)]
/// #[transferable(ser)]
/// #[transferable(des(where_error(From<DeserializeError>)))]
/// struct GetFileOptions {
///     name: String,
///     create: bool
/// }
/// ```
/// 
/// ```no_run
/// # use wasm_transferable::*;
/// # 
/// # #[cfg(feature = "std-ext")]
/// #[derive(Transferable)]
/// #[transferable(ser)]
/// #[transferable(des(where_error(From<DeserializeError>)))]
/// enum GetFileResult {
///     Success(Vec<u8>),
///     NotFound
/// }
/// ```
#[cfg(feature = "derive")]
pub use wasm_transferable_macro::Transferable;

/// シリアライズ処理の結果を格納する構造体。
pub struct SerializeResult {
    /// データをシリアライズした結果である [`JsValue`]。
    pub data: JsValue,
    /// 収集された Transferable Objects の配列。
    pub transfers: Array
}

/// デシリアライザの初期化時に発生するエラー。
#[derive(Debug, Error)]
pub enum DeserializerInitializationError {
    /// データ構造が無効である場合のエラー。
    #[error("data type is invalid")]
    Format
}

/// デシリアライズにおいて汎用的に使用されるエラー。
/// 
/// デシリアライズの実行中、主にデシリアライザ（[`Deserializer`]）の操作過程で発生する共通のエラーを表します。
/// 各型のカスタムデシリアライズ処理や、汎用的なエラー型としても利用されます。
#[derive(Debug, Error)]
pub enum DeserializeError {
    /// データ構造が無効である場合のエラー。
    #[error("data type is invalid")]
    Format
}

/// 構造体を [`JsValue`] へシリアライズし、それと同時に `Transferable Objects` を収集するシリアライザ。
pub struct Serializer {
    binary: Vec<u8>,
    values: Array,
    transfers: Array
}

impl Serializer {
    /// 新しい [`Serializer`] インスタンスを作成します。
    pub fn new() -> Self {
        Self {
            binary: Vec::new(),
            values: Array::new(),
            transfers: Array::new(),
        }
    }

    /// シリアライズを完了し、結果である [`SerializeResult`] を返します。
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
    /// Transferable Objects を転送リストに追加します。
    /// 
    /// > **注意:** この関数は、単に `Transferable Objects` の配列に追加して、転送対象としてマークするだけです。
    /// > **[`Serializer::js_value()`] と合わせて使用してください。**
    /// 
    /// # Examples:
    /// ```
    /// # let mut ser = wasm_transferable::Serializer::new();
    /// # let typed_array = js_sys::Uint8Array::new_with_length(0);
    /// // typed_array: js_sys::Uint8Array
    /// ser.js_value(&typed_array);
    /// ser.transfer(&typed_array.buffer());
    /// ```
    pub fn transfer(&mut self, value: &JsValue) {
        self.transfers.push(value);
    }
    
    /// [`JsValue`] の値をシリアライズします。
    pub fn js_value(&mut self, value: &JsValue) {
        self.values.push(value);
    }

    /// [`Serialize`] を実装した値をシリアライズします。
    pub fn transferable<T, E>(&mut self, value: T) -> Result<(), E>
    where
        T: Serialize<E>
    {
        value.serialize(self)
    }

    /// 1バイトの値をシリアライズします。
    pub fn byte(&mut self, byte: u8) {
        self.binary.push(byte);
    }

    /// 固定長のバイナリをシリアライズします。
    pub fn fixed_binary<const N: usize>(&mut self, bin: [u8; N]) {
        self.binary.extend(bin);
    }
}

/// [`Serializer`] によってシリアライズされた [`JsValue`] から構造体を復元するデシリアライザ。
pub struct Deserializer {
    binary: vec::IntoIter<u8>,
    values: ArrayIntoIter<JsValue>
}

impl Deserializer {
    /// [`Serializer`] によってシリアライズされた [`JsValue`] を元に、デシリアライザを作成します。
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
    /// [`JsValue`] の値をデシリアライズします。
    pub fn js_value(&mut self) -> Result<JsValue, DeserializeError> {
        self.values.next()
            .ok_or_else(|| DeserializeError::Format)
    }

    /// [`Deserialize`] を実装した値をデシリアライズします。
    pub fn transferable<T, E>(&mut self) -> Result<T, E>
    where
        T: Deserialize<E>
    {
        T::deserialize(self)
    }

    /// 1バイトの値をデシリアライズします。
    pub fn byte(&mut self) -> Result<u8, DeserializeError> {
        self.binary.next()
            .ok_or(DeserializeError::Format)
    }

    /// 固定長のバイナリをデシリアライズします。
    pub fn fixed_binary<const N: usize>(&mut self) -> Result<[u8; N], DeserializeError> {
        array_init::from_iter(self.binary.by_ref())
            .ok_or(DeserializeError::Format)
    }
}

/// `wasm-transferable` によってシリアライズできるデータ構造。
/// 
/// > note: 簡単な構造体を作る場合など、多くの場合では [`derive@wasm_transferable_macro::Transferable`] deriveマクロが便利で安全です。
pub trait Serialize<E> {
    /// 自身を消費して、シリアライズします。
    /// 
    /// > 注意: 対応する [`Deserialize::deserialize`] のデシリアライズ手順と順序が同一でなければなりません。
    fn serialize(self, ser: &mut Serializer) -> Result<(), E>;
}

/// `wasm-transferable` によってデシリアライズできるデータ構造。
/// 
/// > note: 簡単な構造体を作る場合など、多くの場合では [`derive@wasm_transferable_macro::Transferable`] deriveマクロが便利で安全です。
pub trait Deserialize<E>
where
    Self: Sized
{
    /// デシリアライズして自身を復元します。
    /// 
    /// > 注意: 対応する [`Serialize::serialize`] のシリアライズ手順と順序が同一でなければなりません。
    fn deserialize(des: &mut Deserializer) -> Result<Self, E>;
}

/// 任意の [`Serialize`] を実装している値を、シリアライズします。
pub fn serialize<T, E>(value: T) -> Result<SerializeResult, E>
where
    T: Serialize<E>
{
    let mut ser = Serializer::new();
    ser.transferable(value)?;
    Ok(ser.end())
}

/// [`JsValue`] から、任意の [`Deserialize`] を実装している型へデシリアライズします。
/// 
/// > 注意: `data` 引数が意図した値[^1]でない場合、**失敗せず、意図していない値が返される可能性があります**。
/// 
/// [^1]: *意図した値*とは、型引数 `T` が一致している [`serialize`] 関数によって返された [`SerializeResult::data`] のことです。
pub fn deserialize<T, E>(data: JsValue) -> Result<T, E>
where
    T: Deserialize<E>,
    E: From<DeserializerInitializationError>
{
    let mut des = Deserializer::new(data)?;
    let value = des.transferable::<T, E>()?;
    Ok(value)
}
