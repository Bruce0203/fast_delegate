///! This library is nightly-only as it relies on `associated_type_defaults`
///! 
///! # example of this crate
///! 
///! ```rust
///! #![feature(associated_type_defaults)]
///! 
///! use delegare::{delegate, Delegate};
///! 
///! #[delegate]
///! pub trait Delegate {
///!     fn run(&self);
///! }
///! 
///! #[delegate]
///! pub trait Delegate2 {
///!     fn run2(&self);
///! }
///! 
///! #[derive(Delegate)]
///! pub struct Delegated {
///!     #[to(Delegate)]
///!     entity: DelegateImpl,
///!     #[to(Delegate2)]
///!     entity2: Delegate2Impl,
///! }
///! 
///! pub struct DelegateImpl;
///! impl Delegate for DelegateImpl {
///!     fn run(&self) {
///!         println!("Delegate");
///!     }
///! }
///! 
///! pub struct Delegate2Impl;
///! impl Delegate2 for Delegate2Impl {
///!     fn run2(&self) {
///!         println!("Delegate2");
///!     }
///! }
///! 
///! #[test]
///! fn delegate_test() {
///!     let player = Delegated {
///!         entity: DelegateImpl {},
///!         entity2: Delegate2Impl {},
///!     };
///!     player.run();
///!     player.run2();
///! }
use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{
    parse_macro_input, parse_quote, spanned::Spanned, Attribute, Field, FnArg, Ident, ItemStruct, ItemTrait, Meta, TraitItem, TraitItemFn, TraitItemType
};

#[proc_macro_attribute]
pub fn delegate(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut item_trait = parse_macro_input!(input as ItemTrait);
    let delegation_functions: Vec<_> = item_trait
        .items
        .iter()
        .map(|trait_item| match trait_item {
            syn::TraitItem::Fn(f) => {
                generate_delegation_function(f).unwrap_or_else(|| trait_item.clone())
            }
            item => item.clone(),
        })
        .collect();

    let name = &item_trait.ident;
    let token: Ident = format_ident!("__InternalDelegareToken{}", name);
    let internal_type: TraitItemType = parse_quote! {
        type __Internal = #token;
    };

    item_trait.items.push(syn::TraitItem::Type(internal_type));

    quote! {
        #item_trait

        pub struct #token;
        impl<T> #name for T
        where
            T: delegare::Delegatable<#token>,
            T::Target: #name,
        {
            #(#delegation_functions)*
        }
    }
    .into()
}


fn generate_delegation_function(f: &TraitItemFn) -> Option<TraitItem>{
    let sig = &f.sig;
    let name = &sig.ident;
    let first_argument = sig.inputs.first()?;
    let delegate_fn_ident = delegate_fn_ident(first_argument)?;
    let inputs_variables: Vec<_> = sig.inputs.iter().filter_map(|input| {
            match input {
                FnArg::Typed(typed) => Some(Ident::new(typed.pat.to_token_stream().to_string().as_str(), typed.pat.span())),
                _ => None
            }
        }).collect();
    Some(TraitItem::Fn(parse_quote! {
        #[inline(always)]
        #sig {
            return self.#delegate_fn_ident().#name(#(#inputs_variables)*)
        }
    }))
}

fn delegate_fn_ident(first_argument: &FnArg)-> Option<Ident> {
    if let syn::FnArg::Receiver(reciever) = first_argument {
        match (reciever.reference.is_some(), reciever.mutability.is_some()) {
            (false, _) => parse_quote! {delegate_owned},
            (true, true) => parse_quote! {delegate_mut},
            (true, false) => parse_quote! {delegate_ref},
        }
    } else {
        None
    }
}


#[proc_macro_derive(Delegate, attributes(to))]
pub fn derive_delegate(input: TokenStream) -> TokenStream {
    let item_struct = parse_macro_input!(input as ItemStruct);
    let struct_name = &item_struct.ident;
    let impls: Vec<_> = item_struct
        .fields
        .iter()
        .filter_map(|field| {
            generate_delegatable_for_field(struct_name, field)
        })
        .collect();
    quote!(#(#(#impls)*)*).into()
}

fn generate_delegatable_for_field(struct_name: &Ident,field: &Field) -> Option<Vec< proc_macro2::TokenStream>>{
    field.attrs.iter().find_map(|attr|  {
        let list = match &attr.meta {
            Meta::List(list) => list,
            _ => return None
        };
        if !list.path.is_ident("to") {
            return None;
        }
        let trait_names = trait_names(attr)?;
        let field_name = &field.ident;
        let field_type = &field.ty;
        let impls: Vec<_> = trait_names.iter().map(|trait_name| {
            quote! {
                impl delegare::Delegatable<<#field_type as #trait_name>::__Internal> for #struct_name {
                    type Target = #field_type;
        
                    fn delegate_mut(&mut self) -> &mut Self::Target {
                        &mut self.#field_name
                    }
        
                    fn delegate_ref(&self) -> &Self::Target {
                        &self.#field_name
                    }
        
                    fn delegate_owned(self) -> Self::Target {
                        self.#field_name
                    }
                }
            }
        }).collect();
        Some(impls)
    })
}

fn trait_names(to_attr: &Attribute) -> Option<Vec<Ident>> {
    let mut tokens = proc_macro2::TokenStream::new();
    to_attr.to_tokens(&mut tokens);

    let mut trait_names: Vec<Ident> = Vec::new();
    to_attr
        .parse_nested_meta(|meta| {
            meta.path
                .segments
                .into_iter()
                .for_each(|s| trait_names.push(s.ident));
            Ok(())
        })
        .ok()?;
    if trait_names.is_empty() {
        None
    } else {
        Some(trait_names)
    }
}
