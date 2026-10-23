use syn::Path;

pub fn path() -> Path {
    syn::parse_quote!(::wasm_transferable)
}

pub fn serializer_path() -> Path {
    let root = path();
    syn::parse_quote!(#root::Serializer)
}

pub fn deserializer_path() -> Path {
    let root = path();
    syn::parse_quote!(#root::Deserializer)
}

pub fn serialize_trait_path() -> Path {
    let root = path();
    syn::parse_quote!(#root::Serialize)
}

pub fn deserialize_trait_path() -> Path {
    let root = path();
    syn::parse_quote!(#root::Deserialize)
}
