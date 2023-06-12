use proc_macro::TokenStream;
use quote::quote;
use syn::{parse, parse_macro_input, Expr, Token};

struct AddInput {
    left: Expr,
    right: Expr,
}

impl parse::Parse for AddInput {
    fn parse(input: parse::ParseStream) -> parse::Result<Self> {
        let left = input.parse()?;
        let _: Token![,] = input.parse()?;
        let right = input.parse()?;
        Ok(Self { left, right })
    }
}

#[proc_macro]
pub fn add(params: TokenStream) -> TokenStream {
    let input = parse_macro_input!(params as AddInput);
    let left = &input.left;
    let right = &input.right;

    quote! {
        #left + #right
    }
    .into()
}
