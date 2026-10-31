use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::{assets, derive_transferable::transformed_ast::*};

pub fn expand(des_impl: DeserializeImpl) -> TokenStream {
    let deserialize_trait_path = assets::crate_root::deserialize_trait_path();
    let deserializer_path = assets::crate_root::deserializer_path();
    let result_path = assets::result_path();

    let DeserializeImpl {
        implementing_type,
        error_type_param_ident,
        where_clause,
    } = des_impl;

    let ImplementingType {
        ident: type_ident,
        generic_params,
        body,
    } = implementing_type;

    let expr = expand_expr(body);

    quote! {
        impl<#error_type_param_ident, #(#generic_params),*> #deserialize_trait_path<#error_type_param_ident> for #type_ident<#(#generic_params),*>
        #where_clause
        {
            fn deserialize(des: &mut #deserializer_path) -> #result_path<Self, #error_type_param_ident> {
                let x = { #expr };

                #result_path::Ok(x)
            }
        }
    }
}

fn expand_expr(body: ImplementingTypeBody) -> TokenStream {
    match body {
        ImplementingTypeBody::Enum(variant_list) => {
            expand_expr_match(variant_list)
        },
        ImplementingTypeBody::Struct(items) => {
            let (items_expr, item_idents) = super::expand_items(items);
            let ops = expand_ops(item_idents);

            quote! {
                #ops

                Self #items_expr
            }
        },
    }
}

fn expand_expr_match(variant_list: VariantList) -> TokenStream {
    let deserialize_error_path = assets::crate_root::deserialize_error_path();

    let VariantList {
        index_bytes,
        variants,
    } = variant_list;

    let match_arm = variants.into_iter()
        .map(expand_match_arm);

    quote! {
        match des.fixed_binary::<#index_bytes>()? {
            #(#match_arm),*

            _ => return Err(#deserialize_error_path::Format.into())
        }
    }
}

fn expand_match_arm(variant: Variant) -> TokenStream {
    let Variant {
        index,
        ident,
        items,
    } = variant;

    let (items_expr, item_idents) = super::expand_items(items);
    let ops = expand_ops(item_idents);

    quote! {
        [ #(#index),* ] => {
            #ops

            Self::#ident #items_expr
        }
    }
}

#[allow(clippy::boxed_local)]
fn expand_ops(item_idents: Box<[Ident]>) -> TokenStream {
    quote! {
        #(
            let #item_idents = des.transferable()?;
        )*
    }
}
