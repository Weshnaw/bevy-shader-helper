use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{DeriveInput, Variant};

pub fn expand(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);

    let name = ast.ident;

    let data = ast.data;

    let variants = match data {
        syn::Data::Enum(data_enum) => data_enum.variants.into_iter().enumerate().map(enumify), //.enumerate(),
        _ => unimplemented!("Shader Entry must be a enum"),
        // TODO: consider implementing for other types, to allow for more a custom use of the helper
    };

    let expanded = quote! {
        impl ShaderEntry for #name {
            fn as_key(&self) -> usize {
                match self {
                    #(Self::#variants),*
                }
            }
        }
    };

    TokenStream::from(expanded)
}

fn enumify(data: (usize, Variant)) -> impl ToTokens {
    let num = data.0;
    let variant = data.1;
    let expanded = quote! {
        #variant => #num
    };

    expanded
}
