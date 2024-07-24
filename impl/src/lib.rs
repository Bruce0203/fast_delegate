use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, parse_quote,
    punctuated::Punctuated,
    spanned::Spanned,
    token::Comma,
    Field, FnArg, GenericParam, Ident, ItemStruct, ItemTrait, Meta, Token, TraitItem, TraitItemFn,
    Type,
};

#[proc_macro_attribute]
pub fn delegate(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let item_trait = parse_macro_input!(input as ItemTrait);

    let delegation_functions: Vec<_> = item_trait
        .items
        .iter()
        .filter_map(|trait_item| match trait_item {
            syn::TraitItem::Fn(f) => {
                Some(generate_delegation_function(f).unwrap_or_else(|| trait_item.clone()))
            }
            syn::TraitItem::Type(_) => None,
            item => Some(item.clone()),
        })
        .collect();

    let mut where_clause = item_trait
        .generics
        .where_clause
        .clone()
        .map(|v| v.predicates)
        .unwrap_or_else(|| parse_quote!());

    let generic_params_in_impl = &item_trait.generics.params;
    for param in generic_params_in_impl.iter() {
        if let GenericParam::Type(ty) = param {
            let ident = &ty.ident;
            where_clause.push(parse_quote!(#ident: '__delegate_lifetime));
        }
    }

    let associated_types_in_generic: Vec<_> = item_trait
        .items
        .iter()
        .filter_map(|item| match item {
            TraitItem::Type(ty) => {
                if ty.default.is_some() {
                    None
                } else {
                    let ty_ident = &ty.ident;
                    Some(quote! {#ty_ident = (), })
                }
            }
            _ => None,
        })
        .collect();

    let associated_types: Vec<_> = item_trait
        .items
        .iter()
        .filter_map(|item| match item {
            TraitItem::Type(ty) => {
                if ty.default.is_some() {
                    None
                } else {
                    let ty_ident = &ty.ident;
                    Some(quote! {type #ty_ident = ();})
                }
            }
            _ => None,
        })
        .collect();

    let mut generic_params = item_trait.generics.params.clone();
    detach_bounds_from_generic(&mut generic_params);
    //convert <const N: usize> to <N>
    let generic_params: Vec<_> = generic_params
        .iter()
        .map(|param| match param {
            GenericParam::Const(value) => value.ident.to_token_stream(),
            other => other.to_token_stream(),
        })
        .collect();

    let name = &item_trait.ident;
    let delegate_generic: Ident = parse_quote!(__DelegateImpl);
    let mut delegate_generic_bound: Punctuated<GenericParam, Comma> = parse_quote! {
        #delegate_generic: fast_delegate::Delegatable<'__delegate_lifetime, &'__delegate_lifetime dyn #name<#(#associated_types_in_generic)*#(#generic_params)*>>,
    };
    let delegate_generic_param = match delegate_generic_bound.first_mut().unwrap() {
        GenericParam::Type(value) => value,
        _ => unreachable!(),
    };

    for bound in item_trait.supertraits.iter() {
        delegate_generic_param.bounds.push(bound.clone());
    }
    quote! {
        #item_trait

        impl<'__delegate_lifetime: 'static, #delegate_generic, #generic_params_in_impl> #name<#(#generic_params)*> for #delegate_generic
        where
            #delegate_generic_bound
            #delegate_generic::Target: #name<#(#generic_params)*>,
            #where_clause
        {
            #(#associated_types)*
            #(#delegation_functions)*
        }
    }
    .into()
}

fn generate_delegation_function(f: &TraitItemFn) -> Option<TraitItem> {
    let sig = &f.sig;
    let name = &sig.ident;
    let first_argument = sig.inputs.first()?;
    let delegate_fn_ident = delegate_fn_ident(first_argument)?;
    let inputs_variables: Vec<_> = sig
        .inputs
        .iter()
        .filter_map(|input| match input {
            FnArg::Typed(typed) => Some(Ident::new(
                typed.pat.to_token_stream().to_string().as_str(),
                typed.pat.span(),
            )),
            _ => None,
        })
        .collect();
    Some(TraitItem::Fn(parse_quote! {
        #[inline(always)]
        #sig {
            self.#delegate_fn_ident().#name(#(#inputs_variables),*)
        }
    }))
}

fn delegate_fn_ident(first_argument: &FnArg) -> Option<Ident> {
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
    let impls: Vec<_> = item_struct
        .fields
        .iter()
        .filter_map(|field| generate_delegatable_for_field(&item_struct, field))
        .collect();
    quote!(#(#(#impls)*)*).into()
}

fn generate_delegatable_for_field(
    item_struct: &ItemStruct,
    field: &Field,
) -> Option<Vec<proc_macro2::TokenStream>> {
    field.attrs.iter().find_map(|attr| {
        let list = match &attr.meta {
            Meta::List(list) => list,
            _ => return None,
        };
        if !list.path.is_ident("to") {
            return None;
        }
        let struct_name = &item_struct.ident;
        let struct_generic_params_in_impl = &item_struct.generics.params;
        let mut struct_generic_params = struct_generic_params_in_impl.clone();
        detach_bounds_from_generic(&mut struct_generic_params);
        let struct_where_clause = &item_struct.generics.where_clause;
        let trait_names = attr.parse_args::<CommaSeparatedTypes>().unwrap().types;
        let field_name = &field.ident;
        let field_type = &field.ty;
        let impls: Vec<_> = trait_names
            .iter()
            .map(|trait_type| {
                quote! {
                    impl<'__delegate_lifetime: 'static, #struct_generic_params_in_impl>
                        fast_delegate::Delegatable<'__delegate_lifetime, &'__delegate_lifetime dyn #trait_type>
                        for #struct_name<#struct_generic_params> #struct_where_clause {
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
            })
            .collect();
        Some(impls)
    })
}

struct CommaSeparatedTypes {
    types: Punctuated<Type, Token![,]>,
}

impl Parse for CommaSeparatedTypes {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        Ok(CommaSeparatedTypes {
            types: input.parse_terminated(Type::parse, Token![,])?,
        })
    }
}

fn detach_bounds_from_generic(params: &mut Punctuated<GenericParam, Comma>) {
    for param in params.iter_mut() {
        match param {
            GenericParam::Lifetime(value) => value.bounds.clear(),
            GenericParam::Type(value) => value.bounds.clear(),
            _ => {}
        }
    }
}
