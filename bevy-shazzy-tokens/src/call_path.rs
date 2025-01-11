use proc_macro2::TokenStream;
use quote::quote;

pub(crate) struct Bevy;
impl quote::ToTokens for Bevy {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(quote! {bevy_shazzy::bevy});
    }
}
pub(crate) const BEVY: Bevy = Bevy;

pub(crate) struct Ecs;
impl quote::ToTokens for Ecs {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(quote! {#BEVY::ecs});
    }
}
pub(crate) const ECS: Ecs = Ecs;

pub(crate) struct Render;
impl quote::ToTokens for Render {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(quote! {#BEVY::render});
    }
}
pub(crate) const RENDER: Render = Render;

pub(crate) struct Buffers;
impl quote::ToTokens for Buffers {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(quote! {bevy_shazzy::internals::buffers});
    }
}
pub(crate) const BUFFERS: Buffers = Buffers;

pub(crate) struct SendSync;
impl quote::ToTokens for SendSync {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(quote! {std::marker::Send + std::marker::Sync + 'static});
    }
}

pub(crate) const SEND_SYNC: SendSync = SendSync;
