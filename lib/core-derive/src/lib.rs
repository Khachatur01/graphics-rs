use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

fn impl_as_any_macro(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl core::AsAny for #name {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self
            }
        }
    };
    gen.into()
}

fn impl_feature_macro(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl core::Feature for #name {}
    };
    gen.into()
}

fn impl_model_macro(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl core::Model for #name {}
    };
    gen.into()
}

#[proc_macro_derive(Feature)]
pub fn feature_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    let mut output: TokenStream = impl_as_any_macro(&ast);
    output.extend(impl_feature_macro(&ast));
    output
}

#[proc_macro_derive(Model)]
pub fn model_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    let mut output: TokenStream = impl_as_any_macro(&ast);
    output.extend(impl_model_macro(&ast));
    output
}
