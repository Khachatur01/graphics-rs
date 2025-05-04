use proc_macro::TokenStream;
use quote::quote;

fn impl_feature_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl core::Feature for #name {}
    };
    gen.into()
}

fn impl_model_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl core::Model for #name {}
    };
    gen.into()
}

#[proc_macro_derive(Feature)]
pub fn feature_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_feature_macro(&ast)
}

#[proc_macro_derive(Model)]
pub fn model_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_model_macro(&ast)
}
