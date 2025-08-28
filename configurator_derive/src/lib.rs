use proc_macro::TokenStream;

mod validator;
mod config;

#[proc_macro_derive(Validate, attributes(garde))]
pub fn derive_validate(input: TokenStream) -> TokenStream {
    validator::derive_validate_inner(input)
}

#[proc_macro]
pub fn select(input: TokenStream) -> TokenStream {
    validator::select_inner(input)
}