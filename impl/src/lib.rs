use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn delegate(attr: TokenStream, input: TokenStream) -> TokenStream {
    input
}

#[proc_macro_derive(Delegate, attributes(to))]
pub fn derive_delegate(input: TokenStream) -> TokenStream {
    quote! {}.into()
}
