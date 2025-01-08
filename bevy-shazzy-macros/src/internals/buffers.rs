use proc_macro::TokenStream;
use proc_macro2::{Span, TokenTree};
use quote::{ToTokens, format_ident, quote, quote_spanned};
use syn::{Data, DataStruct, DeriveInput, Field, Ident, Member, Meta};

pub fn expand(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident, data, attrs, ..
    } = syn::parse_macro_input!(input as DeriveInput);

    let initializer_ident = format_ident!("{}Initializer", ident);
    let initializer = create_initializer(initializer_ident.clone(), ident.clone(), data.clone());

    // TODO: would like to be able to get the shader entry information by passing in the ShaderEntry enum instead of relying on strings
    //       MAYBE by passing the entries as a type parameter into the buffer struct?
    let shader_entries: Vec<_> = attrs
        .into_iter()
        .filter_map(|t| expand_entry(t.meta))
        .collect();

    let span = ident.span();
    let entry_count = shader_entries.len();

    let render = quote! { bevy_shazzy::bevy::render };
    let rr = quote! { #render::render_resource };
    let buffers = quote! { bevy_shazzy::internals::buffers };
    let handle = quote! { bevy_shazzy::bevy::Handle };
    let assets = quote! { bevy_shazzy::bevy::Assets };
    let image = quote! { bevy_shazzy::bevy::Image };
    let commands = quote! { bevy_shazzy::bevy::Commands };
    let ecs = quote! { bevy_shazzy::bevy::ecs };

    let fields: (Vec<_>, Vec<_>) = match data {
        syn::Data::Struct(data) => data
            .fields
            .iter()
            .enumerate()
            .map(|(count, f)| {
                (
                    expand_entries(f.clone(), &buffers, count),
                    expand_field(f.clone(), &rr, span),
                )
            })
            .unzip(),
        _ => unimplemented!("Cannot expand non-struct into buffer group"),
    };
    let bind_entries = fields.0;
    let bind_group_layout = fields.1;
    let field_count = bind_entries.len();

    quote! {
    impl #ecs::system::Resource for #ident
    where
        Self: Send + Sync + 'static,
    {}
    impl BufferGroup<#field_count, #entry_count> for #ident {
        type Initializer = #initializer_ident;

        fn get_bindings<'a>(
            &'a self,
            buffers: &'a #render::render_asset::RenderAssets<#render::storage::GpuShaderStorageBuffer>,
            images: &'a #render::render_asset::RenderAssets<#render::texture::GpuImage>,
        ) -> #rr::BindGroupEntries<'a, #field_count> {
            #rr::BindGroupEntries::sequential((
                #(#bind_entries),*
            ))
        }

        fn insert_resources(
            commands: &mut #commands,
            buffers: &mut #assets<#render::storage::ShaderStorageBuffer>,
            images: &mut #assets<#image>,
            init: #initializer_ident,
        ) {
            commands.insert_resource(init.to_resources(buffers, images));
        }

        fn buffer_entries(stage: #rr::ShaderStages) -> #rr::BindGroupLayoutEntries<#field_count> {
            #rr::BindGroupLayoutEntries::sequential(
                stage,
                (
                    #(#bind_group_layout),*
                ),
            )
        }

        fn entries(
            pipeline_cache: &#rr::PipelineCache,
            layout: #rr::BindGroupLayout,
            shader: #handle<#rr::Shader>,
        ) -> [#rr::CachedComputePipelineId; #entry_count] {
            [
                #(#shader_entries),*
            ]
        }
    }

    #initializer
}.into()
}

fn expand_entries(field: Field, buffers: &impl ToTokens, count: usize) -> impl ToTokens {
    let texture = texture_details(&field).is_some();
    let ident = ident_to_member(field, count);
    let buffer = if texture {
        quote! {images}
    } else {
        quote! {buffers}
    };

    quote! {#buffers::HandleIntoBinding::binding(&self.#ident, #buffer)}
}

fn texture_details(field: &Field) -> Option<(Ident, Ident)> {
    field.attrs.iter().find_map(|a| {
        if a.path().is_ident("texture") {
            let details = a
                .meta
                .require_list()
                .expect("Requires more texture information");
            let a: Vec<_> = details
                .tokens
                .clone()
                .into_iter()
                .filter_map(|t| match t {
                    TokenTree::Ident(ident) => Some(ident),
                    _ => None,
                })
                .collect();
            assert!(a.len() == 2, "Not enough texture information");
            Some((a[0].clone(), a[1].clone()))
        } else {
            None
        }
    })
}

fn ident_to_member(field: Field, count: usize) -> Member {
    if let Some(ident) = field.ident {
        Member::Named(ident)
    } else {
        Member::Unnamed(count.into())
    }
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

fn expand_field(field: Field, rr: &impl ToTokens, span: Span) -> impl ToTokens {
    let texture = texture_details(&field);
    let writeable = field
        .attrs
        .iter()
        .any(|a| a.meta.path().is_ident("writeable"));
    let write_only = field
        .attrs
        .iter()
        .any(|a| a.meta.path().is_ident("write_only"));

    let bind_types = quote! { #rr::binding_types };

    if let Some(attrs) = texture {
        let access = if writeable {
            quote! {ReadWrite}
        } else if write_only {
            quote! {WriteOnly}
        } else {
            quote! {ReadOnly}
        };
        let format = &attrs.0;
        let dim = &attrs.1;
        quote! {
            #rr::IntoBindGroupLayoutEntryBuilder::into_bind_group_layout_entry_builder(#rr::BindingType::StorageTexture {
                access: #rr::StorageTextureAccess::#access,
                format: #rr::TextureFormat::#format,
                view_dimension: #rr::TextureViewDimension::#dim,
            })
        }
    } else {
        assert!(!write_only, "Storage buffers cannot be write only");
        let ty = get_type_from_shader_type(&field);

        let ty = quote_spanned! {span=> #ty};
        let access = if writeable {
            quote! {storage_buffer}
        } else {
            quote! {storage_buffer_read_only}
        };
        quote! { #bind_types::#access::<#ty>(false) }
    }
}

fn get_type_from_shader_type(field: &Field) -> proc_macro2::TokenStream {
    field
        .attrs
        .iter()
        .find_map(|a| {
            if a.path().is_ident("shader_type") {
                let a = a
                    .meta
                    .require_list()
                    .expect("Requires a data type specified");
                let ty = a.tokens.clone();
                // eprintln!("{:#?}", ty);
                Some(ty)
            } else {
                None
            }
        })
        .expect("Fields must contain a shader type")
}

fn expand_named_fields(field: Field) -> impl ToTokens {
    let ident = field.ident.clone().unwrap();
    let ty = expand_unamed_fields(field);
    quote! {#ident: #ty}
}

fn expand_unamed_fields(field: Field) -> impl ToTokens {
    let texture = texture_details(&field);
    let ty = if let Some((format, dim)) = texture {
        quote! {bevy_shazzy::ImageBuilder<bevy_shazzy::texture_details::#format, bevy_shazzy::texture_details::#dim>}
    } else {
        get_type_from_shader_type(&field)
    };
    quote! {#ty}
}

fn struct_tokens(data: DataStruct, ident: &Ident) -> impl ToTokens {
    match data.fields {
        syn::Fields::Named(fields_named) => {
            let a = fields_named.named.into_iter().map(expand_named_fields);

            quote! { struct #ident {
                #(#a),*
            }}
        }
        syn::Fields::Unnamed(fields_unnamed) => {
            let a = fields_unnamed.unnamed.into_iter().map(expand_unamed_fields);

            quote! { struct #ident(#(#a),*);}
        }
        syn::Fields::Unit => quote! { struct #ident; },
    }
}

fn expand_create_buffer(field: Field, ident: &Member) -> impl ToTokens {
    let buffers = quote! { bevy_shazzy::internals::buffers };
    let writeable = field
        .attrs
        .iter()
        .any(|a| a.meta.path().is_ident("writeable") || a.meta.path().is_ident("write_only"));
    let texture = texture_details(&field).is_some();
    if texture {
        quote! {#buffers::create_texture_buffer(images, self.#ident, #writeable)}
    } else {
        quote! {#buffers::create_storage_buffer(buffers, self.#ident, #writeable)}
    }
}

fn expand_named_fields_resource(field: Field) -> impl ToTokens {
    let ident = Member::Named(field.ident.clone().unwrap());
    let data = expand_create_buffer(field, &ident);

    quote! {#ident: #data.into()}
}

fn expand_unnamed_fields_resource(field: Field, count: usize) -> impl ToTokens {
    let ident = Member::Unnamed(count.into());
    let data = expand_create_buffer(field, &ident);

    quote! {#data.into()}
}

fn to_resources_tokens(data: DataStruct, buffer: &Ident) -> impl ToTokens {
    match data.fields {
        syn::Fields::Named(fields_named) => {
            let a = fields_named
                .named
                .into_iter()
                .map(expand_named_fields_resource);

            quote! {
                #buffer {
                    #(#a),*
                }
            }
        }
        syn::Fields::Unnamed(fields_unnamed) => {
            let a = fields_unnamed
                .unnamed
                .into_iter()
                .enumerate()
                .map(|(count, f)| expand_unnamed_fields_resource(f, count));

            quote! { #buffer(#(#a,),*)}
        }
        syn::Fields::Unit => quote! { #buffer; },
    }
}

fn to_init_params(field: Field, count: usize) -> impl ToTokens {
    let ident = field.ident.clone().unwrap_or(format_ident!("_{}", count));
    let ty = expand_unamed_fields(field);
    quote! {#ident: #ty}
}

fn to_init_return(field: Field, count: usize) -> impl ToTokens {
    let ident = if let Some(field) = &field.ident {
        Member::Named(field.clone())
    } else {
        Member::Unnamed(count.into())
    };

    let param = field.ident.unwrap_or(format_ident!("_{}", count));

    quote! {#ident: #param}
}

fn create_buffer_init(ident: &Ident, data: DataStruct) -> impl ToTokens {
    let (params, ret): (Vec<_>, Vec<_>) = data
        .clone()
        .fields
        .into_iter()
        .enumerate()
        .map(|(n, f)| (to_init_params(f.clone(), n), to_init_return(f, n)))
        .unzip();

    quote! {
        fn init(#(#params),*) -> #ident {
            #ident {
                #(#ret),*
            }
        }
    }
}

fn create_initializer(ident: Ident, buffer: Ident, data: Data) -> impl ToTokens {
    //eprintln!("{:#?}", data);
    let (sct, rsc, init) = match data {
        Data::Struct(data_struct) => (
            struct_tokens(data_struct.clone(), &ident),
            to_resources_tokens(data_struct.clone(), &buffer),
            create_buffer_init(&ident, data_struct.clone()),
        ),
        _ => unimplemented!("Cannot expand non-struct into buffer group"),
    };

    let render = quote! { bevy_shazzy::bevy::render };
    let assets = quote! { bevy_shazzy::bevy::Assets };
    let image = quote! { bevy_shazzy::bevy::Image };
    quote! {
        #[derive(Clone, Default)]
        pub #sct

        impl #ident {
            fn to_resources(
                self,
                buffers: &mut #assets<#render::storage::ShaderStorageBuffer>,
                images: &mut #assets<#image>,
            ) -> #buffer {
                #rsc
            }
        }

        impl #buffer {
            pub #init
        }
    }
}

// impl HelloBuffers {
//     pub fn init(a: Vec<u32>, b: Foo, c: Vec3, d: ImageBuilder<R32Float, D2>) -> HelloInitializer {
//         HelloInitializer { a, b, c, d }
//     }
// }
