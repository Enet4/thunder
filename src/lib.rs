//!
#![feature(proc_macro)]

extern crate proc_macro;

#[macro_use]
extern crate syn;

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use quote::ToTokens;
use std::collections::HashSet as Set;
use syn::fold::{self, Fold};
use syn::punctuated::Punctuated;
use syn::synom::Synom;
use syn::{Expr, Ident, Item, ImplItemMethod, ImplItem, ItemImpl, Pat, Stmt, ItemStatic};

///
#[proc_macro_attribute]
pub fn thunderclap(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut i: ItemImpl = match syn::parse(input.clone()) {
        Ok(input) => input,
        Err(e) => panic!("Error: '{}'", e),
    };
    
    let previous = quote!(#i);

    let mut previous = quote! {
        App::new("Whatever")
    };

    for item in &i.items {
        &ImplItem::Method(ref i) => {

            previous = quote! {
                #previous
                .subcommand(#i.sig.ident).args(i.args)
            }

        }
    }


    let tokens = quote! {
        #previous

        /// This block was generated by thunder v0.0.0
        impl MyApp {

            /// Starts the CLI parsing and calls whichever function handles the input
            fn start() {
                // let generated = quote! {
                //     #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
                //     const #dummy_const: () = {
                //         extern crate serde as _serde;
                //         #impl_block
                //     };
                // };

                use clap::App;
                let mut app = App::new("MyApp");

            }
        }
    };

    // for item in &i.items {
    //     match item {
    //         &ImplItem::Method(ref i) => {
    //             let name = quote!(#i.sig.ident);
    //             let foo = quote! {
                    
    //             }

    //             // println!("{:#?}", i.sig.ident)
    //         },
    //         _ => continue,
    //     }
    // }

    // Parse the list of variables the user wanted to print.
    // let mut args: ItemStatic = syn::parse(args).unwrap();

    // Hand the resulting function body back to the compiler.
    // quote!(args).into()
    tokens.into()
}

