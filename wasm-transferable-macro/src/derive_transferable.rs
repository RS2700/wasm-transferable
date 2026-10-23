use darling::FromDeriveInput;
use syn::DeriveInput;

mod expand;
mod source_ast;
mod transform;
mod transformed_ast;

pub fn expand(input: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let source = source_ast::TransferableDeriveInput::from_derive_input(&input)?;
    let transformed = transform::transform(source);
    Ok(expand::expand(transformed))
}
