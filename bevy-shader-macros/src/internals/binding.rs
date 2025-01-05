use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn expand(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident,
        data: _,
        generics: _,
        ..
    } = syn::parse_macro_input!(input as DeriveInput);

    let rr = quote! { bevy_shader_helper::bevy::render::render_resource };
    let handle = quote! { bevy_shader_helper::bevy::asset::Handle };
    let bind_types = quote! { #rr::binding_types };

    let expanded = quote! {
    impl ShaderDataDetails<4, 2> for #ident {
        fn buffer_entries(stage: #rr::ShaderStages) -> #rr::BindGroupLayoutEntries<4> {
            #rr::BindGroupLayoutEntries::sequential(
                stage,
                (
                    #bind_types::storage_buffer::<Vec<u32>>(false),
                    #bind_types::storage_buffer::<Vec<u32>>(false),
                    #bind_types::storage_buffer::<Vec<u32>>(false),
                    #bind_types::storage_buffer_read_only::<u32>(false),
                ),
            )
        }

        fn entries(
            pipeline_cache: &#rr::PipelineCache,
            layout: #rr::BindGroupLayout,
            shader: #handle<#rr::Shader>,
        ) -> [#rr::CachedComputePipelineId; 2] {
            [
                Self::create_entry(pipeline_cache, layout.clone(), shader.clone(), "main", None),
                Self::create_entry(
                    pipeline_cache,
                    layout.clone(),
                    shader.clone(),
                    "update",
                    None,
                ),
            ]
        }
    }};

    expanded.into()
}
