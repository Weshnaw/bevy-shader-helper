use proc_macro2::{Literal, TokenStream, TokenTree};
use quote::{ToTokens, quote};

use crate::call_path::{BEVY, BUFFERS, ECS, RENDER, SEND_SYNC};

/// BufferGroup
pub struct ShaderBuffer {
    /// Ident of the struct
    ident: syn::Ident,
    /// Entry functions into a shader
    entries: Vec<Entry>,
    /// Buffers that are used by the shader
    buffers: Vec<Buffer>,
    /// Flag to denote if this is a named struct or a unnamed struct
    named: bool,
}

impl ShaderBuffer {
    fn initializer_ident(&self) -> syn::Ident {
        quote::format_ident!("{}Initializer", self.ident)
    }

    fn impl_resource(&self) -> TokenStream {
        let ident = &self.ident;

        quote! {
            impl #ECS::system::Resource for #ident
            where
                Self: #SEND_SYNC,
            {}
        }
    }

    fn impl_get_bindings(&self) -> TokenStream {
        let buffer_count = self.buffers.len();
        let bind_entries = self.buffers.iter().map(Buffer::bind_entry);

        quote! {
            fn get_bindings<'a>(
                &'a self,
                buffers: &'a #RENDER::render_asset::RenderAssets<#RENDER::storage::GpuShaderStorageBuffer>,
                images: &'a #RENDER::render_asset::RenderAssets<#RENDER::texture::GpuImage>,
            ) -> #RENDER::render_resource::BindGroupEntries<'a, #buffer_count> {
                #RENDER::render_resource::BindGroupEntries::sequential((
                    #(#bind_entries),*
                ))
            }
        }
    }

    fn impl_insert_resources(&self) -> TokenStream {
        let initializer_ident = self.initializer_ident();

        quote! {
            fn insert_resources(
                commands: &mut #BEVY::Commands,
                buffers: &mut #BEVY::Assets<#RENDER::storage::ShaderStorageBuffer>,
                images: &mut #BEVY::Assets<#BEVY::Image>,
                init: #initializer_ident,
            ) {
                commands.insert_resource(init.to_resources(buffers, images));
            }
        }
    }

    fn impl_buffer_entries(&self) -> TokenStream {
        let buffer_count = self.buffers.len();
        let bind_group_layout = self.buffers.iter().map(Buffer::bind_layout);

        quote! {
            fn buffer_entries(
                stage: #RENDER::render_resource::ShaderStages
            ) -> #RENDER::render_resource::BindGroupLayoutEntries<#buffer_count> {
                #RENDER::render_resource::BindGroupLayoutEntries::sequential(
                    stage,
                    (
                        #(#bind_group_layout),*
                    ),
                )
            }
        }
    }

    fn impl_entries(&self) -> TokenStream {
        let entry_count = self.entries.len();
        let entries = self.entries.iter().map(Entry::create_entry);

        quote! {
            fn entries(
                pipeline_cache: &#RENDER::render_resource::PipelineCache,
                layout: #RENDER::render_resource::BindGroupLayout,
                shader: #BEVY::Handle<#RENDER::render_resource::Shader>,
            ) -> [#RENDER::render_resource::CachedComputePipelineId; #entry_count] {
                [
                    #(#entries),*
                ]
            }
        }
    }

    fn impl_buffer_group(&self) -> TokenStream {
        let buffer_count = self.buffers.len();
        let entry_count = self.entries.len();
        let ident = &self.ident;
        let initializer_ident = self.initializer_ident();

        let get_bindings = self.impl_get_bindings();
        let insert_resources = self.impl_insert_resources();
        let buffer_entries = self.impl_buffer_entries();
        let entries = self.impl_entries();

        quote! {
            impl BufferGroup<#buffer_count, #entry_count> for #ident {
                type Initializer = #initializer_ident;
                #get_bindings
                #insert_resources
                #buffer_entries
                #entries
            }
        }
    }

    fn impl_initializer_struct(&self) -> TokenStream {
        let ident = self.initializer_ident();
        let fields = self.buffers.iter().map(Buffer::shader_data_type);

        let derives = quote! {#[derive(Clone, Default)]};
        if self.named {
            let fields = fields.map(|f| {
                let ident = f.0;
                let ty = f.1;

                quote! {#ident: #ty}
            });

            quote! {
                #derives
                pub struct #ident {
                    #(#fields),*
                }
            }
        } else {
            let fields = fields.map(|f| f.1);
            quote! {
                #derives
                pub struct #ident(
                    #(#fields),*
                );
            }
        }
    }

    fn impl_initializer_to_resources(&self) -> TokenStream {
        let ident = self.initializer_ident();
        let buffer_ident = &self.ident;
        let fields = self.buffers.iter().map(Buffer::create_buffer);

        quote! {
            impl #ident {
                fn to_resources(
                    self,
                    buffers: &mut #BEVY::Assets<#RENDER::storage::ShaderStorageBuffer>,
                    images: &mut #BEVY::Assets<#BEVY::Image>,
                ) -> #buffer_ident {
                    #buffer_ident {
                        #(#fields),*
                    }
                }
            }
        }
    }

    fn impl_initializer_buffer_init(&self) -> TokenStream {
        let ident = self.initializer_ident();
        let buffer_ident = &self.ident;

        let params = self.buffers.iter().map(|b| {
            let (ident, ty) = b.shader_data_type();
            let param = match ident {
                syn::Member::Named(ident) => ident,
                syn::Member::Unnamed(index) => &quote::format_ident!("_{}", index),
            };

            quote! {#param: #ty}
        });

        let fields = self.buffers.iter().map(|b| {
            let ident = b.ident();
            let param = match ident {
                syn::Member::Named(ident) => ident,
                syn::Member::Unnamed(index) => &quote::format_ident!("_{}", index),
            };
            if self.named {
                quote! {#ident}
            } else {
                quote! {#ident: #param}
            }
        });

        quote! {
            impl #buffer_ident {
                pub fn init(#(#params),*) -> #ident {
                    #ident {
                        #(#fields),*
                    }
                }
            }
        }
    }

    fn impl_initializer(&self) -> TokenStream {
        let init_struct = self.impl_initializer_struct();
        let init_to_resources = self.impl_initializer_to_resources();
        let init_buffer_init = self.impl_initializer_buffer_init();

        quote! {
            #init_struct
            #init_to_resources
            #init_buffer_init
        }
    }
}

impl quote::ToTokens for ShaderBuffer {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let impl_resource = self.impl_resource();
        let impl_buffer_group = self.impl_buffer_group();
        let impl_initializer = self.impl_initializer();
        tokens.extend(quote! {
            #impl_resource
            #impl_buffer_group
            #impl_initializer
        });
    }
}

impl From<syn::DeriveInput> for ShaderBuffer {
    fn from(input: syn::DeriveInput) -> Self {
        let syn::DeriveInput {
            ident, data, attrs, ..
        } = input;

        let data = match data {
            syn::Data::Struct(data_struct) => data_struct,
            _ => unimplemented!("Cannot expand non-struct into buffer group"),
        };

        let entries = attrs
            .into_iter()
            .flat_map(|a| {
                if a.path().is_ident("entry") {
                    let meta = a.meta.require_list().ok()?;
                    let mut args = meta.tokens.clone().into_iter().flat_map(|t| match t {
                        TokenTree::Literal(literal) => Some(literal),
                        _ => None,
                    });

                    let name = args.next()?;
                    let label = args.next();

                    Some(Entry { name, label })
                } else {
                    None
                }
            })
            .collect();

        let (named, buffers) = match data.fields {
            syn::Fields::Named(fields_named) => (true, fields_named.named),
            syn::Fields::Unnamed(fields_unnamed) => (false, fields_unnamed.unnamed),
            syn::Fields::Unit => (false, syn::punctuated::Punctuated::new()),
        };

        let buffers = buffers
            .iter()
            .enumerate()
            .map(|(count, f)| {
                let texture = f.attrs.iter().find_map(|a| {
                    if a.path().is_ident("texture") {
                        let meta = a.meta.require_list().ok()?;
                        let mut args = meta.tokens.clone().into_iter().flat_map(|t| match t {
                            TokenTree::Ident(ident) => Some(ident),
                            _ => None,
                        });
                        let format = args.next()?;
                        let dim = args.next()?;
                        Some((format, dim))
                    } else {
                        None
                    }
                });

                let write_only = f.attrs.iter().any(|a| a.path().is_ident("write_only"));
                let writeable = f.attrs.iter().any(|a| a.path().is_ident("writeable"));

                let access = if write_only {
                    BufferAccess::WriteOnly
                } else if writeable {
                    BufferAccess::ReadWrite
                } else {
                    BufferAccess::ReadOnly
                };

                let ident = f
                    .ident
                    .clone()
                    .map_or(syn::Member::Unnamed(count.into()), |i| {
                        syn::Member::Named(i)
                    });
                if let Some((format, dimensions)) = texture {
                    let texture = Texture {
                        ident,
                        dimensions,
                        format,
                        access,
                    };

                    Buffer::Texture(texture)
                } else {
                    let buffer_type = f
                        .attrs
                        .iter()
                        .find_map(|a| {
                            let meta = a.meta.require_list().ok()?;
                            if a.path().is_ident("shader_type") {
                                Some(meta.tokens.clone())
                            } else {
                                None
                            }
                        })
                        .expect("Shader type must reference a ShaderType");

                    let storage_data = StorageData {
                        ident,
                        buffer_type,
                        access,
                    };

                    Buffer::Storage(storage_data)
                }
            })
            .collect();

        Self {
            ident,
            entries,
            buffers,
            named,
        }
    }
}

/// Shader entry function information
pub struct Entry {
    name: Literal,
    label: Option<Literal>,
}

impl Entry {
    fn create_entry(&self) -> TokenStream {
        let name = &self.name;
        let label = self
            .label
            .clone()
            .map_or(quote! {None}, |l| quote! {Some(#l)});

        quote! {
            Self::create_entry(pipeline_cache,
                ::core::clone::Clone::clone(&layout),
                ::core::clone::Clone::clone(&shader),
                #name, #label)
        }
    }
}

/// Type of buffer to create
pub enum Buffer {
    /// Storage buffer
    Storage(StorageData),
    /// Uniform buffer
    Uniform(StorageData),
    /// Texture buffer
    Texture(Texture),
}

impl Buffer {
    fn bind_layout(&self) -> TokenStream {
        match self {
            Buffer::Storage(shader_data) => shader_data.storage_bind_layout(),
            Buffer::Uniform(shader_data) => shader_data.uniform_bind_layout(),
            Buffer::Texture(texture) => texture.bind_layout(),
        }
    }

    fn bind_entry(&self) -> TokenStream {
        let fields = match self {
            Buffer::Storage(shader_data) => shader_data.bind_entry_fields(),
            Buffer::Uniform(shader_data) => shader_data.bind_entry_fields(),
            Buffer::Texture(texture) => texture.bind_entry_fields(),
        };

        quote! {#BUFFERS::HandleIntoBinding::binding(#fields)}
    }

    fn create_buffer(&self) -> TokenStream {
        match self {
            Buffer::Storage(shader_data) => shader_data.create_buffer(),
            Buffer::Uniform(shader_data) => shader_data.create_buffer(),
            Buffer::Texture(texture) => texture.create_buffer(),
        }
    }

    fn shader_data_type(&self) -> (&syn::Member, TokenStream) {
        match self {
            Buffer::Storage(shader_data) => shader_data.shader_data_type(),
            Buffer::Uniform(shader_data) => shader_data.shader_data_type(),
            Buffer::Texture(texture) => texture.shader_data_type(),
        }
    }

    fn ident(&self) -> &syn::Member {
        match self {
            Buffer::Storage(shader_data) => &shader_data.ident,
            Buffer::Uniform(shader_data) => &shader_data.ident,
            Buffer::Texture(texture) => &texture.ident,
        }
    }
}

/// ShaderData type, type must impl ShaderType
pub struct StorageData {
    ident: syn::Member,
    /// type must impl ShaderType
    buffer_type: TokenStream,
    access: BufferAccess,
}

impl StorageData {
    fn storage_bind_layout(&self) -> TokenStream {
        let ty = &self.buffer_type;
        let buffer_binding = self.access.storage_buffer_binding();

        quote! {#buffer_binding::<#ty>(false)}
    }

    fn uniform_bind_layout(&self) -> TokenStream {
        todo!("uniform buffer layout not implemented")
    }

    fn bind_entry_fields(&self) -> TokenStream {
        let ident = &self.ident;

        quote! {&self.#ident, buffers}
    }

    fn create_buffer(&self) -> TokenStream {
        let ident = &self.ident;
        let writeable = &self.access.writeable();

        quote! {#ident: #BUFFERS::create_storage_buffer(buffers, self.#ident, #writeable).into()}
    }

    fn shader_data_type(&self) -> (&syn::Member, TokenStream) {
        (&self.ident, self.buffer_type.to_token_stream())
    }
}

/// Texture details
pub struct Texture {
    ident: syn::Member,
    /// TextureDimension
    dimensions: syn::Ident,
    /// TextureFormat
    format: syn::Ident,
    /// StorageTextureAccess
    access: BufferAccess,
}

impl Texture {
    fn bind_layout(&self) -> TokenStream {
        let dim = &self.dimensions;
        let format = &self.format;
        let access = self.access.texture_access();

        quote! {
            #RENDER::render_resource::IntoBindGroupLayoutEntryBuilder::into_bind_group_layout_entry_builder(
                #RENDER::render_resource::BindingType::StorageTexture {
                access: #access,
                format: #RENDER::render_resource::TextureFormat::#format,
                view_dimension: #RENDER::render_resource::TextureViewDimension::#dim,
            })
        }
    }
    fn bind_entry_fields(&self) -> TokenStream {
        let ident = &self.ident;

        quote! {&self.#ident, images}
    }

    fn create_buffer(&self) -> TokenStream {
        let ident = &self.ident;
        let writeable = &self.access.writeable();

        quote! {#ident: #BUFFERS::create_texture_buffer(images, self.#ident, #writeable).into()}
    }

    fn shader_data_type(&self) -> (&syn::Member, TokenStream) {
        let format = &self.format;
        let dim = &self.dimensions;

        let ty = quote! {
            bevy_shazzy::ImageBuilder<
                bevy_shazzy::texture_details::#format,
                bevy_shazzy::texture_details::#dim
            >
        };

        (&self.ident, ty)
    }
}

/// Access level of the buffers
pub enum BufferAccess {
    ReadOnly,
    WriteOnly,
    ReadWrite,
}

impl BufferAccess {
    fn texture_access(&self) -> TokenStream {
        let access = match self {
            BufferAccess::ReadOnly => quote! {ReadOnly},
            BufferAccess::WriteOnly => quote! {WriteOnly},
            BufferAccess::ReadWrite => quote! {ReadWrite},
        };

        quote! {#RENDER::render_resource::StorageTextureAccess::#access}
    }

    fn storage_buffer_binding(&self) -> TokenStream {
        let storage_buffer = match self.writeable() {
            true => quote! {storage_buffer},
            false => quote! {storage_buffer_read_only},
        };

        quote! {#RENDER::render_resource::binding_types::#storage_buffer}
    }

    fn writeable(&self) -> bool {
        !matches!(self, Self::ReadOnly)
    }
}

#[cfg(test)]
mod tests {}
