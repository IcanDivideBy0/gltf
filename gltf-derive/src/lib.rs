// Adapted from `validator_derive` (https://github.com/Keats/validator).
//
// See LICENSE for details.

#![recursion_limit = "128"]

#[macro_use]
extern crate quote;
extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro_derive(Validate)]
pub fn derive_validate(input: TokenStream) -> TokenStream {
    let source = input.to_string();
    let ast = syn::parse_macro_input(&source).unwrap();
    let tokens = expand(&ast);
    tokens.parse().unwrap()
}

fn expand(ast: &syn::MacroInput) -> quote::Tokens {
    let fields = match ast.body {
        syn::Body::Struct(syn::VariantData::Struct(ref fields)) => fields,
        _ => panic!("#[derive(Validate)] only works on `struct`s"),
    };
    let ident = &ast.ident;
    let minimal_validations: Vec<quote::Tokens> = fields.iter()
        .map(|f| f.ident.as_ref().unwrap())
        .map(|ident| {
            use inflections::Inflect;
            let field = ident.as_ref().to_camel_case();
            quote!(
                self.#ident.validate_minimally(
                    _root,
                    || _path().field(#field),
                    _report,
                )
            )
        })
        .collect();
    let complete_validations: Vec<quote::Tokens> = fields.iter()
        .map(|f| f.ident.as_ref().unwrap())
        .map(|ident| {
            use inflections::Inflect;
            let field = ident.as_ref().to_camel_case();
            quote!(
                self.#ident.validate_completely(
                    _root,
                    || _path().field(#field),
                    _report,
                )
            )
        })
        .collect();
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    quote!(
        impl #impl_generics crate::validation::Validate
            for #ident #ty_generics #where_clause
        {
            fn validate_minimally<P, R>(
                &self,
                _root: &crate::Root,
                _path: P,
                _report: &mut R
            ) where
                P: Fn() -> crate::Path,
                R: FnMut(&Fn() -> crate::Path, crate::validation::Error),
            {
                #(
                    #minimal_validations;
                )*
            }

            fn validate_completely<P, R>(
                &self,
                _root: &crate::Root,
                _path: P,
                _report: &mut R
            ) where
                P: Fn() -> crate::Path,
                R: FnMut(&Fn() -> crate::Path, crate::validation::Error),
            {
                #(
                    #complete_validations;
                )*
            }
        }
    )
}
