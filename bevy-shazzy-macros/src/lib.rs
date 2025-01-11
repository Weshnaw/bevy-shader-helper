use bevy_shazzy_tokens::buffers::ShaderBuffer;
use proc_macro::TokenStream;
use syn::DeriveInput;

// TODO: add impl / Check that BufferGroups are ExtractResource and by extension Clone
// TODO: restrict BufferGroup to structs which fields are all Buffer Types
//       Can I somehow derive the writeable/write_only attributes purely from the field type information?
#[proc_macro_derive(
    BufferGroup,
    attributes(entry, writeable, write_only, texture, shader_type)
)]
pub fn buffer_group(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);

    let shader = ShaderBuffer::from(input);

    let shader = quote::quote! {#shader};
    // panic!("{}", shader.to_string());
    shader.into()
}
