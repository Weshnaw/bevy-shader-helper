use proc_macro::TokenStream;

mod internals;

// TODO: restrict ShaderEntry to enums which impl Debug, PartialEq, Eq, Hash, Clone 
#[proc_macro_derive(ShaderEntry)]
pub fn shader_entry(input: TokenStream) -> TokenStream {
    internals::entries::expand(input)
}

// TODO: add impl / Check that BufferGroups are ExtractResource and by extension Clone
// TODO: restrict BufferGroup to structs which fields are all Buffer Types
//       Can I somehow derive the writeable/write_only attributes purely from the field type information?
#[proc_macro_derive(BufferGroup, attributes(entry, writeable, write_only, texture, shader_type))]
pub fn buffer_group(input: TokenStream) -> TokenStream {
    internals::buffers::expand(input)
}