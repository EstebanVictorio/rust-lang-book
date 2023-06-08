use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_attribute]
pub fn property(attr: TokenStream, item: TokenStream) -> TokenStream {
    match &attr.to_string()[..] {
        "Message" => impl_message(&syn::parse(item).unwrap()),
        "Number" => impl_number(&syn::parse(item).unwrap()),
        _ => panic!("Unknown attribute"),
    }
}

fn impl_number(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen: quote::__private::TokenStream = quote! {
        struct #name {
            number: i32
        }
    };
    gen.into()
}

fn impl_message(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        struct #name {
            message: String
        }
    };
    gen.into()
}
