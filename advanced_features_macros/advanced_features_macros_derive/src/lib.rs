use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Greet)]
pub fn advanced_features_macros_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_advanced_features_macros(&ast)
}

fn impl_advanced_features_macros(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Greet for #name {
            fn greet() {
                println!("Hello, my name is {}", stringify!(#name));
            }
        }
    };
    gen.into()
}
