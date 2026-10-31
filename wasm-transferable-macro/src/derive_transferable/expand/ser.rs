use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::{assets, derive_transferable::transformed_ast::*};

pub fn expand(ser_impl: SerializeImpl) -> TokenStream {
    let serialize_trait_path = assets::crate_root::serialize_trait_path();
    let serializer_path = assets::crate_root::serializer_path();
    let result_path = assets::result_path();

    let SerializeImpl {
        implementing_type,
        error_type_param_ident,
        where_clause,
    } = ser_impl;

    let ImplementingType {
        ident: type_ident,
        generic_params,
        body,
    } = implementing_type;

    let expr = expand_expr(body);

    quote! {
        impl<#error_type_param_ident, #(#generic_params),*> #serialize_trait_path<#error_type_param_ident> for #type_ident<#(#generic_params),*>
        #where_clause
        {
            fn serialize(self, ser: &mut #serializer_path) -> #result_path<(), #error_type_param_ident> {
                #expr

                #result_path::Ok(())
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
                let Self #items_expr = self;

                #ops
            }
        },
    }
}

fn expand_expr_match(variant_list: VariantList) -> TokenStream {
    let VariantList {
        index_bytes: _,
        variants,
    } = variant_list;

    let match_arm = variants.into_iter()
        .map(expand_match_arm);

    quote! {
        match self {
            #(#match_arm),*
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
        Self::#ident #items_expr => {
            ser.fixed_binary([ #(#index),* ]);

            #ops
        }
    }
}

#[allow(clippy::boxed_local)]
fn expand_ops(item_idents: Box<[Ident]>) -> TokenStream {
    quote! {
        #(
            ser.transferable(#item_idents)?;
        )*
    }
}
