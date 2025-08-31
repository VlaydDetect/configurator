use proc_macro::TokenStream;

mod validator;
mod config;
mod util;

#[proc_macro_derive(Validate, attributes(validate))]
pub fn derive_validate(input: TokenStream) -> TokenStream {
    validator::derive_validate_inner(input)
}

#[proc_macro]
pub fn select(input: TokenStream) -> TokenStream {
    validator::select_inner(input)
}

// #[proc_macro_derive(Config, attributes(conf))]
// pub fn derive_config(input: TokenStream) -> TokenStream {
//     config::derive_config_inner(input)
// }