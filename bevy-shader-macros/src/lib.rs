use proc_macro::TokenStream;

mod internals;

#[proc_macro_derive(ShaderEntry)]
pub fn shader_entry_macro_derive(input: TokenStream) -> TokenStream {
    internals::entries::expand(input)
}