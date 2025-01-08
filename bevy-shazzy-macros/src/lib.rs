use proc_macro::TokenStream;

mod internals;

// TODO: add impl / Check that BufferGroups are ExtractResource and by extension Clone
// TODO: restrict BufferGroup to structs which fields are all Buffer Types
//       Can I somehow derive the writeable/write_only attributes purely from the field type information?
#[proc_macro_derive(
    BufferGroup,
    attributes(entry, writeable, write_only, texture, shader_type)
)]
pub fn buffer_group(input: TokenStream) -> TokenStream {
    internals::buffers::expand(input)
}
