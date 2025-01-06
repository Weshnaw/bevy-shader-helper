use proc_macro::TokenStream;
use proc_macro2::TokenTree;
use quote::{ToTokens, quote};
use syn::{DeriveInput, Field, Meta, MetaList};

pub fn expand(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident, data, attrs, ..
    } = syn::parse_macro_input!(input as DeriveInput);

    let entries: Vec<_> = attrs
        .into_iter()
        .filter_map(|t| expand_entry(t.meta))
        .collect();
    let entry_count = entries.len();

    let rr = quote! { bevy_shader_helper::bevy::render::render_resource };
    let fields: Vec<_> = match data {
        syn::Data::Struct(data) => data
            .fields
            .into_iter()
            .map(|t| expand_field(t, &rr))
            .collect(),
        _ => unimplemented!("Cannot expand non-struct into shader data"),
    };
    let fields_count = fields.len();

    let handle = quote! { bevy_shader_helper::bevy::asset::Handle };

    let expanded = quote! {
    impl ShaderDataDetails<#fields_count, #entry_count> for #ident {
        fn buffer_entries(stage: #rr::ShaderStages) -> #rr::BindGroupLayoutEntries<#fields_count> {
            #rr::BindGroupLayoutEntries::sequential(
                stage,
                (
                    #(#fields),*
                ),
            )
        }

        fn entries(
            pipeline_cache: &#rr::PipelineCache,
            layout: #rr::BindGroupLayout,
            shader: #handle<#rr::Shader>,
        ) -> [#rr::CachedComputePipelineId; #entry_count] {
            [
                #(#entries),*
            ]
        }
    }};

    expanded.into()
}

fn expand_entry(meta: Meta) -> Option<impl ToTokens> {
    let meta = meta.require_list().ok()?;
    if !meta.path.is_ident("entry") {
        return None;
    }

    let mut args = meta.tokens.clone().into_iter().filter_map(|t| match t {
        TokenTree::Literal(literal) => Some(literal),
        _ => None,
    });

    let name = args.next()?;

    let label = if let Some(lit) = args.next() {
        quote! { Some(#lit.into()) }
    } else {
        quote! { None }
    };

    // eprintln!("{:#?}", args);
    Some(
        quote! { Self::create_entry(pipeline_cache, layout.clone(), shader.clone(), #name, #label) },
    )
}

enum FieldAttr {
    Texture(MetaList),
    ReadOnly,
}

fn expand_field(field: Field, rr: &impl ToTokens) -> impl ToTokens {
    let attr = field.attrs.into_iter().find_map(|a| {
        if a.path().is_ident("texture") {
            match a.meta {
                Meta::List(meta) => Some(FieldAttr::Texture(meta)),
                _ => unimplemented!("Not enough texture information")
            }
        } else if a.path().is_ident("read_only") {
            Some(FieldAttr::ReadOnly)
        } else {
            None
        }
    });

    let bind_types = quote! { #rr::binding_types };
    let ty = field.ty;
    if let Some(attr) = attr {
        match attr {
            FieldAttr::Texture(attrs) => {
                let a: Vec<_> = attrs.tokens.into_iter().filter_map(|t| match t {
                    TokenTree::Ident(ident) => Some(ident),
                    _ => None
                }).collect();
                assert!(a.len() == 3, "Not enough texture information");
                let access = &a[0];
                let format = &a[1];
                let dim = &a[2];
                // eprintln!("{:#?}", a);
                quote! {
                        #rr::IntoBindGroupLayoutEntryBuilder::into_bind_group_layout_entry_builder(#rr::BindingType::StorageTexture {
                            access: #rr::StorageTextureAccess::#access,
                            format: #rr::TextureFormat::#format,
                            view_dimension: #rr::TextureViewDimension::#dim,
                        })
                }
            }
            FieldAttr::ReadOnly => quote! { #bind_types::storage_buffer_read_only::<#ty>(false) },
        }
    } else {
        quote! { #bind_types::storage_buffer::<#ty>(false) }
    }
}
