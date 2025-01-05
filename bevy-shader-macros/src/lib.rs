use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(ShaderEntry)]
pub fn entry_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_entry_macro(&ast)
}

fn impl_entry_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let impl_tokens = quote! {
        impl ShaderEntry for #name {
            fn as_key(&self) -> usize {
            //     match self {
            //         #name::Main => 0,
            //         #name::Update => 1,
            //     }
            // }
                todo!();
            }
        }
    };

    impl_tokens.into()
}
