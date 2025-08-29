// use proc_macro::TokenStream;
// use proc_macro2::Span;
// use quote::quote;
// use syn::{Data, DeriveInput};
// use syn::spanned::Spanned;
// use crate::util::MaybeFoldError;
//
// const CRATE_PATH: &str = "configurator";
//
// pub fn derive_config_inner(input: TokenStream) -> TokenStream {
// 	let input = syn::parse_macro_input!(input as DeriveInput);
// 	let attrs = match parse_input_attr_list(&input.attrs) {
// 		Ok(attrs) => attrs,
// 		Err(err) => return err.to_compile_error().into(),
// 	};
//
// 	let name = &input.ident;
// 	match &input.data {
// 		Data::Struct(st) => {
//             quote! {
//                 impl #name {
// 					pub fn
//                 }
//             }.into()
//         },
// 		_ => syn::Error::new_spanned(input, "Config macro allowed only for structs")
// 			.into_compile_error()
// 			.into(),
// 	}
// }
//
// fn parse_input_attr_list(attrs: &[syn::Attribute]) -> syn::Result<Vec<(Span, Attr)>> {
// 	let mut error = None;
// 	let mut out = Vec::new();
//
// 	for attr in attrs.iter() {
// 		if attr.path().is_ident("conf") {
// 			match parse_input_attr(attr) {
// 				Ok(v) => out.push((attr.span(), v)),
// 				Err(e) => error.maybe_fold(e),
// 			}
// 		}
// 	}
//
// 	if let Some(error) = error {
// 		return Err(error);
// 	}
//
// 	Ok(out)
// }
//
// fn parse_input_attr(attr: &syn::Attribute) -> syn::Result<Attr> {
// 	let meta_list = match attr.meta.require_list() {
// 		Ok(v) => v,
// 		Err(_) => {
// 			return Err(syn::Error::new(
// 				attr.meta.span(),
// 				"invalid attr style, expected parenthesized arguments",
// 			))
// 		}
// 	};
//
// 	syn::parse2::<Attr>(meta_list.tokens.clone())
// }
//
// struct Attrs {
//
// }
//
// enum Attr {
//
// }