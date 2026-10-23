use syn::Path;

pub mod crate_root;

pub fn result_path() -> Path {
    syn::parse_quote!(::std::result::Result)
}
