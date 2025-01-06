use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{DeriveInput, Field, Member};

pub fn expand(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident, data, attrs, ..
    } = syn::parse_macro_input!(input as DeriveInput);

    let data_type = attrs
        .into_iter()
        .find(|a| {
            a.meta
                .require_list().is_ok_and(|a| a.path.is_ident("data"))
        })
        .expect("Buffer Groups must reference a buffer data type")
        .meta
        .require_list()
        .expect("Unknown Buffer Data Type")
        .tokens
        .clone()
        .into_iter()
        .next()
        .expect("Must contain a Buffer Data Type");

    let render = quote! { bevy_shader_helper::bevy::render };
    let rr = quote! { #render::render_resource };
    let assets = quote! { bevy_shader_helper::bevy::Assets };
    let image = quote! { bevy_shader_helper::bevy::Image };
    let commands = quote! { bevy_shader_helper::bevy::Commands };
    let buffers = quote! { bevy_shader_helper::internals::buffers };

    let fields: (Vec<_>, Vec<_>) = match data {
        syn::Data::Struct(data) => data
            .fields
            .into_iter()
            .enumerate()
            .map(|(count, f)| {
                (
                    expand_entries(f.clone(), &buffers, count),
                    expand_resources(f, &buffers, count),
                )
            })
            .unzip(),
        _ => unimplemented!("Cannot expand non-struct into buffer group"),
    };
    let entries = fields.0;
    let resources = fields.1;
    let size = entries.len();
    quote! {
     // I do not know why this is needed...
    use bevy_shader_helper::bevy::bevy_ecs;
    impl BufferGroup<#data_type, #size> for #ident {
        fn get_bindings<'a>(
            &'a self,
            buffers: &'a #render::render_asset::RenderAssets<#render::storage::GpuShaderStorageBuffer>,
            images: &'a #render::render_asset::RenderAssets<#render::texture::GpuImage>,
        ) -> #rr::BindGroupEntries<'a, #size> {
            #rr::BindGroupEntries::sequential((
                #(#entries),*
            ))
        }

        fn insert_resources(
            commands: &mut #commands,
            buffers: &mut #assets<#render::storage::ShaderStorageBuffer>,
            images: &mut #assets<#image>,
            d: #data_type,
        ) {
            commands.insert_resource(Self {
                #(#resources),*
            });
        }
    }
        }
    .into()
}

fn expand_entries(field: Field, buffers: &impl ToTokens, count: usize) -> impl ToTokens {
    let texture = field.attrs.iter().any(|a| {
        a.meta
            .require_path_only().is_ok_and(|t| t.is_ident("texture"))
    });
    let ident = ident_to_member(field, count);
    let buffer = if texture {
        quote! {images}
    } else {
        quote! {buffers}
    };

    quote! {#buffers::HandleIntoBinding::binding(&self.#ident, #buffer)}
}

fn expand_resources(field: Field, buffers: &impl ToTokens, count: usize) -> impl ToTokens {
    let texture = field.attrs.iter().any(|a| {
        a.meta
            .require_path_only().is_ok_and(|t| t.is_ident("texture"))
    });
    let writeable = field.attrs.iter().any(|a| {
        a.meta
            .require_path_only().is_ok_and(|t| t.is_ident("writeable"))
    });
    let ident = ident_to_member(field, count);

    let create = if texture {
        quote! {create_texture_buffer(images, d.#ident, #writeable)}
    } else {
        quote! {create_storage_buffer(buffers, d.#ident, #writeable)}
    };

    quote! {#ident: #buffers::#create.into()}
}

fn ident_to_member(field: Field, count: usize) -> Member {
    if let Some(ident) = field.ident {
        Member::Named(ident)
    } else {
        Member::Unnamed(count.into())
    }
}
