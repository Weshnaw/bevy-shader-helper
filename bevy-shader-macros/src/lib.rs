use proc_macro::TokenStream;

mod internals;

#[proc_macro_derive(ShaderEntry)]
pub fn shader_entry(input: TokenStream) -> TokenStream {
    internals::entries::expand(input)
}

#[proc_macro_derive(ShaderDataDetails, attributes(entry, read_only, texture))]
pub fn shader_data_details(input: TokenStream) -> TokenStream {
    internals::binding::expand(input)
}
