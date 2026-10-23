use syn::{Ident, WhereClause};

pub struct DeriveTransferable {
    pub ser_impl: Option<SerializeImpl>,
    pub des_impl: Option<DeserializeImpl>
}

pub struct SerializeImpl {
    pub implementing_type: ImplementingType,

    pub error_type_param_ident: Ident,
    pub where_clause: Option<WhereClause>
}

pub struct DeserializeImpl {
    pub implementing_type: ImplementingType,

    pub error_type_param_ident: Ident,
    pub where_clause: Option<WhereClause>
}

#[derive(Clone)]
pub struct ImplementingType {
    pub ident: Ident,
    pub generic_params: Vec<syn::GenericParam>,

    pub body: ImplementingTypeBody
}

#[derive(Clone)]
pub enum ImplementingTypeBody {
    Enum(VariantList),
    Struct(Items)
}

#[derive(Clone)]
pub struct VariantList {
    pub index_bytes: usize,
    pub variants: Box<[Variant]>
}

#[derive(Clone)]
pub struct Variant {
    pub index: Box<[u8]>,
    pub ident: Ident,
    pub items: Items
}

#[derive(Clone)]
pub enum Items {
    Struct(Box<[Item]>),
    Tuple(Box<[Item]>),
    Unit
}

#[derive(Clone)]
pub struct Item {
    pub ident: Ident
}
