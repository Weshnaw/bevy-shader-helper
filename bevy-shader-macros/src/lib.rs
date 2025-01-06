use proc_macro::TokenStream;

mod internals;

// TODO: restrict ShaderEntry to enums which impl Debug, PartialEq, Eq, Hash, Clone 
#[proc_macro_derive(ShaderEntry)]
pub fn shader_entry(input: TokenStream) -> TokenStream {
    internals::entries::expand(input)
}

// TODO: restrict ShaderDataDetails to structs which impl Clone
#[proc_macro_derive(ShaderDataDetails, attributes(entry, read_only, texture))]
pub fn shader_data_details(input: TokenStream) -> TokenStream {
    internals::binding::expand(input)
}

// TODO: restrict BufferGroup to structs which impl Resource, ExtractResource and which types are all Buffer Types
#[proc_macro_derive(BufferGroup, attributes(data, writeable, texture))]
pub fn buffer_group(input: TokenStream) -> TokenStream {
    internals::buffers::expand(input)
}