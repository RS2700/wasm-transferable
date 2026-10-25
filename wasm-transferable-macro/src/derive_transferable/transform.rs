use darling::util::Override;
use proc_macro2::Span;
use syn::{Ident, punctuated::Punctuated};

use crate::derive_transferable::{source_ast, transformed_ast};

pub fn transform(source: source_ast::TransferableDeriveInput) -> transformed_ast::DeriveTransferable {
    let source_ast::TransferableDeriveInput {
        ident,
        generics,
        data,
        ser,
        des,
        where_predicates,
    } = source;
    
    let darling::ast::Generics {
        params: generic_params,
        mut where_clause,
    } = generics;

    let implementing_type = to_implementing_type(ident, generic_params, data);
    where_clause.expand(where_predicates.predicates);

    transformed_ast::DeriveTransferable {
        ser_impl: ser.map(|ser| to_ser_impl(implementing_type.clone(), ser, where_clause.clone())),
        des_impl: des.map(|des| to_des_impl(implementing_type, des, where_clause)),
    }
}

fn to_implementing_type(
    ident: Ident,
    generic_params: Vec<syn::GenericParam>,
    data: darling::ast::Data<source_ast::Variant, source_ast::Field>
) -> transformed_ast::ImplementingType {
    transformed_ast::ImplementingType {
        ident,
        generic_params,
        body: to_implementing_type_body(data),
    }
}

fn to_implementing_type_body(data: darling::ast::Data<source_ast::Variant, source_ast::Field>) -> transformed_ast::ImplementingTypeBody {
    match data {
        darling::ast::Data::Enum(items) => {
            let transformed = to_variant_list(items);

            transformed_ast::ImplementingTypeBody::Enum(transformed)
        },
        darling::ast::Data::Struct(fields) => {
            let transformed = to_items(fields);

            transformed_ast::ImplementingTypeBody::Struct(transformed)
        },
    }
}

fn to_variant_list(source: Vec<source_ast::Variant>) -> transformed_ast::VariantList {
    const USIZE_BYTES: usize = (usize::BITS / u8::BITS) as usize;

    let wasted_bytes = leading_zero_bytes(source.len());
    let index_bytes = USIZE_BYTES - wasted_bytes;

    let variants = source.into_iter()
        .enumerate()
        .map(|(index, source)| {
            let index = &index.to_be_bytes()[wasted_bytes..];
            to_variant(source, Box::from(index))
        })
        .collect();

    transformed_ast::VariantList {
        index_bytes,
        variants,
    }
}

fn to_variant(source: source_ast::Variant, index: Box<[u8]>) -> transformed_ast::Variant {
    let source_ast::Variant {
        ident,
        fields,
    } = source;

    transformed_ast::Variant {
        index,
        ident,
        items: to_items(fields),
    }
}

fn to_items(source: darling::ast::Fields<source_ast::Field>) -> transformed_ast::Items {
    match &source.style {
        darling::ast::Style::Tuple => {
            let transformed = source.fields.into_iter()
                .enumerate()
                .map(|(index, _source)| {
                    let ident_str = format!("field{}", index);
                    let ident = Ident::new(&ident_str, Span::call_site());
                    to_item(ident)
                })
                .collect();

            transformed_ast::Items::Tuple(transformed)
        },
        darling::ast::Style::Struct => {
            let transformed = source.fields.into_iter()
                .map(|source| {
                    let ident = source.ident.unwrap();
                    to_item(ident)
                })
                .collect();

            transformed_ast::Items::Struct(transformed)
        },
        darling::ast::Style::Unit => {
            transformed_ast::Items::Unit
        },
    }
}

fn to_item(ident: Ident) -> transformed_ast::Item {
    transformed_ast::Item {
        ident,
    }
}


fn to_ser_impl(
    implementing_type: transformed_ast::ImplementingType,
    options: Override<source_ast::SerializeOptions>,
    mut where_clause: Option<syn::WhereClause>
) -> transformed_ast::SerializeImpl {
    let error_type_param_ident = Ident::new("__SerializeError", Span::call_site());

    let source_ast::SerializeOptions {
        where_predicates,
        error_type_bounds,
    } = options.unwrap_or_default();

    where_clause.expand(where_predicates.predicates);
    where_clause.expand(expand_type_param_bounds(&error_type_param_ident, error_type_bounds.into()));

    transformed_ast::SerializeImpl {
        implementing_type,
        error_type_param_ident,
        where_clause,
    }
}

fn to_des_impl(
    implementing_type: transformed_ast::ImplementingType,
    options: Override<source_ast::DeserializeOptions>,
    mut where_clause: Option<syn::WhereClause>
) -> transformed_ast::DeserializeImpl {
    let error_type_param_ident = Ident::new("__DeserializeError", Span::call_site());

    let source_ast::DeserializeOptions {
        where_predicates,
        error_type_bounds,
    } = options.unwrap_or_default();

    where_clause.expand(where_predicates.predicates);
    where_clause.expand(expand_type_param_bounds(&error_type_param_ident, error_type_bounds.into()));

    transformed_ast::DeserializeImpl {
        implementing_type,
        error_type_param_ident,
        where_clause,
    }
}

fn expand_type_param_bounds(
    ident: &Ident,
    bounds: Punctuated<syn::TypeParamBound, syn::Token![+]>
) -> Option<syn::WherePredicate> {
    if bounds.is_empty() {
        return None
    }

    let predicate = syn::PredicateType {
        attrs: Vec::new(),
        lifetimes: None,
        bounded_ty: syn::parse_quote!(#ident),
        colon_token: Default::default(),
        bounds,
    };
    Some(syn::WherePredicate::Type(predicate))
}


fn leading_zero_bytes(x: usize) -> usize {
    let bits = x.leading_zeros();
    let bytes = bits / u8::BITS;
    bytes as usize
}

trait ExtendToOptionedClosure {
    fn get_or_init(&mut self) -> &mut syn::WhereClause;
    fn expand(&mut self, iter: impl IntoIterator<Item = syn::WherePredicate>);
}

impl ExtendToOptionedClosure for Option<syn::WhereClause> {
    fn get_or_init(&mut self) -> &mut syn::WhereClause {
        self.get_or_insert_with(|| {
            syn::WhereClause {
                where_token: Default::default(),
                predicates: syn::punctuated::Punctuated::new(),
            }
        })
    }

    fn expand(&mut self, iter: impl IntoIterator<Item = syn::WherePredicate>) {
        let mut iter = iter.into_iter().peekable();

        if iter.peek().is_none() {
            // イテレーターが空なので、何もしない
            return
        }

        let closure = self.get_or_init();
        closure.predicates.extend(iter);
    }
}