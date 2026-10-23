use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

mod assets;
mod derive_transferable;

#[proc_macro_derive(Transferable, attributes(transferable))]
pub fn derive_transferable(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);

    derive_transferable::expand(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
