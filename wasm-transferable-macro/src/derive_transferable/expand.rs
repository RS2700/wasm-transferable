use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::derive_transferable::transformed_ast::*;

mod des;
mod ser;

pub fn expand(transformed: DeriveTransferable) -> TokenStream {
    let DeriveTransferable {
        ser_impl,
        des_impl,
    } = transformed;

    let ser = ser_impl.map(|ser_impl| ser::expand(ser_impl));
    let des = des_impl.map(|des_impl| des::expand(des_impl));

    quote! {
        #ser

        #des
    }
}


fn expand_items(items: Items) -> (TokenStream, Box<[Ident]>) {
    match items {
        Items::Struct(items) => {
            let item_idents = expand_item_idents(items);

            let ts = quote! {
                { #(#item_idents),* }
            };

            (ts, item_idents)
        },
        Items::Tuple(items) => {
            let item_idents = expand_item_idents(items);

            let ts = quote! {
                ( #(#item_idents),* )
            };
            
            (ts, item_idents)
        },
        Items::Unit => {
            Default::default()
        },
    }
}

fn expand_item_idents(items: Box<[Item]>) -> Box<[Ident]> {
    items.into_iter()
        .map(|x| x.ident)
        .collect()
}
