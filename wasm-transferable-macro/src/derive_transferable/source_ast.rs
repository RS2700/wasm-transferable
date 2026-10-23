use darling::{FromDeriveInput, FromField, FromMeta, FromVariant, util::Override};
use syn::{Ident, Token, punctuated::Punctuated};

type TypeParamBounds = Punctuated<syn::TypeParamBound, Token![+]>;

#[derive(FromField)]
#[darling(attributes(transferable))]
pub struct Field {
    pub ident: Option<Ident>,
}

#[derive(FromVariant)]
#[darling(attributes(transferable))]
pub struct Variant {
    pub ident: Ident,
    pub fields: darling::ast::Fields<Field>
}

#[derive(Default)]
pub struct WherePredicates {
    pub predicates: Vec<syn::WherePredicate>
}

impl WherePredicates {
    pub fn is_empty(&self) -> bool {
        self.predicates.is_empty()
    }

    pub fn len(&self) -> usize {
        self.predicates.len()
    }

    pub fn push(&mut self, value: Vec<syn::WherePredicate>) {
        self.predicates.extend(value);
    }
}

#[derive(FromDeriveInput)]
#[darling(attributes(transferable))]
pub struct TransferableDeriveInput {
    pub ident: Ident,
    pub generics: darling::ast::Generics<syn::GenericParam>,
    pub data: darling::ast::Data<Variant, Field>,

    #[darling(default)]
    pub ser: Option<Override<SerializeOptions>>,
    #[darling(default)]
    pub des: Option<Override<DeserializeOptions>>,

    #[darling(default, multiple, rename = "where")]
    pub where_predicates: WherePredicates
}

#[derive(Default, FromMeta)]
pub struct SerializeOptions {
    #[darling(default, multiple, rename = "where")]
    pub where_predicates: WherePredicates,

    #[darling(default, rename = "where_error")]
    pub error_type_bounds: TypeParamBounds
}

#[derive(Default, FromMeta)]
pub struct DeserializeOptions {
    #[darling(default, multiple, rename = "where")]
    pub where_predicates: WherePredicates,

    #[darling(default, rename = "where_error")]
    pub error_type_bounds: TypeParamBounds
}
