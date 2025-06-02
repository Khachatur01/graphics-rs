use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

fn impl_as_any_macro(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl entity_model_feature::AsAny for #name {
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
    let generics = &ast.generics;

    quote! {
        impl #generics entity_model_feature::Feature for #name #generics {}
    }.into()
}

fn impl_model_macro(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generics = &ast.generics;

    quote! {
        impl #generics entity_model_feature::Model for #name #generics {}
    }.into()
}

fn impl_as_serialize_macro(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generics = &ast.generics;

    quote! {
        impl #generics entity_model_feature::AsSerialize for #name #generics {
            fn as_serialize(&self) -> &dyn dyn_serde::Serialize {
                self
            }
        }
    }.into()
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
    output.extend(impl_as_serialize_macro(&ast));
    output
}

#[proc_macro_derive(AsSerialize)]
pub fn as_serialize_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    impl_as_serialize_macro(&ast)
}
